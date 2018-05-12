//! `keys` provides access to keyboard input from the user.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/05/12

use std::io;
use std::iter::Iterator;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, SendError,};
use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::um::wincon::INPUT_RECORD;
use winapi::um::winnt::HANDLE;

mod key_event;

pub use self::key_event::*;

const NULL_KEYS: *mut Keys = 0 as *mut _;
static mut KEYS_PTR: AtomicPtr<Keys> = AtomicPtr::new(NULL_KEYS);

/// Attempts to get the standard input handle of the program.
/// 
/// # Errors
/// 
/// * There was an OS Error while getting the input handle.
fn get_input_handle() -> io::Result<HANDLE> {
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_INPUT_HANDLE;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    
    match unsafe { GetStdHandle(STD_INPUT_HANDLE) } {
        //There was an issue getting the input handle.
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

/// A `Keys` is an iterator over `KeyEvents` from the keyboard.
/// 
/// Only one instance of this struct can exist at a time; once an instance is dropped a
/// new one can be created using `Keys::instance`.
pub struct Keys {
    console_handle: usize,
    msg: [INPUT_RECORD; 1],
    msg_read: u32,
}

impl Keys {
    fn new(console_handle: HANDLE) -> Self {
        unsafe { Self {
            console_handle: console_handle as usize,
            msg: ::std::mem::uninitialized(),
            msg_read: ::std::mem::uninitialized(),
        }}
    }
    /// Creates a `Keys` instance for this program.
    /// 
    /// Only one `Keys` instance can exist at a time, so additional calls to this function
    /// while an instance exists will return an error.
    /// 
    /// # Errors
    /// 
    /// * There was an OS Error while getting the input handle.
    /// * There is already an instance of `Keys`.
    pub fn instance() -> io::Result<Box<Self>> {
        unsafe {
            if KEYS_PTR.compare_and_swap(NULL_KEYS,
                Box::into_raw(Box::new(
                    Keys::new(get_input_handle()?))),
                    Ordering::Relaxed
                ) == NULL_KEYS {
                Ok(Box::from_raw(KEYS_PTR.load(Ordering::Relaxed)))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "A `Keys` instance already exists."
                ))
            }
        }
    }
    /// Attempts to start a new thread which simply waits for keyboard events.
    /// 
    /// Spawns a new thread which simply outputs all the `KeyEvents` to the returned `Receiver`.  
    /// The thread will be named "Keys".
    /// 
    /// # Errors
    /// 
    /// * `Keys::instance` failed to return a `Keys` instance.
    /// * Failed to spawn a new thread.
    pub fn start<T: 'static + From<KeyEvent> + Send>() -> io::Result<(JoinHandle<Result<(), SendError<T>>>, Receiver<T>)> {
        use std::thread::Builder;
        use std::sync::mpsc::channel;

        //The `Keys` instance to get key events from.
        let keys = Keys::instance()?;
        //The channel which used to output the `KeyEvents`.
        let (output, recv) = channel();
        //A handle to the new thread.
        let handle = Builder::new()
            //The thread will be named.
            .name("Keys".into())
            .spawn(move || {
                //Loop for each `KeyEvent`.
                for event in keys {
                    match event {
                        //The `KeyEvent` was fine, convert and send the event.
                        Ok(event) => if let Err(e) = output.send(KeyEvent::from(event).into()) {
                            //There was an error sending the event, the thread exits.
                            eprintln!("Key Error: {}", e); return Err(e)
                        },
                        //There was an error with this event, display it.
                        Err(e) => eprintln!("Key Error: {}", e),
                    }
                }

                #[cfg(debug_assertions)] eprintln!("Keys Thread Exiting.");

                Ok(())
            })?;

        Ok((handle, recv))
    }
}

impl Iterator for Keys {
    type Item = io::Result<KeyEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        use winapi::um::consoleapi::ReadConsoleInputW;
        use winapi::um::wincon::KEY_EVENT;

        //Gets key events forever.
        loop { unsafe {
            //Wait for a message from the console.
            if ReadConsoleInputW(
                    self.console_handle as HANDLE, self.msg.as_mut_ptr(),
                    self.msg.len() as u32, &mut self.msg_read
                ) == 0 {
                //There was an error reading a message from the console.
                return Some(Err(io::Error::last_os_error()))
            } else {
                //The message which was read from the console.
                let msg = &self.msg[0];
                
                //Check that the message is a key event.
                if msg.EventType == KEY_EVENT {
                    //Convert the message into a `KeyEvent`.
                    let msg = (*msg.Event.KeyEvent()).into();

                    //Output the event if debug is enabled.
                    #[cfg(feature = "debug")]
                    eprintln!("Key Event: {}, code={}",
                        if msg.bKeyDown != 0 { "WM_KEYDOWN" }
                        else { "WM_KEYUP" },
                        msg.wVirtualKeyCode
                    );

                    return Some(Ok(msg))
                }
            }
        }}
    }
}

impl Drop for Keys {
    fn drop(&mut self) {
        //Only one instance of `Keys` should exist, 
        unsafe { KEYS_PTR.compare_and_swap(self, NULL_KEYS, Ordering::Relaxed); }
    }
}

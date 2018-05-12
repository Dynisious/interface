
use std::io;
use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::um::wincon::INPUT_RECORD;
pub use winapi::um::wincon::KEY_EVENT_RECORD;
use ::{get_input_handle, HANDLE};

pub fn get_keys() -> io::Result<&'static mut Keys> {
    static mut KEYS_PTR: AtomicPtr<Keys> = AtomicPtr::new(0 as *mut _);

    unsafe {
        if KEYS_PTR.compare_and_swap(0 as *mut _,
            Box::into_raw(Box::new(Keys::new(get_input_handle()?))),
            Ordering::Relaxed) == 0 as *mut _
        { Ok(&mut **KEYS_PTR.get_mut()) }
        else { Err(io::ErrorKind::Other.into()) }
    }
}

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
}

impl ::std::iter::Iterator for &'static mut Keys {
    type Item = io::Result<&'static KEY_EVENT_RECORD>;

    fn next(&mut self) -> Option<Self::Item> {
        use winapi::um::consoleapi::ReadConsoleInputW;
        use winapi::um::wincon::KEY_EVENT;

        loop { unsafe {
            if ReadConsoleInputW(self.console_handle as HANDLE, self.msg.as_mut_ptr(),
                self.msg.len() as u32, &mut self.msg_read) == 0 {
                return Some(Err(io::Error::last_os_error()))
            } else {
                let msg = &*self.msg.as_ptr();
                
                if msg.EventType == KEY_EVENT {
                    let msg = msg.Event.KeyEvent();

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

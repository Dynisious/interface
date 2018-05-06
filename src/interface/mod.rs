//! `interface` covers all things related to IO with the player.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/05/06

use std::io;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use position::Pos;
use winapi::um::winnt::HANDLE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use world::WorldState;

mod keys;
mod display;

use self::keys::*;
use self::display::*;

/// Attempts to get the standard input handle.
/// 
/// # Errors
/// 
/// Err --- There was an OS Error while getting the input handle.
fn get_input_handle() -> io::Result<HANDLE> {
    use winapi::um::winbase::STD_INPUT_HANDLE;
    
    match unsafe { GetStdHandle(STD_INPUT_HANDLE) } {
        //There was an issue getting the input handle.
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

fn get_output_handle() -> io::Result<HANDLE> {
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    
    match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

pub fn start(world_update: Receiver<WorldState>) -> io::Result<(JoinHandle<io::Result<()>>, JoinHandle<()>)> {
    use std::sync::mpsc::channel;
    use std::thread::spawn;

    let output_handle = get_output_handle()
        .map_err(|e| { eprintln!("Output Error: {}", e); e })? as usize;
    let (mut interface_send, interface_recv) = channel();
    let display_thread = spawn(move || {
        let mut cursor = Pos::new(0, 0);
        let mut state = Some(Vec::with_capacity(0));

        loop {
            if let Some(state) = state.take() {
                display(
                    output_handle,
                    state.iter().map(|(a, b)| (*a - cursor, *b))
                ).map_err(|e| { eprintln!("Display Error: {}", e); e })?;
            }

            if let Ok(new) = world_update.try_recv() { state = Some(new) }
            if let Ok(key) = interface_recv.try_recv() {
                match key {
                    KeyMessage::Exit => return Ok(()),
                    KeyMessage::MoveCam(mv) => cursor += mv,
                }
            }
        }
    });
    let keys = get_keys().expect("Failed to get Keys.");
    let interface_thread = spawn(move || for event in keys {
        match event {
            Ok(event) => if event.bKeyDown != 0 {
                if !handle(&mut interface_send, event.wVirtualKeyCode as i16) { return }
            },
            Err(e) => eprintln!("Key Error: {}", e),
        }
    });

    Ok((display_thread, interface_thread))
}

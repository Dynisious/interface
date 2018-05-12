//! `interface` covers all things related to IO with the player.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/05/06

extern crate winapi;
extern crate position;

use std::io;
use winapi::um::winnt::HANDLE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

mod keys;
//mod display;

pub use self::keys::*;

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


use std::io;
use position::Pos;
use winapi::um::winnt::HANDLE;
use winapi::um::wincon::COORD;
use winapi::um::processenv::GetStdHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

mod keys;
mod display;

use self::keys::*;
use self::display::*;

fn get_input_handle() -> io::Result<HANDLE> {
    use winapi::um::winbase::STD_INPUT_HANDLE;
    
    match unsafe { GetStdHandle(STD_INPUT_HANDLE) } {
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

pub fn get_output_handle() -> io::Result<HANDLE> {
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    
    match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

pub fn start(output_handle: HANDLE) -> io::Result<()> {
    let state = vec![(Pos::new(10, 10), 'b')];
    display(
        output_handle,
        state.iter().map(|(a, b)| (a, *b))
    )?;
    for event in get_keys().expect("Failed to get Keys.") {
        match event {
            Ok(event) => if event.bKeyDown != 0 {
                handle(event.wVirtualKeyCode);
                if let Err(e) = display(
                    output_handle,
                    state.iter().map(|(a, b)| (a, *b))
                ) { eprintln!("Display Error: {}", e) }
            },
            Err(e) => eprintln!("Key Error: {}", e),
        }
    }

    Ok(())
}


use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO,};
use super::{HANDLE, COORD, io, Pos};

fn get_csbi(output_handle: HANDLE, mut csbi: CONSOLE_SCREEN_BUFFER_INFO) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO> {
    use winapi::um::wincon::GetConsoleScreenBufferInfo;
    unsafe {
        if 0 == GetConsoleScreenBufferInfo(output_handle, &mut csbi)
        { Err(io::Error::last_os_error()) }
        else { Ok(csbi) }
    }.map_err(|e| { eprintln!("CSBI Error: {}", e); e })
}

pub fn set_cursor(output_handle: HANDLE, cursor: COORD) -> io::Result<()> {
    use winapi::um::wincon::SetConsoleCursorPosition;
    unsafe { 
        if 0 == SetConsoleCursorPosition(output_handle, cursor)
        {  Err(io::Error::last_os_error()) }
        else { Ok(()) }
    }.map_err(|e| { eprintln!("Cursor Error: {}", e); e })
}

fn pos_to_coord(pos: &Pos) -> COORD {
    COORD { X: pos.x as i16, Y: pos.y as i16, }
}

pub fn display<'a, Iter>(output_handle: HANDLE, items: Iter) -> io::Result<()>
    where Iter: IntoIterator<Item = (&'a Pos, char)> {
    use std::io::Write;

    clear(output_handle)
    .and_then(|_| {
        const WIDTH: i16 = 83;
        const HEIGHT: i16 = 23;

        let out = io::stdout();
        let mut out = out.lock();
        let mut cursor = COORD { X: 1, Y: 0 };
        
        while cursor.X < (WIDTH - 1) {
            set_cursor(output_handle, cursor)?;
            print!("-");
            out.flush()?;
            
            cursor.X += 1;
        }
        
        while cursor.Y < HEIGHT {
            cursor.X = 0;
            set_cursor(output_handle, cursor)?;
            print!("|");
            out.flush()?;
            
            cursor.X = WIDTH - 1;
            set_cursor(output_handle, cursor)?;
            print!("|");
            out.flush()?;
            
            cursor.Y += 1;
        }
        
        cursor.X = 1;
        cursor.Y -= 1;
        while cursor.X < (WIDTH - 1) {
            set_cursor(output_handle, cursor)?;
            print!("-");
            out.flush()?;
            
            cursor.X += 1;
        }

        for (pos, c) in items {
            set_cursor(output_handle, pos_to_coord(pos))?;
            print!("{}", c);
            out.flush()?;
        }
        
        cursor = COORD { X: WIDTH / 2, Y: HEIGHT / 2 };
        set_cursor(output_handle, cursor)
    }).map_err(|e| { eprintln!("Display Error: {}", e); e })
}

pub fn clear(output_handle: HANDLE) -> io::Result<()> {
    use winapi::um::wincon::{FillConsoleOutputCharacterW, FillConsoleOutputAttribute};
    
    static CONSOLE_HOME: COORD = COORD { X: 0, Y: 0 };

    Ok(()).and_then(|_| {
        let mut csbi;
        let console_size;
        let mut chars_written = 0;

        unsafe {
            csbi = get_csbi(output_handle, ::std::mem::uninitialized())?;
            console_size = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32;
            
            if 0 == FillConsoleOutputCharacterW(
                    output_handle,
                    b' ' as u16,
                    console_size,
                    CONSOLE_HOME,
                    &mut chars_written) {
                return Err(io::Error::last_os_error())
            }
        }

        csbi = get_csbi(output_handle, csbi)?;

        unsafe {
            if 0 == FillConsoleOutputAttribute(
                    output_handle,
                    csbi.wAttributes,
                    console_size,
                    CONSOLE_HOME,
                    &mut chars_written) {
                return Err(io::Error::last_os_error())
            }
        }

        set_cursor(output_handle, CONSOLE_HOME)
    }).map_err(|e| { eprintln!("Clear Error: {}", e); e })
}


use std::io;
use std::ops::{Add, Mul,};
use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD,};
use winapi::um::winnt::HANDLE;

const WIDTH: i16 = 83;
const HEIGHT: i16 = 28;
static SIZE: COORD = coord(WIDTH, HEIGHT);
static CONSOLE_HOME: COORD = coord(0, 0);
static CENTRE: COORD = coord((WIDTH - 1) / 2, HEIGHT / 2);

/// A constructor for a `COORD`.
pub const fn coord(x: i16, y: i16) -> COORD { COORD { X: x, Y: y } }

/// A adds two `COORDs`.
pub const fn coord_add(lhs: COORD, rhs: COORD) -> COORD { coord(lhs.X + rhs.X, lhs.Y + rhs.Y) }

/// Attempts to get the standard output handle of the program.
/// 
/// # Errors
/// 
/// Err --- There was an OS Error while getting the output handle.
pub fn get_output_handle() -> io::Result<HANDLE> {
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    
    match unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
        handle => Ok(handle),
    }
}

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

const fn index(x: i16, y: i16) -> usize {
    x as usize + ((WIDTH + 1) as usize * y as usize)
}

fn write_display(display: &mut [u8], size: &COORD, cursor: &COORD, bytes: &[u8]) {
    #[cfg(debug_assertions)] {
    debug_assert_eq!((size.X + 1) * size.Y, display.len() as i16,
        "The size of `display` is not the size expected from `size`."
    )}
    let index = index(cursor.X, cursor.Y);
    
    for (index, &b) in (index..(index + bytes.len())).zip(bytes) {
        display[index] = b
    }
}

fn gen_display<Iter>(items: Iter) -> Vec<u8>
    where Iter: IntoIterator<Item = (COORD, u8)> {
    const DASH: u8 = b'-';
    static LINES: [u8; 1] = [b'|'; 1];
    static DASHES: [u8; WIDTH as usize - 2] =  [DASH; WIDTH as usize - 2];
    
    let mut cursor = coord(1, 0);
    let mut buffer = vec![b' '; (WIDTH as usize + 1) * HEIGHT as usize];

    for byte in buffer.iter_mut()
        .skip(WIDTH as usize)
        .step_by(WIDTH as usize + 1) { *byte = b'\n' }
    
    write_display(&mut buffer, &SIZE, &cursor, &DASHES);
    cursor.Y = HEIGHT - 1;
    write_display(&mut buffer, &SIZE, &cursor, &DASHES);
    
    cursor.Y = 0;
    while cursor.Y < HEIGHT {
        cursor.X = 0;
        write_display(&mut buffer, &SIZE, &cursor, &LINES);
        
        cursor.X = WIDTH - 1;
        write_display(&mut buffer, &SIZE, &cursor, &LINES);
        
        cursor.Y += 1;
    }

    for (pos, c) in items.into_iter()
        .map(|(pos, c)| (coord_add(pos, CENTRE), c))
        .filter(|(pos, _)| 1 < pos.X && pos.X < WIDTH
            && 0 < pos.Y && pos.Y < (HEIGHT - 1)) {
        write_display(&mut buffer, &SIZE, &pos, &[c.into()]);
    }
    
    buffer
}

pub fn display(output_handle: HANDLE, items: impl IntoIterator<Item = (COORD, u8)>) -> io::Result<()> {
    use winapi::um::fileapi::WriteFile;
    
    let buffer = gen_display(items);

    set_cursor(output_handle, CONSOLE_HOME)?;

    unsafe {
        if 0 == WriteFile(
            output_handle as HANDLE,
            buffer.as_ptr() as *const _,
            buffer.len() as _,
            0 as *mut _, 0 as *mut _
        ) { return Err(io::Error::last_os_error()) }
    }

    set_cursor(output_handle, CENTRE)?;

    Ok(())
}

pub fn clear(output_handle: HANDLE) -> io::Result<()> {
    use winapi::um::wincon::{FillConsoleOutputCharacterW, FillConsoleOutputAttribute};
    
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

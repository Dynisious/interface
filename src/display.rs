
use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD,};
use super::{HANDLE, io, Pos,};

const WIDTH: i32 = 83;
const HEIGHT: i32 = 28;
static CONSOLE_HOME: COORD = COORD { X: 0, Y: 0 };
static CENTRE: Pos = Pos::new((WIDTH - 1) / 2, HEIGHT / 2);

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

macro_rules! index {
    ($x:expr, $y:expr) => (index!(i32, $x, $y));
    ($tp:ty, $x:expr, $y:expr) => (((WIDTH + 1) as $tp * $y) + $x);
}

fn write_display(display: &mut [u8], size: &Pos, cursor: &Pos, bytes: &[u8]) {
    #[cfg(debug_assertions)] {
    debug_assert_eq!((size.x + 1) * size.y, display.len() as i32,
        "The size of `display` is not the size expected from `size`."
    )}
    let index = index!(usize, cursor.x as usize, cursor.y as usize);

    for (index, &b) in (index..(index + bytes.len())).zip(bytes) {
        display[index] = b
    }
}

fn gen_display<Iter>(items: Iter) -> Vec<u8>
    where Iter: IntoIterator<Item = (Pos, u8)> {
    static SIZE: Pos = Pos::new(WIDTH, HEIGHT);
    const DASH: u8 = b'-';
    static LINES: [u8; 1] = [b'|'; 1];
    static DASHES: [u8; WIDTH as usize - 2] =  [DASH; WIDTH as usize - 2];
    
    let mut cursor = Pos::new(1, 0);
    let mut buffer = vec![b' '; (WIDTH as usize + 1) * HEIGHT as usize];

    for byte in buffer.iter_mut().skip(WIDTH as usize).step_by(WIDTH as usize + 1) { *byte = b'\n' }
    
    write_display(&mut buffer, &SIZE, &cursor, &DASHES);
    cursor.y = HEIGHT - 1;
    write_display(&mut buffer, &SIZE, &cursor, &DASHES);
    
    cursor.y = 0;
    while cursor.y < HEIGHT {
        cursor.x = 0;
        write_display(&mut buffer, &SIZE, &cursor, &LINES);
        
        cursor.x = WIDTH - 1;
        write_display(&mut buffer, &SIZE, &cursor, &LINES);
        
        cursor.y += 1;
    }

    for (pos, c) in items.into_iter()
        .map(|(pos, c)| (pos + CENTRE, c))
        .filter(|(pos, _)| 1 < pos.x && pos.x < WIDTH
            && 0 < pos.y && pos.y < (HEIGHT - 1)) {
        write_display(&mut buffer, &SIZE, &pos, &[c.into()]);
    }
    
    cursor = Pos::default();
    unsafe {
        use super::FRMS;

        write_display(&mut buffer, &SIZE, &cursor, format!("Frames:{}", FRMS).as_bytes());
    }
    cursor = Pos::new(0, HEIGHT - 1);
    unsafe {
        use super::FPS;

        write_display(&mut buffer, &SIZE, &cursor, format!("FPS:{}", FPS).as_bytes());
    }
    buffer
}

pub fn display(output_handle: HANDLE, items: impl IntoIterator<Item = (Pos, u8)>) -> io::Result<()> {
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

    set_cursor(output_handle, COORD { X: CENTRE.x as i16, Y: CENTRE.y as i16 })?;

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

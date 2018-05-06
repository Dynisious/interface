
use std::sync::mpsc::Sender;
use position::Pos;
use winapi::shared::minwindef::HKL;

mod keys;

pub use self::keys::*;

fn get_keyboard_layout() -> HKL {
    use winapi::um::winuser::GetKeyboardLayout;

    unsafe { GetKeyboardLayout(0) }
}

pub fn handle(output: &mut Sender<KeyMessage>, key_code: i16) -> bool {
    use winapi::um::winuser::VkKeyScanExW;
    unsafe { if key_code == VkKeyScanExW(b'=' as u16, get_keyboard_layout()) {
        output.send(KeyMessage::Exit).ok(); return false
    } else if key_code == VkKeyScanExW(b'w' as u16, get_keyboard_layout()) {
        output.send(KeyMessage::MoveCam(Pos::new(0, -1))).ok();
    } else if key_code == VkKeyScanExW(b's' as u16, get_keyboard_layout()) {
        output.send(KeyMessage::MoveCam(Pos::new(0, 1))).ok();
    } else if key_code == VkKeyScanExW(b'd' as u16, get_keyboard_layout()) {
        output.send(KeyMessage::MoveCam(Pos::new(1, 0))).ok();
    } else if key_code == VkKeyScanExW(b'a' as u16, get_keyboard_layout()) {
        output.send(KeyMessage::MoveCam(Pos::new(-1, 0))).ok();
    }}

    true
}

pub enum KeyMessage {
    MoveCam(Pos),
    Exit,
}

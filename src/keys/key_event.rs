
use std::ops::{Deref, DerefMut,};
use winapi::um::wincon::KEY_EVENT_RECORD;

/// A `KeyEvent` provides some quality of life functions for winapi's `KEY_EVENT_RECORDs`.
pub struct KeyEvent(KEY_EVENT_RECORD);

impl From<KEY_EVENT_RECORD> for KeyEvent {
    fn from(from: KEY_EVENT_RECORD) -> Self { KeyEvent(from) }
}

impl Into<KEY_EVENT_RECORD> for KeyEvent {
    fn into(self) -> KEY_EVENT_RECORD { self.0 }
}

impl Deref for KeyEvent {
    type Target = KEY_EVENT_RECORD;

    fn deref(&self) -> &Self::Target { &self.0 } 
}

impl DerefMut for KeyEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

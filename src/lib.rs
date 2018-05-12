//! `interface` provides IO with the user.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/05/12

#![feature(iterator_step_by)]
#![feature(const_fn)]

extern crate winapi;

#[cfg(feature = "keys")] pub mod keys;
#[cfg(feature = "display")] pub mod display;

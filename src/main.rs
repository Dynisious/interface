
#![feature(use_extern_macros)]
#![feature(proc_macro_path_invoc)]
#![feature(const_fn)]
#![feature(try_from)]
#![feature(rustc_private)]

extern crate winapi;
extern crate position;

mod world;
mod interface;

fn main() {
    match interface::get_output_handle() {
        Err(e) => eprintln!("Output Error: {}", e),
        Ok(output_handle) => {
            let world = world::initialise();
            
            if let Err(e) = interface::start(output_handle) {
                eprintln!("Interface Error: {}", e)
            }
        },
    }
}

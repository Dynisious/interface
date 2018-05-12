
use std::io;
use std::sync::mpsc::{sync_channel, Receiver,};
use std::thread::{JoinHandle, spawn,};

mod keys;

pub use self::keys::*;

pub fn start(buffer: usize) -> io::Result<(JoinHandle<()>, Receiver<KEY_EVENT_RECORD>)> {
    let mut keys = get_keys()?;
    
    let (output, recv) = sync_channel(buffer);
    let handle = spawn(move || {
        loop {
            let event = if let Some(event) = keys.next() { event } else { return };

            match event {
                Ok(event) => if let Err(e) = output.send(event.clone()) {
                    eprintln!("Key Interface Exiting: {}", e); return
                },
                Err(e) => eprintln!("Key Error: {}", e),
            }
        }
    });

    Ok((handle, recv))
}


#![feature(mpsc_select)]
#![feature(iterator_step_by)]
#![feature(use_extern_macros)]
#![feature(proc_macro_path_invoc)]
#![feature(const_fn)]
#![feature(try_from)]
#![feature(rustc_private)]

extern crate winapi;
extern crate position;
extern crate timer;

mod world;
mod interface;

fn main() {
    use std::time::Duration;
    use world::{World, WorldPool,};

    const SECOND: u64 = 1000;
    const TICK: Duration = Duration::from_millis(SECOND / 5);
    
    let mut world_tick = timer::Timer::new(TICK);
    let world_sub = world_tick.subscribe();

    if let Some(world_sub) = world_sub {
        let (mut world, world_receiver) = World::new(WorldPool::default());
        world_sub.run(move |signal| match signal {
            timer::TimerSignal::TimerStopped => return,
            timer::TimerSignal::Tick => world.update(),
            _ => (),
        });

        match interface::start(world_receiver) {
            Err(e) => eprintln!("Interface Error: {}", e),
            Ok((display_thread, interface_thread)) => {
                world_tick = world_tick.start();
                interface_thread.join().ok();
                world_tick.stop();
                display_thread.join().ok();
            },
        };
    }
}

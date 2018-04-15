use std::sync::mpsc::Sender;
use std::thread;

use signal::trap::Trap;
use signal::Signal::SIGWINCH;

use models::event::Event;

pub struct SignalHandler;

impl SignalHandler {
    pub fn start(event_channel: Sender<Event>) {
        let trap = Trap::trap(&[SIGWINCH]);
        thread::spawn(move || {
            for _ in trap {
                event_channel.send(Event::WindowSizeChange).unwrap()
            }
        });
    }
}

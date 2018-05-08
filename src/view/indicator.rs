use view::terminal::Terminal;

use serenity::prelude::{Mutex, RwLock};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use models::event::Event;

use serenity::model::event::TypingStartEvent;
use serenity::model::id::UserId;
use serenity::model::user::User;
use termbuf::TermSize;

const BOTTOM_START: usize = 1;
const SIDE_PADDING: usize = 3;

pub struct Indicator {
    events: Arc<Mutex<Vec<TypingStartEvent>>>,
    user_cache: RefCell<HashMap<UserId, Arc<RwLock<User>>>>,
    event_channel: Sender<Event>,
}

impl Indicator {
    pub fn new(event_channel: Sender<Event>) -> Indicator {
        Indicator {
            events: Arc::new(Mutex::new(Vec::new())),
            user_cache: RefCell::new(HashMap::new()),
            event_channel,
        }
    }

    fn fetch_user_name(&self, user_id: UserId) -> Option<String> {
        let mut cache = self.user_cache.borrow_mut();
        let entry = cache.entry(user_id);

        use std::collections::hash_map::Entry::*;
        let user = match entry {
            Occupied(o) => Some(o.into_mut()),
            Vacant(v) => {
                if let Some(user) = user_id.find() {
                    Some(v.insert(user))
                } else {
                    None
                }
            }
        };
        user.map(|user| user.read().name.clone())
    }

    pub fn typing_start(&self, event: TypingStartEvent) {
        if self.fetch_user_name(event.user_id).is_none() {
            return;
        }

        for prev_event in self.events.lock().iter() {
            if prev_event.user_id == event.user_id {
                return;
            }
        }

        self.events.lock().push(event);
        let events = self.events.clone();
        let channel = self.event_channel.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(11));
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let since_the_epoch = since_the_epoch.as_secs();

            events
                .lock()
                .retain(|i| (since_the_epoch - i.timestamp) < 10);
            channel.send(Event::WindowSizeChange).unwrap();
        });
    }

    pub fn render(&self, screen: &mut Terminal, size: TermSize) {
        let text = self.events
            .lock()
            .iter()
            .filter_map(|e| self.fetch_user_name(e.user_id))
            .collect::<Vec<_>>()
            .join(", ");
        screen.buf.put_string(
            &text,
            size.width.saturating_sub(SIDE_PADDING + 1 + text.len()),
            size.height - BOTTOM_START,
        );
    }
}

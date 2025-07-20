/*!
A very super simple event system, only three functions: subscribe, unsubscribe and publish. (and a singleton, never mind.)
Supports async. (see `main-async` in examples)
```rust
let handle = event_system::EventSystem::singleton().subscribe(|x: &usize| println!("I am {}", x));
event_system::EventSystem::singleton().publish(&4usize);

event_system::EventSystem::singleton().subscribe(test);
event_system::EventSystem::singleton().publish(&6usize);

event_system::EventSystem::singleton().unsubscribe::<usize>(handle);
event_system::EventSystem::singleton().publish(&8usize);

event_system::EventSystem::singleton().subscribe(on_login);
event_system::EventSystem::singleton().publish(&Login);

fn test(i: &usize) {
    println!("I am F {}", i);
}

// with no params, you have to create a struct to distinguish between events
struct Login;

fn on_login(_: &Login) {
    println!("on login");
}
```
```text
Output:
I am 4
I am 6
I am F 6
I am F 8
on login
```
*/

use lazy_static::lazy_static;
use lockfree::map::Map;
use std::any::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

struct Event {
    id: usize,
    pub(crate) callback: Arc<dyn Fn(&(dyn Any + Send + Sync)) + Send + Sync + 'static>,
}

pub struct EventSystem {
    idx: AtomicUsize,
    events: Map<TypeId, Mutex<Vec<Event>>>,
}

impl EventSystem {
    pub fn singleton() -> &'static Self {
        lazy_static! {
            static ref SINGLETON: EventSystem = EventSystem {
                idx: AtomicUsize::new(0),
                events: Map::new()
            };
        };

        &*SINGLETON
    }

    /// returns the id of the callback for unsubscribing
    pub fn subscribe<T, F>(&self, callback: F) -> usize
    where
        T: 'static + Send + Sync,
        F: Fn(&T) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let list = if let Some(list) = self.events.get(&type_id) {
            list
        } else {
            self.events.insert(type_id, Mutex::new(Vec::new()));
            self.events.get(&type_id).unwrap()
        };

        let event = Event {
            id: self.idx.fetch_add(1, Ordering::SeqCst),
            callback: Arc::new(move |x: &(dyn Any + Send + Sync)| {
                if let Some(val) = x.downcast_ref::<T>() {
                    callback(val);
                } else {
                    println!(
                        "Wrong type for callback {:?}, expect {:?}",
                        type_name_of_val(x),
                        type_name::<T>()
                    );
                }
            }),
        };
        let id = event.id;
        list.val().lock().unwrap().push(event);

        id
    }

    pub fn unsubscribe<T>(&self, id: usize)
    where
        T: 'static + Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        if let Some(callbacks) = self.events.get(&type_id) {
            callbacks.val().lock().unwrap().retain(|x| x.id != id);
        }
    }

    pub fn publish<T: 'static + Send + Sync>(&self, arg: &T) {
        let type_id = TypeId::of::<T>();
        if let Some(callbacks) = self.events.get(&type_id) {
            for event in callbacks.val().lock().unwrap().iter() {
                (*event.callback)(arg);
            }
        } else {
            println!("No callbacks for event {:?}", type_id);
        }
    }
}

use event_system::EventSystem;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, spawn};

#[tokio::main]
async fn main() {
    let handle = EventSystem::singleton().subscribe(|x: &usize| println!("I am {}", x));

    let _ = join!(
        spawn(async {
            sleep(Duration::from_secs(2)).await;
            EventSystem::singleton().publish(&4usize);
            EventSystem::singleton().subscribe(test);
        }),
        spawn(async {
            sleep(Duration::from_secs(4)).await;
            EventSystem::singleton().publish(&6usize);
            EventSystem::singleton().subscribe(on_login);
        }),
        spawn(async {
            sleep(Duration::from_secs(6)).await;
            EventSystem::singleton().publish(&Login);
        }),
        spawn(async move {
            sleep(Duration::from_secs(8)).await;
            EventSystem::singleton().unsubscribe::<usize>(handle);
            EventSystem::singleton().publish(&8usize);
        }),
    );
}

fn test(i: &usize) {
    println!("I am F {}", i);
}

// with no params, you have to create a struct to distinguish between events
struct Login;

fn on_login(_: &Login) {
    println!("on login");
}

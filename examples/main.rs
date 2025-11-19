use k_event_system::EventSystem;

fn main() {
    let handle = EventSystem::singleton().subscribe(|x: &usize| println!("I am {}", x));
    EventSystem::singleton().publish(&4usize);

    EventSystem::singleton().subscribe(test);
    EventSystem::singleton().publish(&6usize);

    EventSystem::singleton().unsubscribe::<usize>(handle);
    EventSystem::singleton().publish(&8usize);

    EventSystem::singleton().subscribe(on_login);
    EventSystem::singleton().publish(&Login);
}

fn test(i: &usize) {
    println!("I am F {}", i);
}

// with no params, you have to create a struct to distinguish between events
struct Login;

fn on_login(_: &Login) {
    println!("on login");
}

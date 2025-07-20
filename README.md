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
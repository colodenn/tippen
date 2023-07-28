// ./src-tauri/src/keyboard_listener.rs
use rdev::{listen, Event, EventType};

pub fn run_listener<F>(emit: F)
where
    F: Fn(&str, &str) + 'static,
{
    println!("test");
    if let Err(error) = listen(move |event| callback(event, &emit)) {
        println!("Error: {:?}", error)
    }
}

fn callback<F: Fn(&str, &str)>(event: Event, emit: &F) {
    println!("My callback {:?}", event);

    match event.name {
        Some(string) => {
            println!("Some");
            
            println!("Some: {}", string);
            emit("Some", &string);
        }
        None => {
        }
    }
}
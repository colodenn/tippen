// ./src-tauri/src/keyboard_listener.rs
use crate::config;
use rdev::{listen, Event, EventType, Key};
use rodio::OutputStream;
use std::io::BufReader;

fn play_click_sound(file: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file_cache = std::fs::File::open(file).unwrap();
    let sound = stream_handle.play_once(BufReader::new(file_cache)).unwrap();
    sound.set_volume(0.5);
    std::thread::sleep(std::time::Duration::from_millis(150));
}

pub fn run_listener<F>(emit: F)
where
    F: Fn(&str, &str) + 'static,
{
    let mut released = true;
    if let Err(_error) = listen(move |event| callback(event, &emit, &mut released)) {}
}

fn callback<F: Fn(&str, &str)>(event: Event, emit: &F, released: &mut bool) {
    let mut file = "sounds/key2.wav";
    let _config = config::read_config();
    // let theme_config = config::read_config_theme(config.theme);
    match event.name {
        Some(string) => {
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::Space => {
                            file = "sounds/space.wav";
                            *released = false;
                        }
                        Key::Tab => {
                            file = "sounds/tab.wav";
                            *released = false;
                        }
                        Key::ShiftLeft => {
                            file = "sounds/shift.wav";
                            *released = false;
                        }
                        Key::ControlLeft => {
                            file = "sounds/ctrl.wav";
                            *released = false;
                        }
                        Key::Backspace => {
                            file = "sounds/back.wav";
                            *released = false;
                        }
                        Key::CapsLock => {
                            file = "sounds/caps.wav";
                            *released = false;
                        }
                        Key::Return => {
                            file = "sounds/ent.wav";
                            *released = false;
                        }
                        Key::KeyA => {
                            file = "sounds/key1.wav";
                            *released = false;
                        }
                        Key::KeyS => {
                            file = "sounds/key2.wav";
                            *released = false;
                        }
                        Key::KeyD => {
                            file = "sounds/key3.wav";
                            *released = false;
                        }
                        _ => {
                            *released = false;
                        }
                    }
                    std::thread::spawn(move || {
                        play_click_sound(file);
                    });
                }
                _ => {}
            }

            emit("Some", &string);
        }
        None => match event.event_type {
            EventType::KeyPress(key) => {
                if *released {
                    match key {
                        Key::ShiftLeft => {
                            file = "sounds/shift.wav";
                            *released = false;
                        }
                        Key::CapsLock => {
                            file = "sounds/caps.wav";
                            *released = false;
                        }
                        Key::ControlLeft => {
                            file = "sounds/shift.wav";
                            *released = false;
                        }
                        _ => {
                            file = "sounds/key3.wav";
                            *released = false;
                        }
                    }
                    std::thread::spawn(move || {
                        play_click_sound(file);
                    });
                }
            }
            EventType::KeyRelease(key) => match key {
                _ => {
                    *released = true;
                }
            },
            _ => {}
        },
    }
}

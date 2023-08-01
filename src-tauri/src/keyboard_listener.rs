// ./src-tauri/src/keyboard_listener.rs
use crate::utils::get_current_theme_folder;
use crate::{config, theme};
use rdev::{listen, Event, EventType, Key};
use rodio::OutputStream;
use std::fmt::{self, Display};
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
    let config = config::read_config();
    let theme_config = config::read_config_theme(config.theme);
    match event.name {
        Some(string) => {
            match event.event_type {
                EventType::KeyPress(key) => match key {
                    _ => {
                        std::thread::spawn(move || {
                            let key_string = key_to_string(key);
                            let sound_path = theme_config.keys.get(&key_string).unwrap();
                            let path = get_current_theme_folder()
                                .join(sound_path)
                                .into_os_string()
                                .into_string()
                                .unwrap();
                            play_click_sound(&path);
                        });

                        *released = false;
                    }
                },
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

fn key_to_string(key: Key) -> String {
    match key {
        Key::Alt => format!("{}", "Alt"),
        Key::AltGr => format!("{}", "AltGr"),
        Key::Backspace => format!("{}", "Backspace"),
        Key::CapsLock => format!("{}", "CapsLock"),
        Key::ControlLeft => format!("{}", "ControlLeft"),
        Key::ControlRight => format!("{}", "ControlRight"),
        Key::Delete => format!("{}", "Delete"),
        Key::DownArrow => format!("{}", "DownArrow"),
        Key::End => format!("{}", "End"),
        Key::Escape => format!("{}", "Escape"),
        Key::F1 => format!("{}", "F1"),
        Key::F10 => format!("{}", "F10"),
        Key::F11 => format!("{}", "F11"),
        Key::F12 => format!("{}", "F12"),
        Key::F2 => format!("{}", "F2"),
        Key::F3 => format!("{}", "F3"),
        Key::F4 => format!("{}", "F4"),
        Key::F5 => format!("{}", "F5"),
        Key::F6 => format!("{}", "F6"),
        Key::F7 => format!("{}", "F7"),
        Key::F8 => format!("{}", "F8"),
        Key::F9 => format!("{}", "F9"),
        Key::Home => format!("{}", "Home"),
        Key::LeftArrow => format!("{}", "LeftArrow"),
        Key::MetaLeft => format!("{}", "MetaLeft"),
        Key::MetaRight => format!("{}", "MetaRight"),
        Key::PageDown => format!("{}", "PageDown"),
        Key::PageUp => format!("{}", "PageUp"),
        Key::Return => format!("{}", "Return"),
        Key::RightArrow => format!("{}", "RightArrow"),
        Key::ShiftLeft => format!("{}", "ShiftLeft"),
        Key::ShiftRight => format!("{}", "ShiftRight"),
        Key::Space => format!("{}", "Space"),
        Key::Tab => format!("{}", "Tab"),
        Key::UpArrow => format!("{}", "UpArrow"),
        Key::KeyA => format!("{}", "KeyA"),
        Key::KeyB => format!("{}", "KeyB"),
        Key::KeyC => format!("{}", "KeyC"),
        Key::KeyD => format!("{}", "KeyD"),
        Key::KeyE => format!("{}", "KeyE"),
        Key::KeyF => format!("{}", "KeyF"),
        Key::KeyG => format!("{}", "KeyG"),
        Key::KeyH => format!("{}", "KeyH"),
        Key::KeyI => format!("{}", "KeyI"),
        Key::KeyJ => format!("{}", "KeyJ"),
        Key::KeyK => format!("{}", "KeyK"),
        Key::KeyL => format!("{}", "KeyL"),
        Key::KeyM => format!("{}", "KeyM"),
        Key::KeyN => format!("{}", "KeyN"),
        Key::KeyO => format!("{}", "KeyO"),
        Key::KeyP => format!("{}", "KeyP"),
        Key::KeyQ => format!("{}", "KeyQ"),
        Key::KeyR => format!("{}", "KeyR"),
        Key::KeyS => format!("{}", "KeyS"),
        Key::KeyT => format!("{}", "KeyT"),
        Key::KeyU => format!("{}", "KeyU"),
        Key::KeyV => format!("{}", "KeyV"),
        Key::KeyW => format!("{}", "KeyW"),
        Key::KeyX => format!("{}", "KeyX"),
        Key::KeyY => format!("{}", "KeyY"),
        Key::KeyZ => format!("{}", "KeyZ"),

        _ => format!("{}", "KeyA"),
    }
}

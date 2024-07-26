use rdev::{simulate, Event, EventType, Key, grab};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
	let mut key_mapping = HashMap::new();
	key_mapping.insert(Key::UpArrow, Key::KeyF);
	key_mapping.insert(Key::LeftArrow, Key::KeyL);
	key_mapping.insert(Key::DownArrow, Key::KeyB);
	key_mapping.insert(Key::RightArrow, Key::KeyR);
	key_mapping.insert(Key::ShiftLeft, Key::KeyS);
	key_mapping.insert(Key::Space, Key::KeyJ);
	key_mapping.insert(Key::ControlLeft, Key::KeyH);

	// Shared state for pause
	let is_paused = Arc::new(Mutex::new(false));

	// Listener thread for key events
	let listener_thread = {
		let is_paused_listener = Arc::clone(&is_paused);
		thread::spawn(move || {
			if let Err(error) = grab(move |event| {
				callback(event, &key_mapping, &is_paused_listener)
			}) {
				println!("Error: {:?}", error);
			}
		})
	};

	listener_thread.join().unwrap();
}

fn callback(event: Event, key_mapping: &HashMap<Key, Key>, is_paused: &Arc<Mutex<bool>>) -> Option<Event> {
	let paused = *is_paused.lock().unwrap();

	if let EventType::KeyPress(key) = event.event_type {
		if key == Key::Escape {
			// Pause and unpause on ESC
			let mut paused = is_paused.lock().unwrap();
			*paused = !*paused;
			println!("Paused: {}", *paused);
			return None;
		} else if let Some(&new_key) = key_mapping.get(&key) {
			if paused {
				return Some(event);
			}

			simulate_key_press(new_key);
			return None;
		}
	}

	Some(event)
}

fn simulate_key_press(key: Key) {
	simulate(&EventType::KeyPress(key)).unwrap();
	simulate(&EventType::KeyRelease(key)).unwrap();
}

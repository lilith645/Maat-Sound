extern crate ears;

use std::collections::HashMap;

use ears::{Sound, AudioController};



pub struct DukeBox {
  music: HashMap<String, Sound>,
}

impl DukeBox {
  pub fn new() -> DukeBox {
    DukeBox {
      music: HashMap::new(),
    }
  }
  
  pub fn load(&mut self, name: String, location: String) {
    if let Ok(new_sound) = Sound::new(&location) {
      self.music.insert(name, new_sound);
    } else {
      println!("Error: Failed to load sound: {:?}", location);
    }
  }
  
  pub fn play(&mut self, name: String) {
    if self.music.contains_key(&name) {
      self.music.get_mut(&name).unwrap().play();
    }
  }
}

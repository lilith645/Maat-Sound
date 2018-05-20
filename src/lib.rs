extern crate ears;

use std::collections::HashMap;

use ears::{Sound, AudioController};
use ears::State::{Playing, Stopped, Paused};


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
    } else {
      println!("Error: Sound {:?} not loaded", name);
    }
  }
  
  pub fn pause(&mut self, name: String) {
    if self.music.contains_key(&name) {
      self.music.get_mut(&name).unwrap().pause();
    } else {
      println!("Error: Sound {:?} not loaded", name);
    }
  }
  
  pub fn stop(&mut self, name: String) {
    if self.music.contains_key(&name) {
      self.music.get_mut(&name).unwrap().stop();
    } else {
      println!("Error: Sound {:?} not loaded", name);
    }
  }
  
  pub fn get_state(&mut self, name: String) -> ears::State {
    if self.music.contains_key(&name) {
      return self.music.get_mut(&name).unwrap().get_state()
    } else {
      println!("Error: Sound {:?} not loaded", name);
      Stopped
    }
  }
  
  pub fn sound_playing(&self, name: String) -> bool {
    if self.music.contains_key(&name) {
      return self.music.get(&name).unwrap().is_playing()
    }
    println!("Error: Sound {:?} not loaded", name);
    false
  }
  
  pub fn loop_sound(&mut self, name: String, loop_sound: bool) {
    if self.music.contains_key(&name) {
      self.music.get_mut(&name).unwrap().set_looping(loop_sound);
    } else {
      println!("Error: Sound {:?} not loaded", name);
    }
  }
  
  pub fn set_volume(&mut self, name: String, volume: f32) {
    if self.music.contains_key(&name) {
      self.music.get_mut(&name).unwrap().set_volume(volume);
    } else {
      println!("Error: Sound {:?} not loaded", name);
    }
  }
  
  pub fn set_all_volume(&mut self, volume: f32) {
    for mut sound in &mut self.music {
      sound.1.set_volume(volume);
    }
  }
  
  pub fn get_volume(&self, name: String) -> f32 {
    if self.music.contains_key(&name) {
      self.music.get(&name).unwrap().get_volume()
    } else {
      println!("Error: Sound {:?} not loaded", name);
      -1.0
    }
  }
}

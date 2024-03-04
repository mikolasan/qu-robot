use std::collections::HashMap;
use super::frozen_lake::STATES;

struct StateEncoder {
  states: Vec<char>,
  encoding_map: HashMap<char, i8>,
}

impl StateEncoder {
    pub fn new(&mut self) {
      self.states = STATES.to_vec();
    }

    pub fn encode(&self, state: char) -> i8 {
      self.encoding_map[&state]
    }
}
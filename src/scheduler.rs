use std::rc::Rc;
use std::collections::HashMap;

use crate::neuron::Neuron;

pub struct Scheduler<'a> {
  propagating: bool,
  time: u64,
  scheduled: HashMap<u64, Vec<Box<Neuron<'a>>>>,
}

impl<'a> Scheduler<'a> {
  pub fn new() -> Self {
    Scheduler {
      propagating: false,
      time: 0,
      scheduled: HashMap::new(),
    }
  }

  pub fn start(&mut self, starting_objects: Vec<Box<Neuron>>) {
    self.propagating = true;

    while self.propagating {
      // check inputs
      for obj in &starting_objects {
        obj.activate(self.time);
      }
      self.run_scheduled(self.time);
      self.time += 1;
    }
  }

  pub fn add(&mut self, time: u64, neuron: &Neuron<'a>) {
    if !self.scheduled.contains_key(&time) {
      self.scheduled.insert(time, Vec::new());
    }
    
    if let Some(neurons) = self.scheduled.get_mut(&time) {
      neurons.iter()
        .position(|n| n.id == neuron.id)
        .or_else(|| {
          neurons.push(Box::new(*neuron));
          None
        });
    }
  }

  fn run_scheduled(&mut self, time: u64) {
    if !self.scheduled.contains_key(&time) {
      println!("nothing scheduled for time {}. stop.", time);
      self.propagating = false;
      return;
    }

    for neuron in self.scheduled[&time].iter() {
      neuron.activate(time);
    }
  }
}


#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use crate::neuron::Neuron;

  use super::Scheduler;

  #[test]
  fn main() {
    let mut scheduler = Rc::new(Scheduler::new());
    let neuron = Neuron::new("test".to_string(), Rc::clone(&scheduler), 0);
    let p_neuron = Rc::new(neuron);

    // Example usage
    scheduler.add(1, &neuron);
    scheduler.start(vec![p_neuron]);
  }
}
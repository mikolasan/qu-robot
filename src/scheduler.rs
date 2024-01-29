use std::collections::HashMap;

use crate::neuron::Neuron;

struct Scheduler {
  propagating: bool,
  time: u64,
  scheduled: HashMap<u64, Vec<Neuron>>,
}

impl Scheduler {
  fn new() -> Self {
    Scheduler {
      propagating: false,
      time: 0,
      scheduled: HashMap::new(),
    }
  }

  fn start(&mut self, starting_objects: Vec<Neuron>) {
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

  fn add(&mut self, time: u64, neuron: Neuron) {
    if !self.scheduled.contains_key(&time) {
      self.scheduled.insert(time, vec![neuron]);
    } else if let Some(neurons) = self.scheduled.get_mut(&time) {
      if !neurons.contains(&neuron) {
        neurons.push(neuron);
      }
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
    use crate::neuron::Neuron;

    use super::Scheduler;


  #[test]
  fn main() {
    let scheduler = Scheduler::new();
    let neuron = Neuron;

    // Example usage
    scheduler.add(1, neuron);
    scheduler.start(vec![neuron]);
  }
}
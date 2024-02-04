use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use uuid::Uuid;

use crate::scheduler::Scheduler;

struct Axon {
  // neuron: Weak<Neuron>,
  connections: HashMap<[u8; 16], Vec<Weak<Dendrite>>>,
}

impl Axon {
  pub fn new() -> Self {
    Axon {
      connections: HashMap::new(),
    }
  }

  fn add_dendrite(&mut self, neuron: Weak<Neuron>, dendrite: Weak<Dendrite>) {
    let neuron_id = neuron.upgrade().unwrap()
      .id.as_bytes();
    if !self.connections.contains_key(neuron_id) {
      self.connections.insert(*neuron_id, Vec::new());
    }
    let dendrites = self.connections.get_mut(neuron_id).unwrap();
    dendrites.push(dendrite);
  }

  fn activate(&mut self, time: u64, output: f64) {
    for connections in self.connections.values_mut() {
      for dendrite in connections {
        dendrite.upgrade().unwrap()
          .activate(time, output);
      }
    }

  }
}

struct Dendrite {
  neuron: Weak<Neuron>,
  strength: f64,
}

impl Dendrite {
  pub fn new(neuron: Weak<Neuron>, strength: f64) -> Self {
    Dendrite {
      neuron,
      strength,
    }
  }

  pub fn activate(&mut self, time: u64, input: f64) {
    if self.strength > 0.5 {
      println!("{{time {}}} -> ({}) {{{}}}", time, self.neuron.upgrade().unwrap().name, input);
      self.neuron.upgrade().unwrap()
        .record_input(time, input);
      self.strength += 0.1;
    }
  }
}

pub struct Neuron {
  dendrites: Vec<Rc<Dendrite>>,
  axon: Rc<Axon>,
  plasticity_on: bool,
  inputs: HashMap<u64, Vec<f64>>,
  name: String,
  pub id: Uuid,
  transform_fn: fn(Vec<f64>) -> f64,
  scheduler: RefCell<Scheduler>,
  activation_delay: u64,
}

impl Neuron {
  pub fn new(name: String, scheduler: RefCell<Scheduler>, activation_delay: u64) -> Self {
    Neuron {
      dendrites: Vec::new(),
      axon: Rc::new(Axon::new()),
      plasticity_on: false,
      inputs: HashMap::new(),
      name,
      id: Uuid::new_v4(),
      transform_fn: sum,
      scheduler,
      activation_delay,
    }
  }

  fn add_connection(mut self: Rc<Self>, neuron: &mut Rc<Neuron>, strength: Option<f64>) {
    let dendrite = Rc::new(Dendrite::new(
      Rc::downgrade(&self),
      strength.unwrap_or_else(|| rand::random())
    ));
    // add connections to pre-synaptic neuron
    neuron.axon.add_dendrite(
      Rc::downgrade(&self),
      Rc::downgrade(&dendrite)
    );
    // add connections to post-synaptic neuron (this)
    self.dendrites.push(dendrite);
  }

  fn add_input(mut self: Rc<Self>, sensor: &mut Sensor) {
    let dendrite = Rc::new(Dendrite::new(
      Rc::downgrade(&self),
      1.0
    ));
    // pre-synaptic part
    sensor.connections.push(Rc::downgrade(&dendrite));
    // add connections to post-synaptic neuron (this)
    self.dendrites.push(dendrite);
  }

  fn record_input(&mut self, time: u64, input: f64) {
    let activation_time = time + self.activation_delay;
    if !self.inputs.contains_key(&activation_time) {
      self.inputs.insert(activation_time, vec![input]);
    } else if let Some(inputs) = self.inputs.get_mut(&activation_time) {
      inputs.push(input);
    }
    self.scheduler.get_mut().add(activation_time, self);
  }

  pub fn activate(&mut self, time: u64) {
    let output = (self.transform_fn)(self.inputs[&time].clone());
    println!("{{time {}}} Neuron({}) output -> {}", time, self.name, output);
    self.inputs.remove(&time);
    self.axon.activate(time + 1, output);
  }
}


struct Sensor {
  connections: Vec<Weak<Dendrite>>,
}

impl Sensor {
  // Implement Sensor methods as needed
}

fn sum(values: Vec<f64>) -> f64 {
  values.iter().sum()
}

#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  use std::rc::Rc;
  use crate::neuron::{Neuron, Sensor};
  use super::Scheduler;

  
  #[test]
  fn main() {
    // Example usage
    let mut scheduler = RefCell::new(Scheduler::new());
    let mut neuron1 = Neuron::new(
      "Neuron1".to_string(), 
      scheduler, 
      0
    );
    let mut neuron2 = Neuron::new(
      "Neuron2".to_string(), 
      scheduler, 
      0
    );
    let mut sensor = Sensor { 
      connections: Vec::new() 
    };

    // neuron1.add_connection(&mut neuron2, None);
    // neuron1.add_input(&mut sensor);
    // neuron1.record_input(1, 0.5);
    // scheduler.start(vec![Rc::new(neuron1), Rc::new(neuron2)]);
  }
}
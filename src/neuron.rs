use std::rc::Rc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::scheduler::Scheduler;

struct Axon<'a> {
  neuron: &'a Neuron<'a>,
  connections: HashMap<[u8; 16], Vec<Rc<Dendrite<'a>>>>,
}

impl<'a> Axon<'a> {
  pub fn new(neuron: &Neuron) -> Self {
    Axon {
      neuron,
      connections: HashMap::new(),
    }
  }

  fn add_dendrite(&mut self, neuron: Rc<Neuron>, dendrite: Rc<Dendrite<'a>>) {
    let neuron_id = neuron.id.as_bytes();
    if !self.connections.contains_key(neuron_id) {
      self.connections.insert(*neuron_id, vec![dendrite]);
    } else {
      let dendrites = self.connections.get_mut(neuron_id).unwrap();
      dendrites.push(dendrite);
    }
  }

  fn activate(&mut self, time: u64, output: f64) {
    for connections in self.connections.values_mut() {
      for dendrite in connections {
        dendrite.activate(time, output);
      }
    }

  }
}

struct Dendrite<'a> {
  neuron: Rc<Neuron<'a>>,
  strength: f64,
}

impl<'a> Dendrite<'a> {
  pub fn new(neuron: Rc<Neuron<'a>>, strength: f64) -> Self {
    Dendrite { neuron, strength }
  }

  pub fn activate(&mut self, time: u64, input: f64) {
    if self.strength > 0.5 {
      println!("{{time {}}} -> ({}) {{{}}}", time, self.neuron.name, input);
      self.neuron.record_input(time, input);
      self.strength += 0.1;
    }
  }
}

pub struct Neuron<'a> {
  dendrites: Vec<Box<Dendrite<'a>>>,
  axon: Option<Axon<'a>>,
  plasticity_on: bool,
  inputs: HashMap<u64, Vec<f64>>,
  name: String,
  pub id: Uuid,
  transform_fn: fn(Vec<f64>) -> f64,
  scheduler: Rc<Scheduler<'a>>,
  activation_delay: u64,
}

impl<'a> Neuron<'a> {
  pub fn new(name: String, scheduler: Rc<Scheduler>, activation_delay: u64) -> Self {
    let mut neuron = Neuron {
      dendrites: Vec::new(),
      axon: None,
      plasticity_on: false,
      inputs: HashMap::new(),
      name,
      id: Uuid::new_v4(),
      transform_fn: sum,
      scheduler,
      activation_delay,
    };
    let axon = Some(Axon::new(&neuron));
    neuron.axon = axon;
    neuron
  }

  fn add_connection(&mut self, neuron: &mut Neuron, strength: Option<f64>) {
    let this_neuron = self as &Neuron;
    let dendrite = Box::new(Dendrite::new(
      Rc::new(*this_neuron),
      strength.unwrap_or_else(|| rand::random())
    ));
    // add connections to pre-synaptic neuron
    let mut axon = neuron.axon.unwrap();
    axon.add_dendrite(Rc::new(*this_neuron), Rc::new(*dendrite));
    // add connections to post-synaptic neuron (this)
    self.dendrites.push(dendrite);
  }

  fn add_input(&mut self, sensor: &mut Sensor) {
    let this_neuron = self as &Neuron;
    let dendrite = Box::new(Dendrite::new(Rc::new(*this_neuron), 1.0));
    // pre-synaptic part
    sensor.connections.push(Rc::new(*dendrite));
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
    let this_neuron = self as &Neuron;
    self.scheduler.add(activation_time, self);
  }

  pub fn activate(&mut self, time: u64) {
    let output = (self.transform_fn)(self.inputs[&time].clone());
    println!("{{time {}}} Neuron({}) output -> {}", time, self.name, output);
    self.inputs.remove(&time);
    self.axon.unwrap().activate(time + 1, output);
  }
}


struct Sensor<'a> {
  connections: Vec<Rc<Dendrite<'a>>>,
}

impl<'a> Sensor<'a> {
  // Implement Sensor methods as needed
}

fn sum(values: Vec<f64>) -> f64 {
  values.iter().sum()
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use crate::neuron::{Neuron, Sensor};
  use super::Scheduler;

  
  #[test]
  fn main() {
    // Example usage
    let mut scheduler = Rc::new(Scheduler::new());
    let mut neuron1 = Neuron::new("Neuron1".to_string(), Rc::clone(&scheduler), 0);
    let mut neuron2 = Neuron::new("Neuron2".to_string(), Rc::clone(&scheduler), 0);
    let mut sensor = Sensor { connections: Vec::new() };

    neuron1.add_connection(&mut neuron2, None);
    neuron1.add_input(&mut sensor);
    neuron1.record_input(1, 0.5);
    scheduler.start(vec![Rc::new(neuron1), Rc::new(neuron2)]);
  }
}
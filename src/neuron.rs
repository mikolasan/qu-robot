use std::collections::HashMap;
use uuid::Uuid;

pub struct Neuron {
  dendrites: Vec<Dendrite>,
  axon: Axon,
  plasticity_on: bool,
  inputs: HashMap<u64, Vec<f64>>,
  name: String,
  id: Uuid,
  transform_fn: fn(Vec<f64>) -> f64,
  scheduler: Scheduler,
  activation_delay: u64,
}

impl Neuron {
  fn new(name: String, scheduler: Scheduler, activation_delay: u64) -> Self {
    Neuron {
      dendrites: Vec::new(),
      axon: Axon {},
      plasticity_on: false,
      inputs: HashMap::new(),
      name,
      id: Uuid::new_v4(),
      transform_fn: sum,
      scheduler,
      activation_delay,
    }
  }

  fn add_connection(&mut self, neuron: &mut Neuron, strength: Option<f64>) {
    let dendrite = Dendrite::new(
      self,
      strength.unwrap_or_else(|| rand::random())
    );
    // add connections to pre-synaptic neuron
    neuron.axon.add_dendrite(self, dendrite);
    // add connections to post-synaptic neuron (this)
    self.dendrites.push(dendrite);
  }

  fn add_input(&mut self, sensor: &mut Sensor) {
    let dendrite = Dendrite::new(self, 1.0);
    // pre-synaptic part
    sensor.connections.push(dendrite);
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
    self.scheduler.add(activation_time, self);
  }

  pub fn activate(&mut self, time: u64) {
    let output = (self.transform_fn)(self.inputs[&time].clone());
    println!("{{time {}}} Neuron({}) output -> {}", time, self.name, output);
    self.inputs.remove(&time);
    self.axon.activate(time + 1, output);
  }
}

struct Axon;

impl Axon {
  fn activate(&self, time: u64, output: f64) {
    // Implement axon activation logic
  }
}

struct Dendrite {
  neuron: Neuron,
  strength: f64,
}

impl Dendrite {
  fn new(neuron: Neuron, strength: f64) -> Self {
    Dendrite { neuron, strength }
  }
}

struct Sensor {
  connections: Vec<Dendrite>,
}

impl Sensor {
  // Implement Sensor methods as needed
}

fn sum(values: Vec<f64>) -> f64 {
  values.iter().sum()
}

#[cfg(test)]
mod tests {

  #[test]
  fn main() {
    // Example usage
    let mut scheduler = Scheduler::new();
    let mut neuron1 = Neuron::new("Neuron1".to_string(), scheduler.clone(), 0);
    let mut neuron2 = Neuron::new("Neuron2".to_string(), scheduler.clone(), 0);
    let mut sensor = Sensor { connections: Vec::new() };

    neuron1.add_connection(&mut neuron2, None);
    neuron1.add_input(&mut sensor);
    neuron1.record_input(1, 0.5);
    scheduler.start(vec![neuron1, neuron2]);
  }
}
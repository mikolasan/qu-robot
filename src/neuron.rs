use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::string;
use uuid::Uuid;

// use crate::scheduler::Scheduler;

pub type NeuronId = String;

// struct Axon {
//   // neuron: Weak<Neuron>,
//   connections: HashMap<NeuronId, Vec<RefCell<Dendrite>>>,
// }

// impl Axon {
//   pub fn new() -> Self {
//     Axon {
//       connections: HashMap::new(),
//     }
//   }

//   fn add_dendrite(&mut self, neuron_id: &NeuronId, dendrite: RefCell<Dendrite>) {
//     if !self.connections.contains_key(neuron_id) {
//       self.connections.insert(*neuron_id, Vec::new());
//     }
//     let dendrites = self.connections.get_mut(neuron_id).unwrap();
//     dendrites.push(dendrite);
//   }

//   fn activate(&self, time: u64, output: f64) {
//     for connections in self.connections.values() {
//       for dendrite in connections {
//         dendrite.borrow_mut()
//           .activate(time, output);
//       }
//     }

//   }
// }

struct Dendrite {
  // neuron: Weak<Neuron>,
  strength: f64,
}

impl Dendrite {
  pub fn new(strength: f64) -> Self {
    Dendrite {
      strength
    }
  }

  pub fn activate(&mut self) {
    self.strength += 0.1;
  }

  // pub fn new(neuron: Weak<Neuron>, strength: f64) -> Self {
  //   Dendrite {
  //     neuron,
  //     strength,
  //   }
  // }

  // pub fn activate(&mut self, time: u64, input: f64) {
  //   if self.strength > 0.5 {
  //     println!("{{time {}}} -> ({}) {{{}}}", time, self.neuron.upgrade().unwrap().name, input);
  //     self.neuron.upgrade().unwrap()
  //       .record_input(time, input);
  //     self.strength += 0.1;
  //   }
  // }
}

// Transform functions
fn sum(values: Vec<f64>) -> f64 {
  values.iter().sum()
}

pub struct Neuron {
  dendrites: Vec<Dendrite>,
  axon_connections: Vec<Neuron>,
//   axon: Rc<Axon>,
//   inputs: HashMap<u64, Vec<f64>>,
  name: String,
//   pub id: Uuid,
//   transform_fn: fn(Vec<f64>) -> f64,
//   scheduler: RefCell<Scheduler>,
//   activation_delay: u64,
}

impl Neuron {
  pub fn new(name: String) -> Self {
    Neuron {
      dendrites: Vec::new(),
      axon_connections: Vec::new(),
      name,
    }
  }
//   pub fn new(name: String, scheduler: RefCell<Scheduler>, activation_delay: u64) -> Self {
//     Neuron {
//       dendrites: Vec::new(),
//       axon: Rc::new(Axon::new()),
//       plasticity_on: false,
//       inputs: HashMap::new(),
//       name,
//       id: Uuid::new_v4(),
//       transform_fn: sum,
//       scheduler,
//       activation_delay,
//     }
//   }

  fn add_dendrite(&mut self, strength: Option<f64>) -> &Dendrite {
    let dendrite = Dendrite::new(strength.unwrap_or_else(|| rand::random()));
    self.dendrites.push(dendrite);
    self.dendrites.last().unwrap()
  }

  fn add_connection(&mut self, neuron: Neuron, strength: Option<f64>) -> &mut Neuron {
    self.axon_connections.push(neuron);
    let new_neuron = self.axon_connections.last_mut().unwrap();
    // self.axon_connections.last_mut().unwrap()
    //   .add_dendrite(strength);
    new_neuron.add_dendrite(strength);
    new_neuron
  }

  fn get_last_neuron_mut(&mut self) -> &mut Neuron {
    self.axon_connections.last_mut().unwrap()
  }

//   fn add_connection(mut self: Rc<Self>, neuron: &mut Rc<Neuron>, strength: Option<f64>) {
//     let dendrite = RefCell::new(Dendrite::new(
//       Rc::downgrade(&self),
//       strength.unwrap_or_else(|| rand::random())
//     ));
//     // add connections to pre-synaptic neuron
//     let neuron_id = neuron.id.as_bytes();
//     neuron.axon.add_dendrite(
//       neuron_id,
//       dendrite
//     );
//     // add connections to post-synaptic neuron (this)
//     self.dendrites.push(dendrite);
//   }

//   fn add_input(mut self: Rc<Self>, sensor: &mut Sensor) {
//     let dendrite = RefCell::new(Dendrite::new(
//       Rc::downgrade(&self),
//       1.0
//     ));
//     // pre-synaptic part
//     sensor.connections.push(dendrite.clone());
//     // add connections to post-synaptic neuron (this)
//     self.dendrites.push(dendrite);
//   }

//   fn record_input(&mut self, time: u64, input: f64) {
//     let activation_time = time + self.activation_delay;
//     if !self.inputs.contains_key(&activation_time) {
//       self.inputs.insert(activation_time, vec![input]);
//     } else if let Some(inputs) = self.inputs.get_mut(&activation_time) {
//       inputs.push(input);
//     }
//     self.scheduler.get_mut().add(activation_time, RefCell::new(*self));
//   }

//   pub fn activate(&mut self, time: u64) {
//     let output = (self.transform_fn)(self.inputs[&time].clone());
//     println!("{{time {}}} Neuron({}) output -> {}", time, self.name, output);
//     self.inputs.remove(&time);
//     self.axon.activate(time + 1, output);
//   }
  pub fn activate(&self, time: u64) {
    for neuron in &self.axon_connections {
      neuron.activate(time)
    }
  }
}


// struct Sensor {
//   connections: Vec<RefCell<Dendrite>>,
// }

// impl Sensor {
//   // Implement Sensor methods as needed
// }


#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  use std::rc::Rc;
  use crate::neuron::{
    Dendrite,
    Neuron, 
    // Sensor
  };
  // use super::Scheduler;

  
  #[test]
  fn dendrite_test() {
    let mut d = Dendrite::new(0.0);
    d.activate();
    assert_eq!(d.strength, 0.1, "strength after one activation is wrong");
    
    // let mut n1 = Neuron::new();
    // let n2 = Neuron::new();
    // let n3 = Neuron::new();
    // let n2 = n1.add_connection(n2, Some(1.0));
    // let n3 = n2.add_connection(n3, Some(1.0));
    // n1.activate(0);

    // let mut n1 = Neuron::new();
    // n1
    //   .add_connection(Neuron::new(), Some(1.0))
    //   .add_connection(Neuron::new(), Some(1.0));
    // n1.activate(0);

    let mut n1 = Neuron::new("n1".to_string());
    n1.add_connection(Neuron::new("n21".to_string()), Some(1.0));
    n1.add_connection(Neuron::new("n22".to_string()), Some(1.0));
    n1.get_last_neuron_mut().add_connection(Neuron::new("32".to_string()), Some(1.0));
    n1.activate(0);
  }

  // #[test]
  // fn main() {
  //   // Example usage
  //   let mut scheduler = RefCell::new(Scheduler::new());
  //   let mut neuron1 = Neuron::new(
  //     "Neuron1".to_string(), 
  //     scheduler, 
  //     0
  //   );
  //   let mut neuron2 = Neuron::new(
  //     "Neuron2".to_string(), 
  //     scheduler, 
  //     0
  //   );
  //   let mut sensor = Sensor { 
  //     connections: Vec::new() 
  //   };

  //   neuron1.add_connection(&mut neuron2, None);
  //   neuron1.add_input(&mut sensor);
  //   neuron1.record_input(1, 0.5);
  //   scheduler.start(vec![Rc::new(neuron1), Rc::new(neuron2)]);
  // }
}
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::rc::{Rc};
use std::collections::HashMap;
use std::string;
use std::sync::{Arc,Weak};
use uuid::Uuid;

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
  neuron_id: String,
  strength: f64,
}

impl Dendrite {
  pub fn new(neuron_id: String, strength: f64) -> Self {
    Dendrite {
      neuron_id,
      strength
    }
  }

  pub fn inc_strength(&mut self) {
    self.strength += 0.1;
  }

  pub fn dec_strength(&mut self) {
    self.strength -= 0.1;
  }

  pub fn get_neuron_id(&self) -> &String {
    &self.neuron_id
  }

  // pub fn get_neuron_mut(&mut self) -> &mut Arc<Neuron> {
  //   &mut self.neuron
  // }

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

// #[derive(Clone)]
pub struct Neuron {
  // pre_synaptic_connections
  post_synaptic_connections: Vec<Box<Dendrite>>,

  // dendrites: Vec<Arc<Dendrite>>,
  // axon_connections: Vec<Dendrite>,
//   axon: Rc<Axon>,
//   inputs: HashMap<u64, Vec<f64>>,
  name: String,
//   pub id: Uuid,
  pub potential: f64,
  pub threshold: i32,
  transform_fn: fn(Vec<f64>) -> f64,
//   scheduler: RefCell<Scheduler>,
//   activation_delay: u64,
}

impl Neuron {
  pub fn empty() -> Self {
    Neuron {
      post_synaptic_connections: Vec::new(),
      // dendrites: Vec::new(),
      // axon_connections: Vec::new(),
      name: String::new(),
      potential: 0.0,
      threshold: i32::MAX,
      transform_fn: sum,
    }
  }

  pub fn new(threshold: i32, name: Option<String>) -> Self {
    Neuron {
      post_synaptic_connections: Vec::new(),
      // dendrites: Vec::new(),
      // axon_connections: Vec::new(),
      name: name.unwrap_or_else(|| Uuid::new_v4().to_string()),
      potential: 0.0,
      threshold,
      transform_fn: sum,
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

  pub fn set_name(&mut self, new_name: String) {
    self.name = new_name;
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn connect_to(&mut self, post_neuron_id: String, strength: Option<f64>) {
    self.post_synaptic_connections.push(Box::new(
      Dendrite {
        neuron_id: post_neuron_id,
        strength: strength.unwrap_or(0.0),
      }
    ));
  }

  // fn create_dendrite(self: &Arc<Self>, strength: Option<f64>) -> Dendrite {
  //   Dendrite::new(
  //     Arc::clone(&self),
  //     strength.unwrap_or_else(|| rand::random())
  //   )
  // }
  
  // fn add_dendrite(&mut self, strength: Option<f64>) -> &Rc<Dendrite> {
  //   let dendrite = Rc::new(Dendrite::new(
  //     Rc::new(Neuron::new("name".to_string())),
  //     strength.unwrap_or_else(|| rand::random())));
  //   self.dendrites.push(dendrite);
  //   self.dendrites.last().unwrap()
  // }

  // pub fn add_connection(& mut self, neuron: &Arc<Neuron>, strength: Option<f64>) {
  //   // let mut binding = neuron.upgrade().unwrap();
  //   // let rc_neuron = binding.borrow_mut();
  //   // assert_eq!(1, Rc::strong_count(neuron), "expect only one ref for neuron");
  //   // let raw_neuron = Rc::make_mut(neuron);
  //   // let new_dendrite = raw_neuron.add_dendrite(strength);
  //   let new_dendrite = neuron.create_dendrite(strength);
  //   self.axon_connections.push(new_dendrite);
  //   // let new_neuron = self.axon_connections.last_mut().unwrap();
  //   // self.axon_connections.last_mut().unwrap()
  //   //   .add_dendrite(strength);
  //   // new_neuron

  // }

  // fn get_last_neuron_mut(&mut self) -> &mut Neuron {
  //   let last_dendrite = self.axon_connections.last_mut().unwrap();
  //   last_dendrite.get_neuron_mut()
  // }

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

  pub fn transmit(&mut self, activated_neurons: &mut HashMap<String, Vec<f64>>) {
    // self.potential += 1;
    // if self.potential < self.threshold {
    //   return;
    // }

    for dendrite in self.post_synaptic_connections.iter_mut() {
      let neuron_id = dendrite.get_neuron_id();
      if activated_neurons.contains_key(neuron_id) {
        activated_neurons.get_mut(neuron_id).unwrap()
          .push(dendrite.strength);
      } else {
        activated_neurons.insert(neuron_id.clone(), vec![dendrite.strength]);
      }
      // dendrite.inc_strength();
    }
    // self.post_synaptic_connections
    //   .iter()
    //   .map(|dendrite| dendrite.neuron_id.clone())
    //   .collect()
  }

  pub fn process_signals(&self, signals: &Vec<f64>) -> Option<f64> {
    let output = (self.transform_fn)(signals.clone());
    Some(output)
  }

  pub fn update_potential(&mut self, diff: f64) {
    self.potential += diff;
  }

  // pub fn activate(&mut self, time: u64) {
  //   for dendrite in self.axon_connections.iter_mut() {
  //     // let neuron = dendrite.get_neuron();
  //     dendrite.activate();
  //     // let mut dendrite_clone = dendrite.clone();
  //     // let new_dendrite = Arc::make_mut(&mut dendrite_clone);
  //     // println!("activating {} -> {}", self.name, neuron.get_name());
  //     // neuron.activate(time);
  //   }
  //   // for dendrite in self.axon_connections.iter_mut() {
  //   //   // let neuron = dendrite.get_neuron_mut();
  //   //   assert_eq!(1, Rc::strong_count(&dendrite), "expect only one ref for dendrite");
  //   //   let neuron = Rc::make_mut(dendrite)
  //   //     .get_neuron_mut();
  //   //   Rc::make_mut(neuron)
  //   //     .activate(time)
  //   // }
  // }

}

// impl Deref for Neuron {
//   type Target = 
//   fn deref(&self) -> &Self::Target {
//     &self.value
//   }
// }
// impl DerefMut for Neuron {
//   fn deref_mut(&mut self) -> &Self::Target {
//     &self.value
//   }
// }

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
  use std::sync::Arc;

  use crate::neuron::{
    Dendrite,
    Neuron, 
    // Sensor
  };
  // use super::Scheduler;

  
  #[test]
  fn dendrite_test() {
    // let mut d = Dendrite::new(
    //   Arc::new(Neuron::new("test neuron".to_string())),
    //   0.0);
    // d.activate();
    // assert_eq!(d.strength, 0.1, "strength after one activation is wrong");
    
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

    // let mut n1 = Neuron::new("n1".to_string());
    // n1.add_connection(&mut Rc::new(Neuron::new("n21".to_string())), Some(1.0));
    // n1.add_connection(&mut Rc::new(Neuron::new("n22".to_string())), Some(1.0));
    // n1.activate(0);

    let mut n1 = Neuron::new(0, Some("n11".to_string()));
    let mut n2 = Neuron::new(0, Some("n12".to_string()));
    let n21 = Arc::new(Neuron::new(0, Some("n21".to_string())));
    let n22 = Arc::new(Neuron::new(0, Some("n22".to_string())));
    // n1.add_connection(&n21, Some(1.0));
    // n1.add_connection(&n22, Some(1.0));
    // n2.add_connection(&n21, Some(1.0));
    // n2.add_connection(&n22, Some(1.0));
    // n1.activate(0);
    // n2.activate(0);
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
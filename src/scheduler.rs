use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// use crate::neuron::Neuron;

// pub struct Scheduler {
//   propagating: bool,
//   time: u64,
//   scheduled: HashMap<u64, Vec<RefCell<Neuron>>>,
// }

// impl Scheduler {
//   pub fn new() -> Self {
//     Scheduler {
//       propagating: false,
//       time: 0,
//       scheduled: HashMap::new(),
//     }
//   }



//   pub fn add(&mut self, time: u64, neuron: RefCell<Neuron>) {
//     if !self.scheduled.contains_key(&time) {
//       self.scheduled.insert(time, Vec::new());
//     }
    
//     if let Some(neurons) = self.scheduled.get_mut(&time) {
//       if neurons.iter().position(|n| n.borrow().id == neuron.borrow().id).is_none() {
//         neurons.push(neuron);
//       }
//     }
//   }

//   pub fn start(&mut self, starting_neurons: &mut Vec<RefCell<Neuron>>) {
//     self.propagating = true;

//     while self.propagating {
//       // check inputs
//       for neuron in starting_neurons.iter_mut() {
//         neuron.borrow_mut().activate(self.time);
//       }

//       if !self.scheduled.contains_key(&self.time) {
//         println!("nothing scheduled for time {}. stop.", self.time);
//         self.propagating = false;
//         break;
//       }
//       self.run_scheduled(self.time);
//       self.time += 1;
//     }
//   }

//   fn run_scheduled(&mut self, time: u64) {
//     for neuron in self.scheduled[&time].iter_mut() {
//       neuron.borrow_mut()
//         .activate(time);
//     }
//   }
// }


// #[cfg(test)]
// mod tests {
//   use std::cell::RefCell;
//   use std::rc::Rc;
//   use crate::neuron::Neuron;

//   use super::Scheduler;

//   #[test]
//   fn main() {
//     let mut scheduler = RefCell::new(Scheduler::new());
//     let neuron = RefCell::new(Neuron::new(
//       "test".to_string(), 
//       scheduler, 
//       0
//     ));
    
//     // Example usage
//     scheduler.get_mut().add(1, neuron);
//     scheduler.get_mut().start(&mut vec![neuron]);
//   }
// }
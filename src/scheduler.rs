use std::{cell::RefCell, result};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use uuid::Uuid;

use crate::neuron::{Neuron, self};

pub struct Scheduler {
  pool: HashMap<String, Arc<Box<Neuron>>>,
//   propagating: bool,
//   time: u64,
//   scheduled: HashMap<u64, Vec<RefCell<Neuron>>>,
}

impl Scheduler {
  pub fn new() -> Self {
    Scheduler {
      pool: HashMap::new(),
    }
  }
//   pub fn new() -> Self {
//     Scheduler {
//       propagating: false,
//       time: 0,
//       scheduled: HashMap::new(),
//     }
//   }

  // pub fn add_neuron(&mut self) {
  //   let neuron = Box::new(Neuron::new(0, "hmm".to_string()));
  //   self.test = Some(neuron);

  //   let neuron2 = Rc::new(
  //     Box::new(
  //       Neuron::new(0, "well".to_string())
  //   ));
  //   self.test2.push(neuron2);
  // }

  // pub fn add(&mut self, time: u64, mut neuron: Box<Neuron>) -> String {
  //   let name = Uuid::new_v4().to_string();
  //   neuron.set_name(name.clone());
  //   let new_neuron = Arc::new(neuron);
  //   self.pool.insert(name.clone(), new_neuron.clone());
  //   name
  // }

  pub fn add_neuron(&mut self, threshold: i32, name: Option<String>) -> String {
    let neuron = Box::new(Neuron::new(threshold, name));
    let neuron_id = neuron.get_name().clone();
    let new_neuron = Arc::new(neuron);
    self.pool.insert(neuron_id.clone(), new_neuron);
    neuron_id.clone()
  }

  pub fn connect_neurons(&mut self, pre_id: &String, post_id: &String, strength: Option<f64>) {
    let post = self.find_neuron_by_id(post_id);
    let pre = self.find_neuron_by_id_mut(pre_id).unwrap();
    pre.connect_to(post, strength);
  }

  pub fn prepare_next_layer(&mut self, mut activated_neurons: HashMap<String, Vec<f64>>) -> Vec<String> {
    let mut neurons_next_layer: Vec<String> = Vec::new();
    for (neuron_id, signals) in activated_neurons.iter() {
      if let Some(neuron) = self.find_neuron_by_id_mut(&neuron_id) {
        // TODO: name it! it's potential activity or something
        let result = neuron.process_signals(signals);
        if let Some(potential_diff) = result {
          neuron.update_potential(potential_diff as i32);
          if neuron.potential >= neuron.threshold {
            neurons_next_layer.push(neuron_id.clone());
          }
        }
      }
    }
    neurons_next_layer
  }

  pub fn send_action_potential(&mut self, activated_neurons: Vec<String>) {
    if activated_neurons.is_empty() {
      return;
    }

    // let mut neurons_next_layer: Vec<Vec<String>> = Vec::new();
    let mut strength_per_neuron: HashMap<String, Vec<f64>> = HashMap::new();
    for neuron_id in activated_neurons {
      if let Some(neuron) = self.find_neuron_by_id_mut(&neuron_id) {
        neuron.transmit(&mut strength_per_neuron);
      }
    }
    let neurons_next_layer: Vec<String> = self.prepare_next_layer(strength_per_neuron);
    self.send_action_potential(neurons_next_layer);

    // let neurons_next_layer: Vec<Vec<&String>> = activated_neurons.into_iter()
    //   .map(|neuron_id| 
    //     self.find_neuron_by_id_mut(neuron_id)
    //       .unwrap()
    //       .activate()
    //   )
    //   .collect();
    // neurons_next_layer.iter().for_each(|layer| 
    //   self.send_action_potential(*layer)
    // );
      
  }

  pub fn find_neuron_by_id_mut(&mut self, neuron_id: &String) -> Option<&mut Box<Neuron>> {
    let t = self.pool.get_mut(neuron_id);
    if let Some(tt) = t {
      // println!("count in find of {}: {}", tt.get_name(), Arc::weak_count(tt));
      let ttt = Arc::get_mut(tt).unwrap();
      return Some(ttt);
    }
    None
  }

  pub fn find_neuron_by_id(&self, neuron_id: &String) -> Arc<Box<Neuron>> {
    self.pool.get(neuron_id).unwrap().clone()
  }

  fn take_neuron_by_id(&mut self, neuron_id: &String) -> Option<Box<Neuron>> {
    if let Some(rc_neuron) = self.pool.remove(neuron_id) {
      Arc::into_inner(rc_neuron)
    } else {
      println!("not found by id {}", neuron_id);
      None
    }
  }

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

  pub fn start(&self, starting_neurons: &'static mut [&mut Neuron]) {
    // let mut handle_vec = vec![];
    // for neuron in starting_neurons.iter_mut() {
    //   let handle = thread::spawn(move || {
    //     (*neuron)
    //       .activate(0);
    //   });
    //   handle_vec.push(handle);
    // }
    // handle_vec.into_iter()
    //   .for_each(|handle| handle.join().unwrap());
    
    // for neuron in starting_neurons {
    //   Rc::make_mut(
    //     self.pool.get_mut(neuron.get_name())
    //       .unwrap()
    //   )
    //     .activate(0);
    // }

  }
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

  pub fn print_pool(&self) {
    for (id, neuron) in self.pool.iter() {
      println!("{} - {}", neuron.get_name(), neuron.potential);
    }
  }
}


#[cfg(test)]
mod tests {
  use std::cell::RefCell;
  use std::collections::HashMap;
  use std::rc::Rc;
  use std::sync::Arc;
  use crate::neuron::Neuron;

  use super::Scheduler;

  #[test]
  fn add_take_neurons() {
    let mut scheduler = Box::new(Scheduler::new());
    let id = scheduler.add_neuron(0, None);
    {
      let rc_neuron = scheduler.find_neuron_by_id(&id);
      println!("added neuron had id {}", rc_neuron.get_name());
    }
    let neuron = scheduler.take_neuron_by_id(&id);
    assert_eq!(neuron.is_none(), false, "take failed");
    let mut n = neuron.unwrap();
    n.set_name("cool".to_string());
    println!("taken neuron had name {}", n.get_name());
  }

  #[test]
  fn connect_neurons_branching() {
    let mut scheduler = Box::new(Scheduler::new());
    let id1 = scheduler.add_neuron(0, None);
    let id2 = scheduler.add_neuron(0, None);
    let id3 = scheduler.add_neuron(0, None);
    let id4 = scheduler.add_neuron(0, None);
    scheduler.connect_neurons(&id1, &id2, None);
    scheduler.connect_neurons(&id1, &id3, None);
    scheduler.connect_neurons(&id1, &id4, None);
    {
      let rc_neuron1 = scheduler.find_neuron_by_id(&id1);
      println!("can access neuron with id {}", rc_neuron1.get_name());

      let rc_neuron2 = scheduler.find_neuron_by_id(&id2);
      println!("can access neuron with id {}", rc_neuron2.get_name());

      let rc_neuron3 = scheduler.find_neuron_by_id(&id3);
      println!("can access neuron with id {}", rc_neuron3.get_name());

      let rc_neuron4 = scheduler.find_neuron_by_id(&id4);
      println!("can access neuron with id {}", rc_neuron4.get_name());
    }
  }

  #[test]
  fn connect_neurons_merging() {
    let mut scheduler = Box::new(Scheduler::new());
    let id1 = scheduler.add_neuron(0, None);
    let id2 = scheduler.add_neuron(0, None);
    let id3 = scheduler.add_neuron(0, None);
    let id4 = scheduler.add_neuron(0, None);
    let id5 = scheduler.add_neuron(0, None);
    scheduler.connect_neurons(&id1, &id2, None);
    scheduler.connect_neurons(&id3, &id2, None);
    scheduler.connect_neurons(&id2, &id4, None);
    scheduler.connect_neurons(&id2, &id5, None);
    {
      let rc_neuron1 = scheduler.find_neuron_by_id(&id1);
      println!("can access neuron with id {}", rc_neuron1.get_name());

      let rc_neuron2 = scheduler.find_neuron_by_id(&id2);
      println!("can access neuron with id {}", rc_neuron2.get_name());

      let rc_neuron3 = scheduler.find_neuron_by_id(&id3);
      println!("can access neuron with id {}", rc_neuron3.get_name());

      let rc_neuron4 = scheduler.find_neuron_by_id(&id4);
      println!("can access neuron with id {}", rc_neuron4.get_name());
    }
    scheduler.send_action_potential(vec![id1.clone(), id3.clone()]);
  }

  // OR logic, if-else block
  #[test]
  fn feedback_neurons() {
    let mut scheduler = Box::new(Scheduler::new());
    let i1 = scheduler.add_neuron(1, Some("input 1".to_string()));
    let i2 = scheduler.add_neuron(1, Some("input 2".to_string()));
    let encoder = scheduler.add_neuron(4, Some("encoder".to_string()));
    let correction = scheduler.add_neuron(3, Some("correction".to_string()));
    let latent = scheduler.add_neuron(1, Some("latent".to_string()));
    let fix = scheduler.add_neuron(1, Some("fix".to_string()));
    
    scheduler.connect_neurons(&i1, &encoder, Some(1.0));
    scheduler.connect_neurons(&i1, &correction, Some(1.0));
    scheduler.connect_neurons(&i2, &encoder, Some(1.0));
    scheduler.connect_neurons(&i2, &correction, Some(1.0));
    
    scheduler.connect_neurons(&encoder, &latent, Some(1.0));
    scheduler.connect_neurons(&encoder, &fix, Some(-1.0));
    
    scheduler.connect_neurons(&correction, &fix, Some(1.0));
    scheduler.connect_neurons(&fix, &encoder, Some(1.0));

    let a1 = scheduler.prepare_next_layer(HashMap::from([
      (i1.clone(), vec![1.0]),
      (i2.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a1);
    scheduler.print_pool();

    let a2 = scheduler.prepare_next_layer(HashMap::from([
      (i1.clone(), vec![0.0]),
      (i2.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a2);
    scheduler.print_pool();
  }

  #[test]
  fn main() {
  }
}
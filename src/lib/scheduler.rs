use std::{cell::RefCell, result};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::collections::{HashMap, BTreeMap};
use uuid::Uuid;

use crate::neuron::{Neuron, self};

// The time required to transmit a signal from one neuron through action potential 
// to dendrites of the next connected neuron can vary, but it typically ranges from 
// about 1 to 10 milliseconds. 
// In some cases, this process can occur even faster, in microseconds, 
// especially in highly myelinated neurons or in cases of electrical synapses.
// The shortest time between constant activation of sensor neurons in the human eye, 
// such as cones and rods, is known as the flicker fusion threshold, which is about 10-16 ms.

// The refractory period is the time during which a neuron is incapable of generating another action potential.
// Absolute refractory period is the initial phase during which the neuron cannot generate another action potential 
// under any circumstances, typically, it lasts for about 1 to 2 milliseconds. 
// Following the absolute refractory period, there is a relative refractory period 
// during which the neuron can generate another action potential, 
// but only in response to a stronger-than-normal stimulus. Can range from 2 to 4 milliseconds or more.

pub struct Scheduler {
  pool: BTreeMap<String, Arc<Box<Neuron>>>,
  pub time: u64,
//   propagating: bool,
//   scheduled: HashMap<u64, Vec<RefCell<Neuron>>>,
}

impl Scheduler {
  pub fn new() -> Self {
    Scheduler {
      pool: BTreeMap::new(),
      time: 0,
    }
  }

  pub fn add_neuron(&mut self, threshold: i32, name: Option<String>) -> String {
    let neuron = Box::new(Neuron::new(threshold, name));
    let neuron_id = neuron.get_name().clone();
    let new_neuron = Arc::new(neuron);
    self.pool.insert(neuron_id.clone(), new_neuron);
    neuron_id.clone()
  }

  pub fn connect_neurons(&mut self, pre_id: &String, post_id: &String, strength: Option<f64>) {
    let pre = self.find_neuron_by_id_mut(pre_id).unwrap();
    pre.connect_to(post_id.clone(), strength);
  }

  pub fn prepare_next_layer(&mut self, mut activated_neurons: HashMap<String, Vec<f64>>) -> Vec<String> {
    let mut neurons_next_layer: Vec<String> = Vec::new();
    for (neuron_id, signals) in activated_neurons.iter() {
      if let Some(neuron) = self.find_neuron_by_id_mut(&neuron_id) {
        // TODO: name it! it's potential activity or something
        let prev_potential = neuron.potential;
        println!("{} + {:?} ===> {}", prev_potential, signals, neuron_id.clone());
        let result = neuron.process_signals(signals);
        if let Some(potential_diff) = result {
          neuron.update_potential(potential_diff);
          if neuron.potential >= neuron.threshold as f64 {
            neurons_next_layer.push(neuron_id.clone());
          }
        }
      }
    }
    neurons_next_layer
  }

  pub fn send_action_potential(&mut self, activated_neurons: Vec<String>) {
    if activated_neurons.is_empty() {
      println!("activation path is over at {}", self.time);
      return;
    }

    // let mut neurons_next_layer: Vec<Vec<String>> = Vec::new();
    let mut strength_per_neuron: HashMap<String, Vec<f64>> = HashMap::new();
    for neuron_id in activated_neurons {
      if let Some(neuron) = self.find_neuron_by_id_mut(&neuron_id) {
        neuron.potential = 0.0;
        neuron.transmit(&mut strength_per_neuron);
      } else {
        println!("Failed to get neuron '{}'", neuron_id);
      }
    }

    self.time += 1;
    let neurons_next_layer: Vec<String> = self.prepare_next_layer(strength_per_neuron);
    println!(">> time {} <<", self.time);
    self.print_pool();
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
    if let Some(tt) = self.pool.get_mut(neuron_id) {
      return Arc::get_mut(tt);
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
    let latent = scheduler.add_neuron(10, Some("latent".to_string())); // stop propagation on this neuron => big threshold
    let fix = scheduler.add_neuron(1, Some("fix".to_string()));
    
    scheduler.connect_neurons(&i1, &encoder, Some(1.0));
    scheduler.connect_neurons(&i1, &correction, Some(1.0));
    scheduler.connect_neurons(&i2, &encoder, Some(1.0));
    scheduler.connect_neurons(&i2, &correction, Some(1.0));
    
    scheduler.connect_neurons(&encoder, &latent, Some(1.0));
    scheduler.connect_neurons(&encoder, &fix, Some(-1.0));
    
    scheduler.connect_neurons(&correction, &fix, Some(1.0));
    scheduler.connect_neurons(&fix, &encoder, Some(1.0));

    println!("-- signal 1 ({}): (1, 1) --", scheduler.time);
    let a1 = scheduler.prepare_next_layer(HashMap::from([
      (i1.clone(), vec![1.0]),
      (i2.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a1);
    // scheduler.print_pool();

    println!("-- signal 2 ({}): (0, 1) --", scheduler.time);
    let a2 = scheduler.prepare_next_layer(HashMap::from([
      (i1.clone(), vec![1.0]),
      (i2.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a2);
    // scheduler.print_pool();
  }

  // CPG
  #[test]
  fn central_pattern_generator() {
    let mut scheduler = Box::new(Scheduler::new());
    
    let signal = scheduler.add_neuron(1, Some("signal".to_string()));
    let feedback1 = scheduler.add_neuron(1, Some("l1".to_string()));
    let feedback2 = scheduler.add_neuron(1, Some("l2".to_string()));
    
    {
      let drive1 = scheduler.add_neuron(10, Some("d1".to_string()));
      let drive2 = scheduler.add_neuron(10, Some("d2".to_string()));
      let a1 = scheduler.add_neuron(2, Some("a1".to_string()));
      let a2 = scheduler.add_neuron(2, Some("a2".to_string()));
      let c1 = scheduler.add_neuron(1, Some("c1".to_string()));
      let c2 = scheduler.add_neuron(1, Some("c2".to_string()));
      let uv1 = scheduler.add_neuron(1, Some("uv1".to_string()));
      let uv2 = scheduler.add_neuron(1, Some("uv2".to_string()));
      
      scheduler.connect_neurons(&signal, &a1, Some(1.0));
      scheduler.connect_neurons(&signal, &a2, Some(1.0));
      
      scheduler.connect_neurons(&feedback1, &uv1, Some(0.5));
      scheduler.connect_neurons(&feedback1, &c1, Some(0.5));
      scheduler.connect_neurons(&feedback2, &uv2, Some(0.5));
      scheduler.connect_neurons(&feedback2, &c2, Some(0.5));
      
      scheduler.connect_neurons(&uv1, &a1, Some(1.0));
      scheduler.connect_neurons(&uv1, &uv1, Some(-1.0));
      scheduler.connect_neurons(&uv1, &uv2, Some(-0.25));
      scheduler.connect_neurons(&uv1, &c1, Some(0.25));
      scheduler.connect_neurons(&uv1, &c2, Some(-0.25));
      
      scheduler.connect_neurons(&uv2, &a2, Some(1.0));
      scheduler.connect_neurons(&uv2, &uv2, Some(-1.0));
      scheduler.connect_neurons(&uv2, &uv1, Some(-0.25));
      scheduler.connect_neurons(&uv2, &c2, Some(0.25));
      scheduler.connect_neurons(&uv2, &c1, Some(-0.25));

      scheduler.connect_neurons(&a1, &drive1, Some(1.0));
      scheduler.connect_neurons(&a1, &uv1, Some(1.0));
      scheduler.connect_neurons(&a1, &uv2, Some(-0.25));
      scheduler.connect_neurons(&a2, &drive2, Some(1.0));
      scheduler.connect_neurons(&a2, &uv2, Some(1.0));
      scheduler.connect_neurons(&a2, &uv1, Some(-0.25));

      scheduler.connect_neurons(&c1, &c2, Some(-1.0));
      scheduler.connect_neurons(&c1, &uv1, Some(1.0)); // modulatory
      scheduler.connect_neurons(&c1, &a1, Some(1.0)); // modulatory
      scheduler.connect_neurons(&c2, &c1, Some(-1.0));
      scheduler.connect_neurons(&c2, &uv2, Some(1.0)); // modulatory
      scheduler.connect_neurons(&c2, &a2, Some(1.0)); // modulatory
    }

    println!("-- signal 1 ({}) --", scheduler.time);
    let a1 = scheduler.prepare_next_layer(HashMap::from([
      (feedback1.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a1.clone());

    println!("-- signal 2 ({}) --", scheduler.time);
    scheduler.send_action_potential(a1);

    println!("-- signal 2 ({}) --", scheduler.time);
    let a2 = scheduler.prepare_next_layer(HashMap::from([
      (signal.clone(), vec![1.0]),
      (feedback1.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a2);

    println!("-- signal 3 ({}) --", scheduler.time);
    let a3 = scheduler.prepare_next_layer(HashMap::from([
      (signal.clone(), vec![1.0]),
      (feedback1.clone(), vec![1.0]),
    ]));
    scheduler.send_action_potential(a3);

  }

  #[test]
  fn main() {
  }
}
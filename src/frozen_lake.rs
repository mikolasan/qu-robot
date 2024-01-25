use std::cmp::max;
use std::collections::HashMap;
use std::io::{Error, Result};

use ndarray::prelude::*;
use ndarray::{array, Array2};
use rand::Rng;
// use rsrl::{
//   domains::{Domain, Observation, State, Reward, Action}, 
//   spaces::{discrete::Integers, ProductSpace},
// };
use crate::{
  ann::ANN,
  grid_world::{GridWorld, Motion},
};

pub(crate) const ACTIONS: [Motion; 4] = [
  Motion::North(1),
  Motion::South(1),
  Motion::East(1),
  Motion::West(1)
];
  
const STATES: [char; 5] = [
  'S',
  'F',
  'H',
  'G',
  'M',
];

// # actions = rows
// # states = columns
//        North South East West
// Start [                     ]
// Free  [                     ]
// Hole  [                     ]
// Goal  [                     ]

// for mut row in transition_prob.axis_iter_mut(Axis(0)) {
//   let p = 1.0 / (row.len() as f32);
//   for e in row.iter_mut() {
//     *e = p;
//   }
// }

pub type GridPos = [usize; 2];
pub type MetaState = [char; 5];
type PossibleActions = [f32; 4];
type RewardMap = HashMap<char, f32>;

fn near_location(loc: &GridPos, mov: [i32; 2]) -> Result<GridPos> {
  let x = mov[0] + loc[0] as i32;
  if x < 0 {
    return Err(Error::other("x < 0"));
  }
  let y = mov[1] + loc[1] as i32;
  if y < 0 {
    return Err(Error::other("y < 0"));
  }
  let new_loc = [x as usize, y as usize];
  Ok(new_loc)
}

fn make_action_probs_uniform() -> PossibleActions {
  [1.0 / (ACTIONS.len() as f32); 4]
}

pub struct FrozenLake {
  // world: Array2<char>,
  grid_world: GridWorld<char>,
  reward_map: RewardMap,
  transition_prob: HashMap<MetaState, PossibleActions>,
  loc: GridPos,
  step: i32,
  reward: f32,
  exploration_prob: f64,
}

impl FrozenLake {
  pub fn new() -> FrozenLake {
    let mut world: Array2<char> = array![
      ['M', 'M', 'M', 'M', 'M', 'M'],
      ['M', 'F', 'F', 'F', 'F', 'M'],
      ['M', 'F', 'H', 'F', 'H', 'M'],
      ['M', 'F', 'F', 'F', 'H', 'M'],
      ['M', 'H', 'F', 'F', 'G', 'M'],
      ['M', 'M', 'M', 'M', 'M', 'M'],
    ];
    world.swap_axes(0, 1);

    FrozenLake {
      // world: world,
      grid_world: GridWorld::<char>::new(world),
      reward_map: HashMap::from([
        ('S', 0.0),
        ('F', 0.0),
        ('H', -1.0),
        ('G', 1.0),
        ('M', -1.0),
      ]),
      transition_prob: HashMap::new(),
      loc: [1, 1],
      step: 0,
      reward: 0.0,
      exploration_prob: 0.0,
    }
  }

  pub fn loc(&self) -> &GridPos {
    &self.loc
  }

  pub fn loc_mut(&mut self) -> &mut GridPos {
    &mut self.loc
  }

  pub fn grid_world(&self) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for i in 0..self.grid_world.height() {
      let mut row = Vec::new();
      for j in 0..self.grid_world.width() {
        row.push(self.grid_world.get([i, j]).unwrap().to_owned());
      }
      grid.push(row);
    }
    grid
  }

  pub fn probs(&self) -> Vec<Vec<Vec<f32>>> {
    let mut p = Vec::new();
    for i in 1..(self.grid_world.height() - 1) {
      let mut row = Vec::new();
      for j in 1..(self.grid_world.width() - 1) {
        let meta_state = self.create_meta_state(&[i, j]);
        let pp = self.action_prob(&meta_state);
        row.push(pp);
      }
      p.push(row);
    }
    p
  }

  pub fn action_prob(&self, meta_state: &MetaState) -> Vec<f32> {
    if let Some(probs) = self.transition_prob.get(meta_state) {
      probs.to_vec()
    } else {
      make_action_probs_uniform().to_vec()
    }
  }

  pub fn create_meta_state(&self, loc: &GridPos) -> MetaState {
    let mut meta_state: MetaState = ['?'; 5];
    let moves: [[i32; 2]; 5] = [
      [0, 0],
      [-1, 0],
      [1, 0],
      [0, -1],
      [0, 1],
    ];
    for (i, mov) in moves.into_iter().enumerate() {
      if let Ok(new_loc) = near_location(loc, mov) {
  
        let state = self.grid_world.get(new_loc).unwrap();
        let r = self.reward_map.get(state).unwrap().to_owned();
        if r != 0.0 {
          meta_state[i] = state.to_owned();
        }
      } else {
        // TODO: need to avoid this situation (don't go into walls, for example)
        let state = self.grid_world.get(*loc).unwrap();
        let r = self.reward_map.get(state).unwrap().to_owned();
        if r != 0.0 {
          meta_state[i] = state.to_owned();
        }
      }
    }
    return meta_state;
  }

  pub fn update_exploration_prob(&mut self) {
    let min_exploration_prob: f64 = 0.1;
    let exploration_decay: f64 = 0.0001;
    let new_decay = f64::powf(std::f64::consts::E, -exploration_decay * (self.step as f64));
    self.exploration_prob = min_exploration_prob.max(new_decay); // but not lower than `min_exploration_prob`
    println!("set new exploration prob {}", self.exploration_prob);
  }

  pub fn next_action_max(&mut self, state: &MetaState) -> Option<usize> {
    let test_probs = self.transition_prob.get(state);
    if test_probs.is_none() {
      self.transition_prob.insert(*state, make_action_probs_uniform());
    }
    let state_probs = self.transition_prob.get(state).unwrap();
    state_probs
      .iter()
      .enumerate()
      .max_by(|(_, a), (_, b)| a.total_cmp(b))
      .map(|(index, _)| index)
  }

  pub fn find_q_max(&mut self, next_state: &MetaState) -> f32 {
    let next_action_id = self.next_action_max(next_state).unwrap();
    let next_state_probs = self.transition_prob.get(next_state).unwrap().clone();
    let max_prob = next_state_probs[next_action_id];
    assert!(max_prob < 1.1, "why?? {} {:?} {} {:?}", max_prob, next_state, next_action_id, next_state_probs);
    max_prob
  }

  pub fn find_next_action(&mut self, state: &MetaState) -> Option<usize> {
    let mut action_id = None;
    let test_probs = self.transition_prob.get(state);
    if test_probs.is_none() {
      self.transition_prob.insert(*state, make_action_probs_uniform());
    }
    
    let mut rng = rand::thread_rng();
    let p: f64 = rng.gen::<f64>();
    if p < self.exploration_prob {
      let mut cumulative_prob = 0.0;
      let p: f32 = rng.gen::<f32>();
      let mut state_probs = self.transition_prob.get_mut(state).unwrap();
      for (i, &weight) in state_probs.iter().enumerate() {
        cumulative_prob += weight;
        if p <= cumulative_prob {
          action_id = Some(i);
          println!("random exploration: {}", i);
          break;
        }
      }
    } else {
      action_id = self.next_action_max(state);
      println!("exploitation: {}", action_id.unwrap());
    }
    action_id
  }

  pub fn make_action(&self, loc: &GridPos, action_id: usize) -> (&char, GridPos) {
    let action = ACTIONS[action_id];
    let mut next_loc = self.grid_world.perform_motion(&loc, action);

    let state_sym: &char = self.grid_world.get(next_loc).unwrap();
    if state_sym == &'M' {
      println!("This move is not allowed (-> W), keeping last position");
      next_loc = *loc;
    }

    (state_sym, next_loc)
  }

  pub fn update_probs(
    &mut self,
    state: &MetaState,
    action_id: usize,
    reward: f32,
    next_state: &MetaState,
    next_action_id: usize,
  ) {
    let discount: f32 = 0.99;
    let learning_rate: f32 = 0.1;

    // if step_reward != 0 {
    // } else {
    //   println!("no reward = no prob updates");
    // }

    // belllman optimality equation
    let next_state_probs = self.transition_prob.get(next_state).unwrap().clone();
    let state_probs = self.transition_prob.get_mut(state).unwrap();
    let predict = state_probs[action_id];
    let max_prob = next_state_probs[next_action_id];
    let target = reward + discount * max_prob;
    // let new_q = (1.0 - learning_rate) * triggered_prob + learning_rate * ((step_reward as f32) + discount * max_prob);
    
    let new_q = predict + learning_rate * (target - predict);
    state_probs[action_id] = new_q.max(0.0);

    // normalize
    // let total = state_probs.iter().fold(0.0, |sum, val| sum + val.abs());
    let total = state_probs.iter().sum::<f32>();
    for e in state_probs.iter_mut() {
      *e = *e / total;
    }

    // let total_afterwards = state_probs.iter().sum::<f32>();
    // assert!(total_afterwards <= 1.1, "it's not normalized {} = {:?}", total_afterwards, state_probs);

    // my version
    // let mut delta = learning_rate * (step_reward as f32) * discount;
    // let mut delta_rem = delta / ((ACTIONS.len() - 1) as f32);
    // let mut new_q = triggered_prob + delta;
    // if new_q > 1.0 {
    //   new_q = 1.0;
    //   delta = 1.0 - triggered_prob;
    //   delta_rem = delta / ((ACTIONS.len() - 1) as f32);
    // }
    // for (i, e) in state_probs.iter_mut().enumerate() {
    //   if i == next_action_id.unwrap() {
    //     *e += delta;
    //   } else {
    //     *e -= delta_rem;
    //   }
    // }
  }

  // returns: state, action, reward, next_state
  pub fn step(&mut self) -> (MetaState, usize, f32, MetaState) {
    self.update_exploration_prob();

    let state = self.create_meta_state(self.loc());
    let action_id = self.find_next_action(&state).expect("failed exploration");
    
    let (next_state_sym, next_loc) = self.make_action(self.loc(), action_id);
    let reward = self.reward_map.get(next_state_sym).unwrap().to_owned();
    let next_state = self.create_meta_state(&next_loc);
    let step_result = (state, action_id, reward, next_state);
    let next_action_id = self.next_action_max(&next_state).expect("failed next max action");
    
    self.loc = next_loc;
    self.reward += reward;
    self.step += 1;
    self.update_probs(&state, action_id, reward, &next_state, next_action_id);
    
    step_result
  }

  pub fn train(&mut self) {
    // let mut network = ANN::new([15, 20, 4]);
    // let epochs = 3000;
    // let inputs: Array2<f32> = arr2(&[
    //   [0.0, 0.0],
    //   [0.0, 1.0],
    //   [1.0, 0.0],
    //   [1.0, 1.0]
    // ]);
    // let outputs: Array2<f32> = arr1(&[0.0, 1.0, 1.0, 0.0]);
    // network.train(epochs, inputs, outputs);
    // network.print();
  }

}

// TODO: insttall openblas
// https://github.com/blas-lapack-rs/openblas-src
// https://github.com/Microsoft/vcpkg

// impl Domain for FrozenLake {
//   type StateSpace = ProductSpace<Integers>;
//   type ActionSpace = Integers;

//   fn state_space(&self) -> Self::StateSpace {
//     todo!()
//   }

//   fn action_space(&self) -> Self::ActionSpace {
//     todo!()
//   }

//   fn emit(&self) -> Observation<State<Self>> {
//     todo!()
//   }

//   fn step(&mut self, _a: &Action<Self>) -> (Observation<State<Self>>, Reward) {
//     todo!()
//   }
// }

#[cfg(test)]
mod tests {
  use ndarray::{Array1, Array2};
  use super::{FrozenLake, MetaState, ACTIONS, STATES};
  
  // meta state -> one hot encoding
  fn action_one_hot_encoding(action: usize) -> Vec<f32> {
    let mut action_encoded = vec![0.0; ACTIONS.len()];
    if action < ACTIONS.len() {
      action_encoded[action] = 1.0;
    }
    action_encoded
  }

  // meta state -> one hot encoding
  fn meta_state_one_hot_encoding(meta_state: &[char; 5]) -> Vec<f32> {
    let state_ids = meta_state.map(
      |c| STATES.iter().position(|&r| r == c).unwrap_or(STATES.len())
    );
    let mut code: Vec<f32> = Vec::new();
    for state_id in state_ids.iter() {
      let mut state_encoded = vec![0.0; STATES.len()];
      if *state_id < STATES.len() {
        state_encoded[*state_id] = 1.0;
      }
      code.append(&mut state_encoded);
    }
    code
  }

  fn encode(
    state: &MetaState, 
    action: usize, 
    reward: f32, 
    next_state: &MetaState) -> Array1<f32>
  {
    let mut transition: Vec<f32> = Vec::new();
    transition.append(&mut meta_state_one_hot_encoding(&state));
    transition.append(&mut action_one_hot_encoding(action));
    transition.push(reward);
    transition.append(&mut meta_state_one_hot_encoding(&next_state));

    Array1::from_vec(transition)
  }

  #[test]
  fn train_rl() {
    let mut env = FrozenLake::new();

    let mut replay: Vec<Array1<f32>> = Vec::new();
    let mut outputs: Vec<f32> = Vec::new();
    // make a batch
    // env.train();
    let max_iter = 50;
    for i in 0..max_iter {
      'steps: loop {
        let (state, action, reward, next_state) = env.step();
        println!("step {:?} [{}] ({}) -> {:?}", 
          state, action, reward, next_state);
        let transition = encode(&state, action, reward, &next_state);
        println!("transition {:?}", transition); // 55 length
        replay.push(transition);
        
        match next_state[0] {
          'G' => {
            outputs.push(reward);
            println!("output {:?}", outputs.last().unwrap());
            break 'steps;
          },
          'H' => {
            outputs.push(reward);
            println!("output {:?}", outputs.last().unwrap());
            break 'steps;
          },
          _ => {
            let discount: f32 = 0.99;
            let max_q = env.find_q_max(&next_state);
            let output = reward + discount * max_q;
            // if output > 1.0 {
            //   println!("max_q {}", max_q);
            // }
            outputs.push(output);
            println!("output {:?}", outputs.last().unwrap());
          }
        }
        
      }
      // reset location
      let loc = env.loc_mut();
      loc[0] = 1;
      loc[1] = 1;
    }

    // replay -> batch


  }
}
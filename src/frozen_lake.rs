use std::collections::HashMap;
use std::io::{Error, Result};

use ndarray::{array, Array2};
use rand::Rng;
use rsrl::{
  domains::{Domain, Observation, State, Reward, Action}, 
  spaces::{discrete::Integers, ProductSpace},
};
use crate::grid_world::{GridWorld, Motion};

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
  'W',
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
type MetaState = [char; 5];
type PossibleActions = [f32; 4];
type RewardMap = HashMap<char, i32>;

fn near_location(loc: GridPos, mov: [i32; 2]) -> Result<GridPos> {
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

pub struct FrozenLake {
  // world: Array2<char>,
  grid_world: GridWorld<char>,
  reward_map: RewardMap,
  transition_prob: HashMap<MetaState, PossibleActions>,
  loc: GridPos,
  reward: i32,
}

impl FrozenLake {
  pub fn new() -> FrozenLake {
    let mut world: Array2<char> = array![
      ['W', 'W', 'W', 'W', 'W', 'W'],
      ['W', 'S', 'F', 'F', 'F', 'W'],
      ['W', 'F', 'H', 'F', 'H', 'W'],
      ['W', 'F', 'F', 'F', 'H', 'W'],
      ['W', 'H', 'F', 'F', 'G', 'W'],
      ['W', 'W', 'W', 'W', 'W', 'W'],
    ];
    world.swap_axes(0, 1);

    FrozenLake {
      // world: world,
      grid_world: GridWorld::<char>::new(world),
      reward_map: HashMap::from([
        ('S', 0),
        ('F', 0),
        ('H', -10),
        ('G', 10),
        ('W', -1),
      ]),
      transition_prob: HashMap::new(),
      loc: [1, 1],
      reward: 0
    }
  }

  pub fn loc(&self) -> GridPos {
    self.loc
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

  pub fn action_prob(&self, meta_state: &MetaState) -> Vec<f32> {
    if let Some(probs) = self.transition_prob.get(meta_state) {
      probs.to_vec()
    } else {
      Vec::new()
    }
  }

  pub fn create_meta_state( &self, loc: GridPos) -> MetaState {
    let mut meta_state: MetaState = ['_'; 5];
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
        if r != 0 {
          meta_state[i] = state.to_owned();
        }
      } else {
        // TODO: need to avoid this situation (don't go into walls, for example)
        let state = self.grid_world.get(loc).unwrap();
        let r = self.reward_map.get(state).unwrap().to_owned();
        if r != 0 {
          meta_state[i] = state.to_owned();
        }
      }
    }
    return meta_state;
  }

  pub fn step(&mut self) -> (char, i32) {
    let meta_state = self.create_meta_state(self.loc);
      
    // find id for the state
    // let state_id = states.iter().position(|&c| c == current_state).unwrap();
    let mut rng = rand::thread_rng();

    let mut next_action_id = None;
    if let Some(state_probs) = self.transition_prob.get(&meta_state) {
      let p: f32 = rng.gen();
      let mut cumulative_prob = 0.0;
      for (i, &weight) in state_probs.iter().enumerate() {
        cumulative_prob += weight;
        if p <= cumulative_prob {
          next_action_id = Some(i);
          break;
        }
      }
    } else {
      let p = 1.0 / (ACTIONS.len() as f32);
      let mut even_spread: [f32; 4] = [p; 4];
      // for e in even_spread.iter_mut() {
      //   *e = p;
      // }
      self.transition_prob.insert(meta_state, even_spread);
      next_action_id = Some(rng.gen_range(0..ACTIONS.len()));
    }
    match next_action_id {
      Some(action) => {
        println!("Randomly selected action: {}", action);
      }
      None => {
        println!("No action selected. Check your weights.");
      }
    }
    let random_action = ACTIONS[next_action_id.unwrap()];

    let new_loc = self.grid_world.perform_motion(self.loc, random_action);
    let state: &char = self.grid_world.get(new_loc).unwrap();
    if state == &'W' {
      println!("This move is not allowed (-> W), keeping last position");
    } else {
      self.loc = new_loc;
    }
    let step_reward = self.reward_map.get(state).unwrap().to_owned();
    if step_reward != 0 {
      if let Some(state_probs) = self.transition_prob.get_mut(&meta_state) {
        let triggered_prob = state_probs[next_action_id.unwrap()];  
        let delta = (step_reward as f32) * triggered_prob / 2.0;
        let delta_rem = delta / ((ACTIONS.len() - 1) as f32);
        for (i, e) in state_probs.iter_mut().enumerate() {
          if i == next_action_id.unwrap() {
            *e += delta;
          } else {
            *e -= delta_rem;
          }
        }
      } else {
        println!("meta state disappeared :O");
      }
    } else {
      println!("no reward = no prob updates");
    }
    self.reward += step_reward;
    (state.to_owned(), step_reward)
  }
}



impl Domain for FrozenLake {
  type StateSpace = ProductSpace<Integers>;
  type ActionSpace = Integers;

  fn state_space(&self) -> Self::StateSpace {
    todo!()
  }

  fn action_space(&self) -> Self::ActionSpace {
    todo!()
  }

  fn emit(&self) -> Observation<State<Self>> {
    todo!()
  }

  fn step(&mut self, _a: &Action<Self>) -> (Observation<State<Self>>, Reward) {
    todo!()
  }
}
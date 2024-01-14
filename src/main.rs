use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::Hash;
use std::io::{Error, Result};
use rand::Rng;
use plotters::{prelude::*, data};
use ndarray::{array, Array2, Axis};
use gtk::{prelude::*, gio};
use gtk::{glib, Application, ApplicationWindow, Button, Image};

mod grid_world;
mod window;
use grid_world::Motion;
use crate::{
  grid_world::GridWorld,
  window::Window,
};
// mod frozen_lake;
// use crate::frozen_lake::FrozenLake;

fn vec2array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

const APP_ID: &str = "xyz.k_robot_lab.QuRobot";
const PLOT_FILE: &str = "reward_vs_steps.png";
const RESOURCES_FILE: &str = "compiled.gresource";

fn build_ui(app: &Application) {
  // Create new window and present it
  let window = Window::new(app);
  window.present();

  // let image = Image::from_file(PLOT_FILE);


  // Connect to "clicked" signal of `button`
  // button.connect_clicked(|button| {
  //   // Set the label to "Hello World!" after the button has been clicked on
  //   button.set_label("Hello World!");
  // });

  // Create a window and set the title
  // let window = ApplicationWindow::builder()
  //     .application(app)
  //     .title("Qu Robot Widget")
  //     .child(&image)
  //     .child(&button)
  //     .build();

  // // Present window
  // window.present();
}

// Function to plot the graph using plotters
fn plot_graph(
  data_points_bad: Vec<(i32, i32, i32)>,
  data_points_good: Vec<(i32, i32, i32)>,
  max_x: i32,
  max_y: i32
) {
  // Create a drawing area with a Cartesian coordinate system
  let root = BitMapBackend::new(
    PLOT_FILE, (800, 600))
    .into_drawing_area();
  root.fill(&WHITE).unwrap();

  // Create a chart context
  let mut chart = ChartBuilder::on(&root)
      .caption("Reward vs Steps", ("sans-serif", 20).into_font())
      .margin(5)
      .x_label_area_size(40)
      .y_label_area_size(40)
      .build_cartesian_2d(0..max_x, 0..max_y)
      .unwrap();

  // Draw the line plot
  chart
      .configure_mesh()
      .y_desc("Steps")
      .x_desc("Iteration")
      .draw()
      .unwrap();

  chart.draw_series(data_points_bad.iter()
    .map(
      |(i, s, _)| Circle::new((i.to_owned(), s.to_owned()), 3, BLACK.filled())
    )).unwrap();
  chart.draw_series(data_points_good.iter()
    .map(
      |(i, s, _)| Circle::new((i.to_owned(), s.to_owned()), 3, BLUE.filled())
    )).unwrap();
}

const ACTIONS: [Motion; 4] = [
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

struct StateEncoder {
  states: Vec<char>,
  encoding_map: HashMap<char, i8>,
}

impl StateEncoder {
    pub fn new(&mut self) {
      self.states = STATES.to_vec();
    }

    pub fn encode(&self, state: char) -> i8 {
      self.encoding_map[&state]
    }
}

type GridPos = [usize; 2];
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

fn create_meta_state(
  loc: GridPos, 
  gw: &GridWorld<char>, 
  reward_map: &RewardMap, 
  default_value: char) 
-> MetaState {
  let mut meta_state: MetaState = [default_value; 5];
  let moves: [[i32; 2]; 5] = [
    [0, 0],
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
  ];
  for (i, mov) in moves.into_iter().enumerate() {
    if let Ok(new_loc) = near_location(loc, mov) {

      let state = gw.get(new_loc).unwrap();
      let r = reward_map.get(state).unwrap().to_owned();
      if r != 0 {
        meta_state[i] = state.to_owned();
      }
    } else {
      // TODO: need to avoid this situation (don't go into walls, for example)
      let state = gw.get(loc).unwrap();
      let r = reward_map.get(state).unwrap().to_owned();
      if r != 0 {
        meta_state[i] = state.to_owned();
      }
    }
  }
  return meta_state;
}

// fn create_meta_state<T>(loc: [usize; 2], gw: &GridWorld<T>, reward_map: &HashMap<T, i32>, default_value: T) -> [T; 5] 
// where T: Hash + Copy, T: std::cmp::Eq {
//   let mut meta_state: [T; 5] = [default_value; 5];
//   let moves: [[i32; 2]; 5] = [
//     [0, 0],
//     [-1, 0],
//     [1, 0],
//     [0, -1],
//     [0, 1],
//   ];
//   for (i, mov) in moves.into_iter().enumerate() {
//     let new_loc = [(mov[0] + loc[0] as i32) as usize, (mov[1] + loc[1] as i32) as usize];
//     let state = gw.get(new_loc).unwrap();
//     let r = reward_map.get(state).unwrap().to_owned();
//     if r != 0 {
//       meta_state[i] = state.to_owned();
//     }
//   }
//   return meta_state;
// }

fn simulate() {
  let mut world: Array2<char> = array![
    ['W', 'W', 'W', 'W', 'W', 'W'],
    ['W', 'S', 'F', 'F', 'F', 'W'],
    ['W', 'F', 'H', 'F', 'H', 'W'],
    ['W', 'F', 'F', 'F', 'H', 'W'],
    ['W', 'H', 'F', 'F', 'G', 'W'],
    ['W', 'W', 'W', 'W', 'W', 'W'],
  ];
  world.swap_axes(0, 1);
  let gw = GridWorld::<char>::new(world);

  let reward_map: RewardMap = HashMap::from([
    ('S', 0),
    ('F', 0),
    ('H', -10),
    ('G', 10),
    ('W', -1),
  ]);

  let mut transition_prob: HashMap<MetaState, PossibleActions> = HashMap::new();
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
  
  let mut rng = rand::thread_rng();
  let max_iter = 50;
  let mut data_points_bad = Vec::new();
  let mut data_points_good = Vec::new();
  let mut max_steps = 0;

  for i in 0..max_iter {
    println!("--- {} ---", i);
    let mut loc: GridPos = [1, 1];
    let mut step = 0;
    let mut reward = 0;
    'steps: loop {
      step += 1;

      // let random_action = actions[p];
      // let current_state = gw.get(loc).unwrap().to_owned();
      let meta_state = create_meta_state(loc, &gw, &reward_map, '_');
      
      // find id for the state
      // let state_id = states.iter().position(|&c| c == current_state).unwrap();
      
      let mut next_action_id = None;
      if let Some(state_probs) = transition_prob.get(&meta_state) {
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
        transition_prob.insert(meta_state, even_spread);
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

      let new_loc = gw.perform_motion(loc, random_action);
      let state: &char = gw.get(new_loc).unwrap();
      if state == &'W' {
        println!("This move is not allowed (-> W), keeping last position");
      } else {
        loc = new_loc;
      }
      let step_reward = reward_map.get(state).unwrap().to_owned();
      if step_reward != 0 {
        if let Some(state_probs) = transition_prob.get_mut(&meta_state) {
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
      reward += step_reward;
      
      println!("step ({}) {} {} = {} | {} ... {}", 
        step, loc[0], loc[1], state, step_reward, reward);
      
      match state {
        'G' => {
          println!("GOAL!");
          data_points_good.push((i, step, reward));
          break 'steps;
        },
        'H' => {
          println!(">> HOLE <<");
          data_points_bad.push((i, step, reward));
          break 'steps;
        },
        _ => {}
      }
    }
    if step > max_steps {
      max_steps = step;
    }
  }

  plot_graph(data_points_bad, data_points_good, max_iter, max_steps);

}

fn main() -> glib::ExitCode {

  // Cargo won't pass OUT_DIR to the compiler unless there is a build script defined in Cargo.toml
  gio::resources_register_include!("compiled.gresource")
    .expect("Failed to register resources.");

  // gtk::glib::set_application_name("Qu Robot");
  // gtk::init().expect("Unable to start GTK4");

  // let res = gio::Resource::load(RESOURCES_FILE)
  //   .expect("Could not load gresource file");
  // gio::resources_register(&res);

  simulate();

  // Create a new application
  let app = Application::builder().application_id(APP_ID).build();

  // Connect to "activate" signal of `app`
  app.connect_activate(build_ui);

  // Run the application
  app.run()
}
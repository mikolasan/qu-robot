use std::collections::HashMap;
use std::convert::TryInto;
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
fn plot_graph(data_points: Vec<(i32, i32)>) {
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
      .build_cartesian_2d(0..data_points.len(), 0..200)
      .unwrap();

  // Draw the line plot
  chart
      .configure_mesh()
      .y_desc("Steps")
      .x_desc("Iteration")
      .draw()
      .unwrap();

  // let arr = vec2array(data_points);
  let series = (0..)
    .zip(data_points.iter())
    .map(
      |(x, y)| Circle::new((x, y.0), 3, BLUE.filled())
    )
    ;

    // (0..).zip(data.iter()).map(|(x, y)| (x, *y))

  // let line_series = LineSeries::new(
  //   (-100..100).
  //     .map(
  //       |i| (i, i)
  //     ), &BLUE
  //   );
  // chart.draw_series(line_series).unwrap();
  chart.draw_series(series).unwrap();
}

fn simulate() {
  // actions
  let actions = vec! [
    Motion::North(1),
    Motion::South(1),
    Motion::East(1),
    Motion::West(1)
  ];
  // let mut states: Vec<char> = Vec::new();
  let states = vec! [
    'S',
    'F',
    'H',
    'G',
  ];
  let mut world: Array2<char> = array![
    ['S', 'F', 'F', 'F'],
    ['F', 'H', 'F', 'H'],
    ['F', 'F', 'F', 'H'],
    ['H', 'F', 'F', 'G'],
  ];
  world.swap_axes(0, 1);
  let gw = GridWorld::<char>::new(world);

  let reward_map = HashMap::from([
    ('S', 0),
    ('F', 0),
    ('H', -1),
    ('G', 1),
  ]);

  let mut transition_prob = Array2::<f32>::zeros((states.len(), actions.len()));
  // # actions = rows
  // # states = columns
  //        North South East West
  // Start [                     ]
  // Free  [                     ]
  // Hole  [                     ]
  // Goal  [                     ]
  for mut row in transition_prob.axis_iter_mut(Axis(0)) {
    let p = 1.0 / (row.len() as f32);
    for e in row.iter_mut() {
      *e = p;
    }
  }
  // for row in transition_prob.rows() {
  //   let p = 1.0 / (actions.len() as f32);
  //   for e in row {
  //     e = p;
  //   }
  // }

  let mut rng = rand::thread_rng();
  // let final_state = 'G';
  let max_iter = 20;
  let mut data_points = Vec::new();

  for i in 0..max_iter {
    println!("--- {} ---", i);
    let mut loc = [0, 0];
    let mut step = 0;
    let mut reward = 0;
    'steps: loop {
      step += 1;

      let p: f32 = rng.gen();
      // let random_action = actions[p];
      let current_state = gw.get(loc).unwrap().clone();
      let state_id = states.iter().position(|&c| c == current_state).unwrap();
      let mut cumulative_prob = 0.0;
      let mut next_action_id = None;

      for (i, &weight) in transition_prob.row(state_id).iter().enumerate() {
        cumulative_prob += weight;
        if p <= cumulative_prob {
          next_action_id = Some(i);
          break;
        }
      }

      match next_action_id {
        Some(action) => {
          println!("Randomly selected action: {}", action);
        }
        None => {
          println!("No action selected. Check your weights.");
        }
      }
      let random_action = actions[next_action_id.unwrap()];

      loc = gw.perform_motion(loc, random_action);
      let state: &char = gw.get(loc).unwrap();
      
      let step_reward = reward_map.get(state).unwrap().clone();
      if step_reward != 0 {
        let triggered_prob = transition_prob.get((state_id, next_action_id.unwrap())).unwrap();
        let delta = (step_reward as f32) * triggered_prob / 2.0;
        let delta_rem = delta / ((actions.len() - 1) as f32);
        let mut row = transition_prob.row_mut(state_id);
        for (i, e) in row.indexed_iter_mut() {
          if i == next_action_id.unwrap() {
            *e += delta;
          } else {
            *e -= delta_rem;
          }
        }

      }
      reward += step_reward;
      
      println!("step ({}) {} {} = {} | {} ... {}", 
        step, loc[0], loc[1], state, step_reward, reward);
      
      match state {
        'G' => {
          println!("GOAL!");
          break 'steps;
        },
        'H' => {
          println!(">> HOLE <<");
          break 'steps;
        },
        _ => {}
      }
    }
    data_points.push((step, reward));

  }

  plot_graph(data_points);

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
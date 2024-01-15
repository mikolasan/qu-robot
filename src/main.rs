use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::Hash;
use std::io::{Error, Result};
use std::sync::Mutex;

use frozen_lake::GridPos;
use rand::Rng;
use plotters::{prelude::*, data};
use ndarray::{array, Array2, Axis};
use gtk::{prelude::*, gio};
use gtk::{glib, Application, ApplicationWindow, Button, Image};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

mod frozen_lake;
mod grid_world;
mod window;

use crate::{
  frozen_lake::{ACTIONS, FrozenLake},
  grid_world::GridWorld,
  window::Window,
};

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
  let mut env = FrozenLake::new();
    
  let max_iter = 50;
  let mut data_points_bad = Vec::new();
  let mut data_points_good = Vec::new();
  let mut max_steps = 0;

  for i in 0..max_iter {
    println!("--- {} ---", i);
    let mut step = 0;
    let mut reward = 0;
    'steps: loop {
      step += 1;

      let (state, step_reward) = env.step();
      reward += step_reward;
      // let random_action = actions[p];
      // let current_state = gw.get(loc).unwrap().to_owned();
      println!("step ({}) {} {} | {} ... {}", 
        step, env.loc()[0], env.loc()[1], step_reward, reward);
      
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

fn ui_main() -> glib::ExitCode {

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

struct AppState {
  env: Mutex<FrozenLake>,
  step: Mutex<i32>,
  reward: Mutex<i32>,
}

// Define a struct to represent your data
#[derive(Serialize, Deserialize)]
struct EnvData {
  prev_loc: GridPos,
  loc: GridPos,
  grid_world: Vec<Vec<char>>,
  total_reward: i32,
  step_reward: i32,
  action_prob: Vec<f32>,
  meta_state: Vec<char>,
}

async fn env_get_handler(data: web::Data<AppState>) -> impl Responder {
  let env = data.env.lock().unwrap();
  let reward = data.reward.lock().unwrap();

  let loc = env.loc();
  let meta_state = env.create_meta_state(loc);
  let env_data = EnvData {
    prev_loc: loc,
    loc: loc,
    grid_world: env.grid_world(),
    total_reward: *reward,
    step_reward: 0,
    action_prob: env.action_prob(&meta_state),
    meta_state: meta_state.to_vec(),
  };
  let json_response = serde_json::to_string(&env_data)
    .expect("Failed to serialize JSON");
  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_response)
}

async fn step_post_handler(data: web::Data<AppState>) -> impl Responder {
  let mut env = data.env.lock().unwrap();
  let mut step = data.step.lock().unwrap();
  let mut reward = data.reward.lock().unwrap();

  let prev_loc = env.loc();
  let (_, step_reward) = env.step();
  *step += 1;
  *reward += step_reward;
  let loc = env.loc();
  let meta_state = env.create_meta_state(loc);

  let env_data = EnvData {
    prev_loc: prev_loc,
    loc: loc,
    grid_world: env.grid_world(),
    total_reward: *reward,
    step_reward: step_reward,
    action_prob: env.action_prob(&meta_state),
    meta_state: meta_state.to_vec(),
  };
  let json_response = serde_json::to_string(&env_data)
    .expect("Failed to serialize JSON");
  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let env = web::Data::new(AppState {
    env: Mutex::new(FrozenLake::new()),
    step: Mutex::new(0),
    reward: Mutex::new(0),
  });
  // Start the web server
  HttpServer::new(move || {
    App::new()
      .app_data(env.clone())
      .route("/env", web::get().to(env_get_handler))
      .route("/step", web::post().to(step_post_handler))
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

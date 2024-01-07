use std::collections::HashMap;
use rand::Rng;
use plotters::prelude::*;
use ndarray::{array, Array2};
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
      .build_cartesian_2d(0..10, -1..1)
      .unwrap();

  // Draw the line plot
  chart
      .configure_mesh()
      .y_desc("Reward")
      .x_desc("Steps")
      .draw()
      .unwrap();

  chart.draw_series(LineSeries::new(data_points, &RED)).unwrap();
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

  let mut rng = rand::thread_rng();
  // let final_state = 'G';
  let max_iter = 1;
  let mut data_points = Vec::new();

  for i in 0..max_iter {
    println!("--- {} ---", i);
    let mut loc = [0, 0];
    let mut step = 0;
    let mut reward = 0;
    'steps: loop {
      step += 1;

      let p = rng.gen_range(0..actions.len());
      let random_action = actions[p];
      loc = gw.perform_motion(loc, random_action);
      let state: &char = gw.get(loc).unwrap();
      
      let step_reward = reward_map.get(state).unwrap();
      data_points.push((step, step_reward.clone()));
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
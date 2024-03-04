use gtk::{prelude::*, gio};
use gtk::{glib, Application, ApplicationWindow, Button, Image};

use crate::{
  window::Window,
};

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

fn ui_main() -> glib::ExitCode {

  // Cargo won't pass OUT_DIR to the compiler unless there is a build script defined in Cargo.toml
  gio::resources_register_include!("compiled.gresource")
    .expect("Failed to register resources.");

  // gtk::glib::set_application_name("Qu Robot");
  // gtk::init().expect("Unable to start GTK4");

  // let res = gio::Resource::load(RESOURCES_FILE)
  //   .expect("Could not load gresource file");
  // gio::resources_register(&res);

  // simulate();

  // Create a new application
  let app = Application::builder().application_id(APP_ID).build();

  // Connect to "activate" signal of `app`
  app.connect_activate(build_ui);

  // Run the application
  app.run()
}
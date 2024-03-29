use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::Hash;
use std::io::{Error, Result};
use std::sync::Mutex;

use frozen_lake::GridPos;
use rand::Rng;
use plotters::{prelude::*, data};
use ndarray::{array, Array2, Axis};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

mod ann;
mod frozen_lake;
mod grid_world;
// mod window;
mod neuron;
mod scheduler;
mod gl;

use crate::{
  frozen_lake::{ACTIONS, FrozenLake},
  grid_world::GridWorld,
};

fn vec2array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn simulate() {
  let mut env = FrozenLake::new();
    
  let max_iter = 50;
  let mut data_points_bad = Vec::new();
  let mut data_points_good = Vec::new();
  let mut max_steps = 0;

  for i in 0..max_iter {
    println!("--- {} ---", i);
    let mut step = 0;
    let mut reward = 0.0;
    'steps: loop {
      step += 1;

      let (prev_state, action, step_reward, state) = env.step();
      reward += step_reward;
      // let random_action = actions[p];
      // let current_state = gw.get(loc).unwrap().to_owned();
      println!("step ({}) {} {} | {} ... {}", 
        step, env.loc()[0], env.loc()[1], step_reward, reward);
      
      match state[0] {
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

  // plot_graph(data_points_bad, data_points_good, max_iter, max_steps);

}

struct AppState {
  env: Mutex<FrozenLake>,
  step: Mutex<i32>,
  reward: Mutex<f32>,
}

// Define a struct to represent your data
#[derive(Serialize, Deserialize)]
struct EnvData {
  loc: GridPos,
  grid_world: Vec<Vec<char>>,
  probs: Vec<Vec<Vec<f32>>>,
  total_reward: f32,
  step_reward: f32,
  action_prob: Vec<f32>,
  meta_state: Vec<char>,
}

async fn env_get_handler(data: web::Data<AppState>) -> impl Responder {
  let env = data.env.lock().unwrap();
  let reward = data.reward.lock().unwrap();

  let loc = env.loc();
  let meta_state = env.create_meta_state(loc);
  let env_data = EnvData {
    loc: *loc,
    grid_world: env.grid_world(),
    probs: env.probs(),
    total_reward: *reward,
    step_reward: 0.0,
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
  let prev_meta_state = env.create_meta_state(prev_loc);

  let (_, _, step_reward, _) = env.step();
  
  *step += 1;
  *reward += step_reward;
  let loc = env.loc();
  let meta_state = env.create_meta_state(loc);

  let env_data = EnvData {
    loc: *loc,
    grid_world: env.grid_world(),
    probs: env.probs(),
    total_reward: *reward,
    step_reward: step_reward,
    action_prob: env.action_prob(&prev_meta_state),
    meta_state: meta_state.to_vec(),
  };
  let json_response = serde_json::to_string(&env_data)
    .expect("Failed to serialize JSON");
  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_response)
}

#[derive(Deserialize)]
struct MoveData {
  x: usize,
  y: usize,
}

async fn move_post_handler(data: web::Data<AppState>, move_data: web::Json<MoveData>) -> impl Responder {
  let mut env = data.env.lock().unwrap();
  let loc = env.loc_mut();
  loc[0] = move_data.x;
  loc[1] = move_data.y;
  HttpResponse::Ok()
    .content_type("application/json")
    .body("GOOD")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let env = web::Data::new(AppState {
    env: Mutex::new(FrozenLake::new()),
    step: Mutex::new(0),
    reward: Mutex::new(0.0),
  });
  // Start the web server
  HttpServer::new(move || {
    App::new()
      .app_data(env.clone())
      .route("/env", web::get().to(env_get_handler))
      .route("/step", web::post().to(step_post_handler))
      .route("/move", web::post().to(move_post_handler))
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

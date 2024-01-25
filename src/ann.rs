use ndarray::prelude::*;
use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::StandardNormal;

fn sigmoid(x: f32) -> f32 {
  1.0 / (1.0 + (-x).exp())
}

fn d_sigmoid(x: f32) -> f32 {
  x * (1.0 - x)
}

fn relu(x: f32) -> f32 {
  x.max(0.0)
}

fn d_relu(x: f32) -> f32 {
  if x > 0.0 {
    1.0
  } else {
    0.1
  }
}

pub struct ANN {
  epoch: i64,
  learning_rate: f32,
  weights_1: Array2<f32>,
  weights_2: Array2<f32>,
  bias_1: Array1<f32>,
  bias_2: Array1<f32>,
}

impl ANN {
  pub fn new(n_neurons_per_layer: [usize; 3]) -> ANN {
    let w1 = Array2::random((n_neurons_per_layer[0], n_neurons_per_layer[1]), StandardNormal) * 0.1;
    let mut w2 = Array2::random((n_neurons_per_layer[1], n_neurons_per_layer[2]), StandardNormal) * 0.1;
    w2.swap_axes(0, 1); // special case for 1-D matrix and dot implementation in ndarray

    let b1 = Array1::zeros(n_neurons_per_layer[1]);
    let b2 = Array1::zeros(n_neurons_per_layer[2]);
    
    ANN {
      epoch: 0,
      learning_rate: 0.1,
      weights_1: w1,
      weights_2: w2,
      bias_1: b1,
      bias_2: b2,
    }
  }

  pub fn back_prop_layer_2(&mut self, y1: ArrayView1<f32>, y2: f32, y: f32) -> f32 {
    let diff = y - y2;
    let grad_2 = d_relu(y2); // d_sigmoid(y2);
    let d_2: f32 = diff * grad_2;
    self.bias_2 = &self.bias_2 + d_2 * self.learning_rate;
    
    let dw2: Array1<f32> = d_2 * &y1;
    self.weights_2 = &self.weights_2 + &dw2 * self.learning_rate;

    d_2
  }

  pub fn back_prop_layer_1(&mut self, y1: ArrayView1<f32>, d2: f32, x: ArrayView1<f32>) {
    let i2: Array1<f32> = Array1::ones(y1.raw_dim());
    let grad_1: Array1<f32> = &y1 * (&i2 - &y1); // sigmoid derivative
    // let w2: ArrayView1<f32> = self.weights_2.index_axis(Axis(0), 0);
    let d_1: Array2<f32> = d2 * &grad_1 * &self.weights_2;
    self.bias_1 = &self.bias_1 + &d_1.index_axis(Axis(0), 0) * self.learning_rate;
    
    // let d_1_t: ArrayView2<f32> = d_1.insert_axis(Axis(0)).t(); // shape 2,1
    let dw1: Array2<f32> = &d_1.t() * &x;
    self.weights_1 = &self.weights_1 + &dw1 * self.learning_rate;
  }

  pub fn train(&mut self, epochs: i32, inputs: Array2<f32>, outputs: Array1<f32>) {
    println!("Training started >>>>>>");
    for _ in 0..epochs {
      let n_training_steps = inputs.shape()[0];
      for idx in 0..n_training_steps {
        let x: ArrayView1<f32> = inputs.index_axis(Axis(0), idx);
        let y: &f32 = outputs.get(idx).unwrap();

        // First hidden layer
        let y_1: Array1<f32> = self.forward_layer_1(x);
        // Output
        let y_2: Array1<f32> = self.forward_layer_2(y_1.view());
        let y_: f32 = y_2.get(0).unwrap().to_owned();

        // let diff = y - y_;
        // let grad_2 = 1.0; // y_ * (1.0 - y_); // sigmoid derivative
        // let d_2: f32 = diff * grad_2;
        // let dw2: Array1<f32> = &y_1 * d_2;
        
        // self.weights_2 = &self.weights_2 + &dw2 * self.learning_rate;
        // self.bias_2 = &self.bias_2 + d_2 * self.learning_rate;
        
        let d_2: f32 = self.back_prop_layer_2(y_1.view(), y_, *y);

        // let i2: Array1<f32> = Array1::ones(y_1.raw_dim());
        // let grad_1: Array1<f32> = &y_1 * (&i2 - &y_1); // sigmoid derivative
        // let w2 = self.weights_2.clone();
        // let d_1: Array1<f32> = w2.remove_axis(Axis(0)) * d_2 * &grad_1;
        // self.bias_1 = &self.bias_1 + &d_1 * self.learning_rate;
        // let d_1_t: Array2<f32> = d_1.insert_axis(Axis(0)); // shape 2,1
        // let dw1: Array2<f32> = &d_1_t.t() * &x;
        // self.weights_1 = &self.weights_1 + &dw1 * self.learning_rate;

        self.back_prop_layer_1(y_1.view(), d_2, x);
      }
    }
  }

  pub fn forward_layer_1(&self, x: ArrayView1<f32>) -> Array1<f32> {
    let y_1: Array1<f32> = self.weights_1.dot(&x) + &self.bias_1;
    y_1.mapv(sigmoid)
  }

  pub fn forward_layer_2(&self, y: ArrayView1<f32>) -> Array1<f32> {
    let y_2: Array1<f32> = self.weights_2.dot(&y) + &self.bias_2;
    y_2.mapv(relu)
  }

  pub fn forward(&self, x: ArrayView1<f32>) -> Array1<f32> {
    let mut y_1: Array1<f32> = self.weights_1.dot(&x) + &self.bias_1;
    y_1 = y_1.mapv(sigmoid);
    let mut y_2: Array1<f32> = self.weights_2.dot(&y_1) + &self.bias_2;
    y_2 = y_2.mapv(relu);
    y_2
  }

  pub fn print(&self) {
    println!("weights 1: {:?}", self.weights_1);
    println!("bias 1: {:?}", self.bias_1);
    println!("weights 2: {:?}", self.weights_2);
    println!("bias 2: {:?}", self.bias_2);
  }

}

#[cfg(test)]
mod tests {
  use std::f32::consts::E;
  
  use ndarray::prelude::*;
  use ndarray::{arr1, arr2, Array1, Array2};

  use super::ANN;

  fn take_first(a: Array1<f32>) -> f32 {
    // y_2.get(0).unwrap().to_owned()
    a.to_vec().first().unwrap().clone()
  }

  #[test]
  fn hadamard_product() {
    let a: Array2<f32> = arr2(&[
      [2.0, 3.0],
      [0.0, 8.0]
    ]);
    let b: Array2<f32> = arr2(&[
      [3.0, 1.0],
      [7.0, 9.0],
    ]);
    let p = a * b;
    assert!(p == arr2(&[[6.0, 3.0], [0.0, 72.0]]), "a o b={}", p);
  }

  #[test]
  fn simplify_weights_2() {
    let w: Array2<f32> = arr2(&[[1.0, 2.0]]); // shape = 1,2
    let ww: Array1<f32> = w.clone().remove_axis(Axis(0));
    let w_: Array1<f32> = arr1(&[1.0, 2.0]);
    assert!(ww == w_, "ww = {}, w_ = {}", ww, w_);

    let www: ArrayView1<f32> = w.index_axis(Axis(0), 0);
    assert!(www == w_, "www = {}, w_ = {}", www, w_);
  }

  #[test]
  fn dot_layer_1() {
    let w: Array2<f32> = arr2(&[
      [2.0, 3.0],
      [4.0, 5.0],
    ]);
    let x: Array1<f32> = arr1(&[1.0, 2.0]);
    let x_t: Array2<f32> = arr2(&[
      [1.0],
      [2.0],
    ]);

    // order from numpy implementation => wrong values
    // [ x1 x2 ] dot [ w11 w12 ] = [ w11 * x1 + w21 * x2    = [ x1 x2 ]
    //               [ w21 w22 ]     w12 * x1 + w22 * x2 ]
    let x_dot_w: Array1<f32> = x.dot(&w);
    let y1: Array1<f32> = arr1(&[10.0, 13.0]);
    assert!(x_dot_w == y1, "dot = {}, y = {}", x_dot_w, y1);

    // normal matrix to vector multiplication
    // error: inputs 2 × 1 and 2 × 2 are not compatible for matrix multiplication
    // let x_t_dot_w: Array2<f32> = x_t.dot(&w);
    // let y1_t: Array2<f32> = arr2(&[[10.0], [13.0]]);
    // assert!(x_t_dot_w == y1_t, "dot = {}, y = {}", x_t_dot_w, y1);
    
    // [ w11 w12 ] dot [ x1 ] = [ w11 * x1 + w12 * x2 ] = [ y1 ]
    // [ w21 w22 ]     [ x2 ]   [ w21 * x1 + w22 * x2 ]   [ y2 ]
    
    let y2: Array1<f32> = arr1(&[8.0, 14.0]);
    let w_dot_x: Array1<f32> = w.dot(&x);
    assert!(w_dot_x == y2, "dot = {}, y = {}", w_dot_x, y2);

    // if vector is transposed
    let y2_t: Array2<f32> = arr2(&[[8.0], [14.0]]); // 2,1
    // println!("shape y2_t: {:?}", y2_t.shape());
    let y2_tt: Array2<f32> = arr2(&[[8.0, 14.0]]); // 1,2
    // println!("shape y2_tt: {:?}", y2_tt.shape());
    let w_dot_x_t: Array2<f32> = w.dot(&x_t);
    assert!(w_dot_x_t == y2_t, "dot = {}, y = {}", w_dot_x_t, y2);

    // manual calculation
    let mut y: Array1<f32> = Array1::zeros(2);
    for i in 0..x.len() {
      let y_i = y.get_mut(i).unwrap();
      let row = w.index_axis(Axis(0), i);
      for (j, x_j) in x.iter().enumerate() {
        let w_ij = row.get(j).unwrap();
        *y_i += w_ij * x_j;
      }
    }
    assert!(y == w_dot_x, "dot = {}, y = {}", w_dot_x, y);
  }

  #[test]
  fn dot_layer_2() {
    let mut w_2: Array2<f32> = arr2(&[[2.0], [4.0]]); // shape = (2,1)
    let x_2: Array1<f32> = arr1(&[7.0, 1.0]);
    
    w_2.swap_axes(0, 1);
    let y_2 = w_2.dot(&x_2);
    // [ w1 ] dot [ x1 x2 ] = [ w1 * x1 + w2 * x2 ]  = y
    // [ w2 ]

    w_2.swap_axes(0, 1);
    // manual calculation
    let mut y: f32 = 0.0;
    for (i, x_i) in x_2.iter().enumerate() {
      let w_i = w_2.get((i, 0)).unwrap();
      y += w_i * x_i;
    }
    assert!(y == 18.0, "y = {}, y_2 = {}", y, y_2);

    let y_2_: Array1<f32> = arr1(&[18.0]);
    assert!(y_2 == y_2_, "y = {}, y_ = {}", y_2, y_2_);
    // let w_dot_x_1 = (w_1 * x_1).sum();
    // assert!(w_dot_x_1 == 42.0, "dot = {}, y = {}", w_dot_x_1, y_1);
  }

  #[test]
  fn forward_layer_1() {
    let mut network = ANN::new([2, 2, 1]);
    network.weights_1 = arr2(&[
      [-0.5, -0.5],
      [-0.5, -0.5],
    ]);
    
    let x = arr1(&[1.0, 1.0]);
    let y = network.forward_layer_1(x.view());
    let expected = 1.0 / (1.0 + E);
    let y_: Array1<f32> = arr1(&[expected, expected]);
    assert!(y == y_, "y = {}, expected = {}", y, y_);
  }

  #[test]
  fn forward_layer_2() {
    let mut network = ANN::new([2, 2, 1]);
    network.weights_2 = arr2(&[[1.0, 1.0]]);
    
    let e = 1.0 / (1.0 + E);
    let y: Array1<f32> = arr1(&[e, e]);
    let z = network.forward_layer_2(y.view());
    let expected = 2.0 * e;
    let z_: Array1<f32> = arr1(&[expected]);
    assert!(z == z_, "z = {}, expected = {}", z, z_);
  }

  #[test]
  fn xor_forward_pretrained() {
    let mut network = ANN::new([2, 2, 1]);
    network.weights_1 = arr2(&[
      [2.27, -2.23],
      [-2.698, 2.64],
    ]);
    network.bias_1 = arr1(&[-1.507, -1.896]);
    network.weights_2 = arr2(&[[2.618, 2.58]]);
    network.bias_2 = arr1(&[-0.812]);

    let x_1 = arr1(&[0.0, 0.0]);
    let y_1 = take_first(network.forward(x_1.view()));
    let y_1_ = 0.0;
    assert!(y_1.round() == y_1_, "(1) y = {}, y_ = {}", y_1, y_1_);
    
    let x_2 = arr1(&[1.0, 0.0]);
    let y_2 = take_first(network.forward(x_2.view()));
    let y_2_ = 1.0;
    assert!(y_2.round() == y_2_, "(1) y = {}, y_ = {}", y_2, y_2_);

    let x_3 = arr1(&[0.0, 1.0]);
    let y_3 = take_first(network.forward(x_3.view()));
    let y_3_ = 1.0;
    assert!(y_3.round() == y_3_, "(1) y = {}, y_ = {}", y_3, y_3_);

    let x_4 = arr1(&[1.0, 1.0]);
    let y_4 = take_first(network.forward(x_4.view()));
    let y_4_ = 0.0;
    assert!(y_4.round() == y_4_, "(1) y = {}, y_ = {}", y_4, y_4_);
  }

  #[test]
  fn back_prop_layer_2_matrices() {
    // self.weights_2 += &dw2 * self.learning_rate;
    
    let w: Array2<f32> = arr2(&[[3.0, 5.0]]); // shape = 1,2
    let dw: Array1<f32> = arr1(&[1.0, 2.0]);
    let rate: f32 = 0.5;
    
    let t: Array1<f32> = &dw * rate;
    let t_: Array1<f32> = arr1(&[0.5, 1.0]);
    assert!(t == t_, "t = {}, t_ = {}", t, t_);

    let ww: Array2<f32> = &w * &t;
    let ww_ = arr2(&[[1.5, 5.0]]);
    assert!(ww == ww_, "ww = {}, ww_ = {}", ww, ww_);

    let mut www: Array2<f32> = w.clone();
    www = www + &t;
    assert!(www == &w + &t, "www = {}", www);
    assert!(www == &w + &dw * rate, "www = {}", www);
    let r: Array2<f32> = arr2(&[[3.5, 6.0]]);
    assert!(www == r, "www = {}, r = {}", www, r);
  }

  #[test]
  fn back_prop_layer_2() {
    let mut network = ANN::new([2, 2, 1]);
    network.weights_2 = arr2(&[[0.0, 0.0]]);
    network.bias_2 = arr1(&[0.0]);

    // let y1: Array1<f32> = arr1(&[1.0, 1.0]);
    // let y2 = 1.0;
    // let y = 1.0;
    // let d2 = network.back_prop_layer_2(y1.view(), y2, y);

    // assert!(network.bias_2 == arr1(&[0.0]), "bias 2 = {}", network.bias_2);
    // assert!(network.weights_2 == arr2(&[[0.0, 0.0]]), "weight 2 = {}", network.weights_2);
    // assert!(d2 == 0.0, "d2 = {}", d2);

    let y1: Array1<f32> = arr1(&[1.0, 2.0]);
    let y2 = 0.5;
    let y = 1.0;
    let d2 = network.back_prop_layer_2(y1.view(), y2, y);

    assert!(network.bias_2 == arr1(&[0.05]), "bias 2 = {}", network.bias_2);
    assert!(network.weights_2 == arr2(&[[0.05, 0.1]]), "weight 2 = {}", network.weights_2);
    assert!(d2 == 0.5, "d2 = {}", d2);
  }

  #[test]
  fn back_prop_layer_1() {
    let mut network = ANN::new([2, 2, 1]);
    network.weights_1 = arr2(&[
      [0.0, 0.0],
      [0.0, 0.0],
    ]);
    network.bias_1 = arr1(&[0.0, 0.0]);
    network.weights_2 = arr2(&[[4.0, -1.0]]);

    let y1: Array1<f32> = arr1(&[0.5, 2.0]);
    let d2 = 1.0;
    let x: Array1<f32> = arr1(&[1.0, 3.0]);
    network.back_prop_layer_1(y1.view(), d2, x.view());

    assert!(network.bias_1 == arr1(&[0.1, 0.2]), "bias 1 = {}", network.bias_1);
    assert!(network.weights_1 == arr2(&[[0.1, 0.3], [0.2, 0.6]]), "weight 1 = {}", network.weights_1);
  }

  #[test]
  fn back_prop_layer_1_matrices() {
    // let d_1 = (&self.weights_2 * &d_2) * grad_1;
    let x: Array1<f32> = arr1(&[1.0, 2.0]);
    // println!("Original 1D array: {:?}", x);
    let d: Array1<f32> = arr1(&[3.0, 5.0]);
    let mut x_: Array2<f32> = x.clone().insert_axis(Axis(0));
    // println!("Converted 2D array: {:?}", x_);
    // println!("Transposed 2D array: {:?}", x_.t());
    let x_t = x_.t();
    // let y: Array2<f32> = &x_t * &d;
    let y: Array2<f32> = &d * &x_t;
    let y_: Array2<f32> = arr2(&[
      [3.0, 5.0],
      [6.0, 10.0],
    ]);
    assert!(y == y_, "y = {}, y_ = {}", y, y_);

    let w: Array2<f32> = arr2(&[[3.0, 5.0]]); // shape = 1,2
    let yy: Array2<f32> = &w.t() * 2.0 * &x;
    let yy_: Array2<f32> = arr2(&[
      [3.0, 5.0],
      [6.0, 10.0],
    ]);
    assert!(yy == &yy_.t() * 2.0, "yy = {}", yy);
  }

  #[test]
  fn xor_train() {
    let mut network = ANN::new([2, 2, 1]);
    let epochs = 3000;
    let inputs: Array2<f32> = arr2(&[
      [0.0, 0.0],
      [0.0, 1.0],
      [1.0, 0.0],
      [1.0, 1.0]
    ]);
    let outputs: Array1<f32> = arr1(&[0.0, 1.0, 1.0, 0.0]);
    network.train(epochs, inputs, outputs);
    
    println!("weights 1: {:?}", network.weights_1);
    println!("bias 1: {:?}", network.bias_1);
    println!("weights 2: {:?}", network.weights_2);
    println!("bias 2: {:?}", network.bias_2);

    let x_1 = arr1(&[0.0, 0.0]);
    let y_1 = take_first(network.forward(x_1.view()));
    let y_1_ = 0.0;
    assert!(y_1.round() == y_1_, "(1) y = {}, y_ = {}", y_1, y_1_);
    
    let x_2 = arr1(&[1.0, 0.0]);
    let y_2 = take_first(network.forward(x_2.view()));
    let y_2_ = 1.0;
    assert!(y_2.round() == y_2_, "(1) y = {}, y_ = {}", y_2, y_2_);

    let x_3 = arr1(&[0.0, 1.0]);
    let y_3 = take_first(network.forward(x_3.view()));
    let y_3_ = 1.0;
    assert!(y_3.round() == y_3_, "(1) y = {}, y_ = {}", y_3, y_3_);

    let x_4 = arr1(&[1.0, 1.0]);
    let y_4 = take_first(network.forward(x_4.view()));
    let y_4_ = 0.0;
    assert!(y_4.round() == y_4_, "(1) y = {}, y_ = {}", y_4, y_4_);
  }
}
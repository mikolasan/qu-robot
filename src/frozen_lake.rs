use rsrl::{
  domains::{Domain, Observation, State, Reward, Action}, 
  spaces::{discrete::Integers, ProductSpace},
};

pub struct FrozenLake;

impl FrozenLake {
  pub fn new() -> FrozenLake {
    FrozenLake
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

  fn step(&mut self, a: &Action<Self>) -> (Observation<State<Self>>, Reward) {
    todo!()
  }
}
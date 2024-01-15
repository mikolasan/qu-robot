use gym::{spaces::Discrete, Client};
use rsrl::{
    control::td::sarsa::Linear,
    domains::{Domain, Observation},
    logging,
    policies::fixed::EpsilonGreedy,
    run,
};

fn main() {
    // Connect to the Frost Lake environment
    let mut client = Client::default().unwrap();
    let env = client
        .make("FrozenLake-v1")
        .map(|mut e| {
            // Customize environment if needed
            e.set_max_episode_steps(Some(1000));
            e
        })
        .unwrap();

    // Set up the SARSA algorithm with a linear function approximator
    let n_actions = env.action_space().card().into();
    let n_features = env.observation_space().shape().iter().product();
    let mut agent = Linear::new(EpsilonGreedy::new(0.1, n_actions), n_features);

    // Run the reinforcement learning loop
    run(
        &mut agent,
        &mut env,
        logging::standard(),
        Some(1000), // Set the number of episodes
        None,       // Run until convergence
    );
}

#![forbid(unsafe_code)]
//! Thin facade for the `use-simulation` workspace.
//!
//! The crate reexports the focused simulation crates directly so consumers can
//! opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_simulation::*;
//!
//! let seed = SimulationSeed::new(5);
//! let path = random_walk(RandomWalkConfig {
//!     start: 0.0,
//!     step_size: 1.0,
//!     steps: 3,
//!     seed,
//! })
//! .unwrap();
//! let result = summarize(&path).unwrap();
//! let mut clock = SimulationClock::new(0.5).unwrap();
//!
//! assert_eq!(clock.advance().unwrap(), 0.5);
//! assert_eq!(result.steps, 3);
//! assert_eq!(step_state(1.0, 0.5, 0).unwrap().next_state, 1.5);
//! ```

pub use use_event_simulation;
pub use use_event_simulation::*;
pub use use_monte_carlo;
pub use use_monte_carlo::*;
pub use use_random_walk;
pub use use_random_walk::*;
pub use use_seed;
pub use use_seed::*;
pub use use_simulation_clock;
pub use use_simulation_clock::*;
pub use use_simulation_result;
pub use use_simulation_result::*;
pub use use_state_step;
pub use use_state_step::*;

#[cfg(test)]
mod tests {
    use super::{
        RandomWalkConfig, SimulationClock, SimulationEvent, SimulationSeed, monte_carlo_mean,
        random_walk, run_event_simulation, step_state, summarize,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let seed = SimulationSeed::new(3);
        let branches = seed.split(2).unwrap();
        assert_eq!(branches.len(), 2);

        let path = random_walk(RandomWalkConfig {
            start: 0.0,
            step_size: 1.0,
            steps: 4,
            seed,
        })
        .unwrap();
        let result = summarize(&path).unwrap();
        assert_eq!(result.steps, 4);

        let estimate = monte_carlo_mean(seed, 4, |sample| sample).unwrap();
        assert_eq!(estimate.samples, 4);

        let states = run_event_simulation(
            0.0,
            &[
                SimulationEvent::new(1.0, 2.0).unwrap(),
                SimulationEvent::new(2.0, -1.0).unwrap(),
            ],
        )
        .unwrap();
        assert_eq!(states.len(), 3);

        let mut clock = SimulationClock::new(1.0).unwrap();
        assert_eq!(clock.advance().unwrap(), 1.0);
        assert_eq!(step_state(1.0, 2.0, 0).unwrap().next_state, 3.0);
    }
}

#![forbid(unsafe_code)]
//! Repeatable one-dimensional random walk helpers.
//!
//! The walk uses `SimulationSeed` to derive deterministic left and right moves
//! so repeated runs with the same configuration produce the same path.
//!
//! # Examples
//!
//! ```rust
//! use use_random_walk::{random_walk, summarize_walk, RandomWalkConfig};
//! use use_seed::SimulationSeed;
//!
//! let path = random_walk(RandomWalkConfig {
//!     start: 0.0,
//!     step_size: 1.0,
//!     steps: 4,
//!     seed: SimulationSeed::new(5),
//! })
//! .unwrap();
//! let summary = summarize_walk(&path).unwrap();
//!
//! assert_eq!(path.len(), 5);
//! assert_eq!(summary.final_position, *path.last().unwrap());
//! ```

use use_seed::SimulationSeed;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomWalkConfig {
    pub start: f64,
    pub step_size: f64,
    pub steps: usize,
    pub seed: SimulationSeed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomWalkSummary {
    pub final_position: f64,
    pub min_position: f64,
    pub max_position: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomWalkError {
    InvalidStart,
    InvalidStepSize,
    NonFinitePosition,
}

pub fn random_walk(config: RandomWalkConfig) -> Result<Vec<f64>, RandomWalkError> {
    if !config.start.is_finite() {
        return Err(RandomWalkError::InvalidStart);
    }

    if !config.step_size.is_finite() || config.step_size < 0.0 {
        return Err(RandomWalkError::InvalidStepSize);
    }

    let mut current = config.start;
    let mut path = Vec::with_capacity(config.steps + 1);
    path.push(current);

    for step_index in 0..config.steps {
        let direction = if config.seed.mix(step_index + 1).to_unit_f64() < 0.5 {
            -1.0
        } else {
            1.0
        };

        let next = current + direction * config.step_size;
        if !next.is_finite() {
            return Err(RandomWalkError::NonFinitePosition);
        }

        path.push(next);
        current = next;
    }

    Ok(path)
}

pub fn summarize_walk(path: &[f64]) -> Option<RandomWalkSummary> {
    if path.is_empty() || path.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let mut min_position = path[0];
    let mut max_position = path[0];
    for value in path.iter().copied().skip(1) {
        if value < min_position {
            min_position = value;
        }
        if value > max_position {
            max_position = value;
        }
    }

    Some(RandomWalkSummary {
        final_position: *path.last()?,
        min_position,
        max_position,
    })
}

#[cfg(test)]
mod tests {
    use super::{RandomWalkConfig, RandomWalkError, random_walk, summarize_walk};
    use use_seed::SimulationSeed;

    #[test]
    fn produces_repeatable_paths() {
        let config = RandomWalkConfig {
            start: 0.0,
            step_size: 1.0,
            steps: 5,
            seed: SimulationSeed::new(4),
        };

        let first = random_walk(config).unwrap();
        let second = random_walk(config).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 6);
        assert!(
            first
                .windows(2)
                .all(|pair| (pair[1] - pair[0]).abs() == 1.0)
        );
    }

    #[test]
    fn summarizes_walk_positions() {
        let path = random_walk(RandomWalkConfig {
            start: 0.0,
            step_size: 0.5,
            steps: 4,
            seed: SimulationSeed::new(7),
        })
        .unwrap();
        let summary = summarize_walk(&path).unwrap();

        assert_eq!(summary.final_position, *path.last().unwrap());
        assert!(summary.min_position <= summary.max_position);
    }

    #[test]
    fn handles_zero_steps() {
        assert_eq!(
            random_walk(RandomWalkConfig {
                start: 3.0,
                step_size: 1.0,
                steps: 0,
                seed: SimulationSeed::new(1),
            })
            .unwrap(),
            vec![3.0]
        );
    }

    #[test]
    fn rejects_invalid_configuration() {
        assert_eq!(
            random_walk(RandomWalkConfig {
                start: f64::NAN,
                step_size: 1.0,
                steps: 1,
                seed: SimulationSeed::new(1),
            }),
            Err(RandomWalkError::InvalidStart)
        );
        assert_eq!(
            random_walk(RandomWalkConfig {
                start: 0.0,
                step_size: -1.0,
                steps: 1,
                seed: SimulationSeed::new(1),
            }),
            Err(RandomWalkError::InvalidStepSize)
        );
        assert_eq!(summarize_walk(&[]), None);
    }
}

#![forbid(unsafe_code)]
//! Primitive Monte Carlo helpers for repeatable simulation runs.
//!
//! The crate evaluates a caller-provided observation function over deterministic
//! unit samples derived from a `SimulationSeed`.
//!
//! # Examples
//!
//! ```rust
//! use use_monte_carlo::monte_carlo_mean;
//! use use_seed::SimulationSeed;
//!
//! let estimate = monte_carlo_mean(SimulationSeed::new(4), 8, |sample| sample * 2.0).unwrap();
//!
//! assert_eq!(estimate.samples, 8);
//! assert!(estimate.mean.is_finite());
//! assert!(estimate.variance.is_finite());
//! ```

use use_seed::SimulationSeed;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonteCarloEstimate {
    pub mean: f64,
    pub variance: f64,
    pub samples: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonteCarloError {
    InvalidSampleCount,
    NonFiniteObservation,
}

pub fn monte_carlo_mean<F>(
    seed: SimulationSeed,
    samples: usize,
    observation: F,
) -> Result<MonteCarloEstimate, MonteCarloError>
where
    F: Fn(f64) -> f64,
{
    if samples == 0 {
        return Err(MonteCarloError::InvalidSampleCount);
    }

    let mut values = Vec::with_capacity(samples);
    for index in 0..samples {
        let sample = seed.mix(index + 1).to_unit_f64();
        let value = observation(sample);
        if !value.is_finite() {
            return Err(MonteCarloError::NonFiniteObservation);
        }

        values.push(value);
    }

    let mean = values.iter().sum::<f64>() / samples as f64;
    let variance = values
        .iter()
        .map(|value| {
            let diff = *value - mean;
            diff * diff
        })
        .sum::<f64>()
        / samples as f64;

    Ok(MonteCarloEstimate {
        mean,
        variance,
        samples,
    })
}

#[cfg(test)]
mod tests {
    use super::{MonteCarloError, monte_carlo_mean};
    use use_seed::SimulationSeed;

    #[test]
    fn estimates_constant_observations() {
        let estimate = monte_carlo_mean(SimulationSeed::new(3), 5, |_sample| 2.0).unwrap();

        assert_eq!(estimate.mean, 2.0);
        assert_eq!(estimate.variance, 0.0);
        assert_eq!(estimate.samples, 5);
    }

    #[test]
    fn stays_repeatable_for_the_same_seed() {
        let first = monte_carlo_mean(SimulationSeed::new(11), 8, |sample| sample * 2.0).unwrap();
        let second = monte_carlo_mean(SimulationSeed::new(11), 8, |sample| sample * 2.0).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            monte_carlo_mean(SimulationSeed::new(1), 0, |sample| sample),
            Err(MonteCarloError::InvalidSampleCount)
        );
        assert_eq!(
            monte_carlo_mean(SimulationSeed::new(1), 2, |_sample| f64::NAN),
            Err(MonteCarloError::NonFiniteObservation)
        );
    }
}

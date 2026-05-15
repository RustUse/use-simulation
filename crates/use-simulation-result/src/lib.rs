#![forbid(unsafe_code)]
//! Summary helpers for finite simulation result series.
//!
//! The crate turns a sequence of finite `f64` values into a reusable summary
//! with simple aggregate helpers.
//!
//! # Examples
//!
//! ```rust
//! use use_simulation_result::summarize;
//!
//! let result = summarize(&[1.0, 2.0, 1.5, 3.0]).unwrap();
//!
//! assert_eq!(result.final_value, 3.0);
//! assert_eq!(result.steps, 3);
//! assert_eq!(result.range(), 2.0);
//! ```

#[derive(Debug, Clone, PartialEq)]
pub struct SimulationResult {
    pub values: Vec<f64>,
    pub final_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub steps: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationResultError {
    EmptySeries,
    NonFiniteValue,
}

impl SimulationResult {
    pub fn mean(&self) -> f64 {
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    pub fn range(&self) -> f64 {
        self.max_value - self.min_value
    }
}

pub fn summarize(values: &[f64]) -> Result<SimulationResult, SimulationResultError> {
    if values.is_empty() {
        return Err(SimulationResultError::EmptySeries);
    }

    if values.iter().any(|value| !value.is_finite()) {
        return Err(SimulationResultError::NonFiniteValue);
    }

    let mut min_value = values[0];
    let mut max_value = values[0];
    for value in values.iter().copied().skip(1) {
        if value < min_value {
            min_value = value;
        }
        if value > max_value {
            max_value = value;
        }
    }

    Ok(SimulationResult {
        values: values.to_vec(),
        final_value: *values.last().unwrap(),
        min_value,
        max_value,
        steps: values.len() - 1,
    })
}

#[cfg(test)]
mod tests {
    use super::{SimulationResultError, summarize};

    #[test]
    fn summarizes_finite_series() {
        let result = summarize(&[1.0, 2.0, 1.5, 3.0]).unwrap();

        assert_eq!(result.final_value, 3.0);
        assert_eq!(result.min_value, 1.0);
        assert_eq!(result.max_value, 3.0);
        assert_eq!(result.steps, 3);
        assert_eq!(result.mean(), 1.875);
        assert_eq!(result.range(), 2.0);
    }

    #[test]
    fn handles_single_value_series() {
        let result = summarize(&[4.0]).unwrap();

        assert_eq!(result.steps, 0);
        assert_eq!(result.range(), 0.0);
        assert_eq!(result.mean(), 4.0);
    }

    #[test]
    fn rejects_invalid_series() {
        assert_eq!(summarize(&[]), Err(SimulationResultError::EmptySeries));
        assert_eq!(
            summarize(&[1.0, f64::NAN]),
            Err(SimulationResultError::NonFiniteValue)
        );
    }
}

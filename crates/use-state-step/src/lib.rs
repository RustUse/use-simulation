#![forbid(unsafe_code)]
//! Explicit state stepping helpers for finite simulations.
//!
//! The crate models each update as a concrete `StateStep` so simulations can
//! retain both the previous and next state for later inspection.
//!
//! # Examples
//!
//! ```rust
//! use use_state_step::{apply_steps, states};
//!
//! let path = states(1.0, &[0.5, -0.25]).unwrap();
//! let steps = apply_steps(1.0, &[0.5, -0.25]).unwrap();
//!
//! assert_eq!(path, vec![1.0, 1.5, 1.25]);
//! assert_eq!(steps[1].delta, -0.25);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateStep {
    pub step_index: usize,
    pub previous_state: f64,
    pub next_state: f64,
    pub delta: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateStepError {
    NonFiniteState,
    NonFiniteDelta,
}

pub fn step_state(
    previous_state: f64,
    delta: f64,
    step_index: usize,
) -> Result<StateStep, StateStepError> {
    if !previous_state.is_finite() {
        return Err(StateStepError::NonFiniteState);
    }

    if !delta.is_finite() {
        return Err(StateStepError::NonFiniteDelta);
    }

    let next_state = previous_state + delta;
    if !next_state.is_finite() {
        return Err(StateStepError::NonFiniteState);
    }

    Ok(StateStep {
        step_index,
        previous_state,
        next_state,
        delta,
    })
}

pub fn apply_steps(initial_state: f64, deltas: &[f64]) -> Result<Vec<StateStep>, StateStepError> {
    if !initial_state.is_finite() {
        return Err(StateStepError::NonFiniteState);
    }

    let mut current_state = initial_state;
    let mut steps = Vec::with_capacity(deltas.len());

    for (step_index, delta) in deltas.iter().copied().enumerate() {
        let step = step_state(current_state, delta, step_index)?;
        current_state = step.next_state;
        steps.push(step);
    }

    Ok(steps)
}

pub fn states(initial_state: f64, deltas: &[f64]) -> Result<Vec<f64>, StateStepError> {
    let steps = apply_steps(initial_state, deltas)?;
    let mut values = Vec::with_capacity(steps.len() + 1);
    values.push(initial_state);
    values.extend(steps.iter().map(|step| step.next_state));
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::{StateStepError, apply_steps, states, step_state};

    #[test]
    fn steps_state_forward() {
        let step = step_state(1.0, 0.5, 0).unwrap();

        assert_eq!(step.previous_state, 1.0);
        assert_eq!(step.next_state, 1.5);
        assert_eq!(step.delta, 0.5);
    }

    #[test]
    fn applies_multiple_steps() {
        let steps = apply_steps(1.0, &[0.5, -0.25, 1.0]).unwrap();

        assert_eq!(steps.len(), 3);
        assert_eq!(steps[2].next_state, 2.25);
        assert_eq!(
            states(1.0, &[0.5, -0.25, 1.0]).unwrap(),
            vec![1.0, 1.5, 1.25, 2.25]
        );
    }

    #[test]
    fn allows_empty_step_lists() {
        assert_eq!(apply_steps(3.0, &[]).unwrap(), Vec::new());
        assert_eq!(states(3.0, &[]).unwrap(), vec![3.0]);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            step_state(f64::NAN, 1.0, 0),
            Err(StateStepError::NonFiniteState)
        );
        assert_eq!(
            step_state(1.0, f64::NAN, 0),
            Err(StateStepError::NonFiniteDelta)
        );
        assert_eq!(
            apply_steps(f64::INFINITY, &[1.0]),
            Err(StateStepError::NonFiniteState)
        );
    }
}

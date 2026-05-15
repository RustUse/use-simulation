#![forbid(unsafe_code)]
//! Deterministic simulation clock helpers.
//!
//! The crate keeps time in integer ticks and derives elapsed time from a
//! finite `f64` tick duration.
//!
//! # Examples
//!
//! ```rust
//! use use_simulation_clock::SimulationClock;
//!
//! let mut clock = SimulationClock::new(0.5).unwrap();
//! assert_eq!(clock.advance().unwrap(), 0.5);
//! assert_eq!(clock.advance_by(3).unwrap(), 2.0);
//! assert_eq!(clock.tick(), 4);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationClock {
    tick: usize,
    tick_duration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationClockError {
    InvalidTickDuration,
    TickOverflow,
    NonFiniteElapsed,
}

impl SimulationClock {
    pub fn new(tick_duration: f64) -> Result<Self, SimulationClockError> {
        Self::at_tick(0, tick_duration)
    }

    pub fn at_tick(tick: usize, tick_duration: f64) -> Result<Self, SimulationClockError> {
        if !tick_duration.is_finite() || tick_duration <= 0.0 {
            return Err(SimulationClockError::InvalidTickDuration);
        }

        let elapsed = tick as f64 * tick_duration;
        if !elapsed.is_finite() {
            return Err(SimulationClockError::NonFiniteElapsed);
        }

        Ok(Self {
            tick,
            tick_duration,
        })
    }

    pub fn tick(&self) -> usize {
        self.tick
    }

    pub fn tick_duration(&self) -> f64 {
        self.tick_duration
    }

    pub fn elapsed(&self) -> f64 {
        self.tick as f64 * self.tick_duration
    }

    pub fn advance(&mut self) -> Result<f64, SimulationClockError> {
        self.advance_by(1)
    }

    pub fn advance_by(&mut self, steps: usize) -> Result<f64, SimulationClockError> {
        self.tick = self
            .tick
            .checked_add(steps)
            .ok_or(SimulationClockError::TickOverflow)?;

        let elapsed = self.elapsed();
        if !elapsed.is_finite() {
            return Err(SimulationClockError::NonFiniteElapsed);
        }

        Ok(elapsed)
    }
}

pub fn elapsed_for(tick_duration: f64, ticks: usize) -> Option<f64> {
    if !tick_duration.is_finite() || tick_duration <= 0.0 {
        return None;
    }

    let elapsed = tick_duration * ticks as f64;
    elapsed.is_finite().then_some(elapsed)
}

#[cfg(test)]
mod tests {
    use super::{SimulationClock, SimulationClockError, elapsed_for};

    #[test]
    fn advances_clock_in_ticks() {
        let mut clock = SimulationClock::new(0.5).unwrap();

        assert_eq!(clock.elapsed(), 0.0);
        assert_eq!(clock.advance().unwrap(), 0.5);
        assert_eq!(clock.advance_by(3).unwrap(), 2.0);
        assert_eq!(clock.tick(), 4);
        assert_eq!(clock.tick_duration(), 0.5);
    }

    #[test]
    fn can_start_at_existing_tick() {
        let clock = SimulationClock::at_tick(3, 0.25).unwrap();

        assert_eq!(clock.tick(), 3);
        assert_eq!(clock.elapsed(), 0.75);
        assert_eq!(elapsed_for(0.25, 3), Some(0.75));
    }

    #[test]
    fn rejects_invalid_duration() {
        assert_eq!(
            SimulationClock::new(0.0),
            Err(SimulationClockError::InvalidTickDuration)
        );
        assert_eq!(elapsed_for(f64::NAN, 2), None);
    }

    #[test]
    fn rejects_non_finite_elapsed_time() {
        assert_eq!(
            SimulationClock::at_tick(usize::MAX, f64::MAX),
            Err(SimulationClockError::NonFiniteElapsed)
        );
    }
}

#![forbid(unsafe_code)]
//! Simple event simulation helpers over finite event lists.
//!
//! Events are modeled as time and change pairs. The runtime sorts them by time
//! and applies each change to a scalar state.
//!
//! # Examples
//!
//! ```rust
//! use use_event_simulation::{run_event_simulation, SimulationEvent};
//!
//! let states = run_event_simulation(
//!     10.0,
//!     &[
//!         SimulationEvent::new(2.0, -1.0).unwrap(),
//!         SimulationEvent::new(1.0, 3.0).unwrap(),
//!     ],
//! )
//! .unwrap();
//!
//! assert_eq!(states[0].state, 10.0);
//! assert_eq!(states[1].time, 1.0);
//! assert_eq!(states[2].state, 12.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationEvent {
    pub time: f64,
    pub change: f64,
}

impl SimulationEvent {
    pub fn new(time: f64, change: f64) -> Option<Self> {
        if !time.is_finite() || time < 0.0 || !change.is_finite() {
            return None;
        }

        Some(Self { time, change })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventState {
    pub time: f64,
    pub state: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSimulationError {
    InvalidInitialState,
    InvalidEvent,
    NonFiniteState,
}

pub fn sorted_events(
    events: &[SimulationEvent],
) -> Result<Vec<SimulationEvent>, EventSimulationError> {
    if events
        .iter()
        .any(|event| !event.time.is_finite() || event.time < 0.0 || !event.change.is_finite())
    {
        return Err(EventSimulationError::InvalidEvent);
    }

    let mut sorted = events.to_vec();
    sorted.sort_by(|left, right| left.time.total_cmp(&right.time));
    Ok(sorted)
}

pub fn run_event_simulation(
    initial_state: f64,
    events: &[SimulationEvent],
) -> Result<Vec<EventState>, EventSimulationError> {
    if !initial_state.is_finite() {
        return Err(EventSimulationError::InvalidInitialState);
    }

    let events = sorted_events(events)?;
    let mut states = Vec::with_capacity(events.len() + 1);
    let mut current_state = initial_state;
    states.push(EventState {
        time: 0.0,
        state: current_state,
    });

    for event in events {
        current_state += event.change;
        if !current_state.is_finite() {
            return Err(EventSimulationError::NonFiniteState);
        }

        states.push(EventState {
            time: event.time,
            state: current_state,
        });
    }

    Ok(states)
}

#[cfg(test)]
mod tests {
    use super::{EventSimulationError, SimulationEvent, run_event_simulation, sorted_events};

    #[test]
    fn sorts_events_by_time() {
        let sorted = sorted_events(&[
            SimulationEvent::new(3.0, 1.0).unwrap(),
            SimulationEvent::new(1.0, 2.0).unwrap(),
        ])
        .unwrap();

        assert_eq!(sorted[0].time, 1.0);
        assert_eq!(sorted[1].time, 3.0);
    }

    #[test]
    fn runs_event_simulation() {
        let states = run_event_simulation(
            10.0,
            &[
                SimulationEvent::new(2.0, -1.0).unwrap(),
                SimulationEvent::new(1.0, 3.0).unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(states.len(), 3);
        assert_eq!(states[0].state, 10.0);
        assert_eq!(states[1].time, 1.0);
        assert_eq!(states[1].state, 13.0);
        assert_eq!(states[2].state, 12.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(SimulationEvent::new(-1.0, 1.0), None);
        assert_eq!(
            run_event_simulation(f64::NAN, &[]),
            Err(EventSimulationError::InvalidInitialState)
        );
        assert_eq!(
            sorted_events(&[SimulationEvent {
                time: 1.0,
                change: f64::NAN,
            }]),
            Err(EventSimulationError::InvalidEvent)
        );
    }
}

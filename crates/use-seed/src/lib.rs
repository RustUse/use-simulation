#![forbid(unsafe_code)]
//! Repeatable seed helpers for primitive simulations.
//!
//! The crate provides a tiny deterministic mixing function so sibling crates
//! can derive stable pseudo-random samples without pulling in a full RNG
//! dependency.
//!
//! # Examples
//!
//! ```rust
//! use use_seed::SimulationSeed;
//!
//! let seed = SimulationSeed::new(7);
//! let branches = seed.split(3).unwrap();
//!
//! assert_eq!(branches.len(), 3);
//! assert!(seed.to_unit_f64() >= 0.0);
//! assert!(seed.to_unit_f64() <= 1.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimulationSeed {
    value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedError {
    ZeroCount,
}

impl SimulationSeed {
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    pub const fn value(self) -> usize {
        self.value
    }

    pub fn mix(self, salt: usize) -> Self {
        let salted = self
            .value
            .wrapping_add(salt.wrapping_mul(0x9E37_79B1usize))
            .wrapping_add(0x85EB_CA6Busize);

        Self {
            value: mix_value(salted),
        }
    }

    pub fn split(self, count: usize) -> Result<Vec<Self>, SeedError> {
        if count == 0 {
            return Err(SeedError::ZeroCount);
        }

        Ok((0..count).map(|index| self.mix(index + 1)).collect())
    }

    pub fn to_unit_f64(self) -> f64 {
        mix_value(self.value) as f64 / usize::MAX as f64
    }
}

pub fn mix_value(mut value: usize) -> usize {
    value ^= value >> (usize::BITS / 3);
    value = value.wrapping_mul(0x85EB_CA6Busize);
    value ^= value >> (usize::BITS / 4);
    value = value.wrapping_mul(0xC2B2_AE35usize);
    value ^= value >> (usize::BITS / 5);
    value
}

pub fn unit_f64_from_seed(seed: usize) -> f64 {
    SimulationSeed::new(seed).to_unit_f64()
}

#[cfg(test)]
mod tests {
    use super::{SeedError, SimulationSeed, mix_value, unit_f64_from_seed};

    #[test]
    fn mixes_repeatably() {
        let seed = SimulationSeed::new(42);

        assert_eq!(seed.mix(3), seed.mix(3));
        assert_ne!(seed.mix(1), seed.mix(2));
        assert_eq!(mix_value(5), mix_value(5));
    }

    #[test]
    fn splits_into_repeatable_branches() {
        let seed = SimulationSeed::new(9);
        let branches = seed.split(3).unwrap();

        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0], seed.mix(1));
        assert_eq!(branches[2], seed.mix(3));
    }

    #[test]
    fn converts_to_unit_interval() {
        let value = unit_f64_from_seed(7);

        assert!((0.0..=1.0).contains(&value));
        assert_eq!(value, SimulationSeed::new(7).to_unit_f64());
    }

    #[test]
    fn rejects_zero_split_count() {
        assert_eq!(SimulationSeed::new(1).split(0), Err(SeedError::ZeroCount));
    }
}

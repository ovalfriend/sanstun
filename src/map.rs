//! The hash map used internally: `hashbrown` with SipHash-1-3, keyed from the interface's seed.
//!
//! This is the hasher `std::collections::HashMap` uses by default. The keys come from the
//! caller's seed instead of the operating system, so it does not need `std`. Random keys matter
//! because the rate limiter's keys are source addresses chosen by whoever sends us packets.

use core::hash::BuildHasher;

use rand::{CryptoRng, RngExt};
use siphasher::sip::SipHasher13;

pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V, RandomState>;

/// Builds SipHash-1-3 hashers with random keys.
#[derive(Clone)]
pub(crate) struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    pub(crate) fn new(rng: &mut impl CryptoRng) -> Self {
        Self {
            k0: rng.random(),
            k1: rng.random(),
        }
    }
}

impl BuildHasher for RandomState {
    type Hasher = SipHasher13;

    fn build_hasher(&self) -> SipHasher13 {
        SipHasher13::new_with_keys(self.k0, self.k1)
    }
}

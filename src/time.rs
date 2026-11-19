//! A caller-provided timestamp for the sans-IO API.

use core::ops::{Add, AddAssign, Sub};
use core::time::Duration;

/// A point in time on a monotonic clock.
///
/// `sanstun` is sans-IO, including time: it never reads a clock. Construct instants from your
/// monotonic clock's elapsed time since an arbitrary epoch, and pass the clock's current value to
/// every call that takes a `now`.
///
/// The clock must not go backwards. Durations saturate like the standard library's `Instant`:
/// [`Instant::duration_since`] returns zero when the other instant is later.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(Duration);

impl Instant {
    /// The instant at your clock's epoch.
    pub const EPOCH: Self = Self(Duration::ZERO);

    /// Creates the instant `elapsed` after your clock's epoch.
    pub const fn from_duration_since_epoch(elapsed: Duration) -> Self {
        Self(elapsed)
    }

    /// Returns the time from `earlier` to `self`, or zero if `earlier` is later.
    pub const fn duration_since(&self, earlier: Self) -> Duration {
        self.0.saturating_sub(earlier.0)
    }

    /// Returns the time from `earlier` to `self`, or `None` if `earlier` is later.
    pub const fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.0.checked_sub(earlier.0)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        Self(
            self.0
                .checked_add(rhs)
                .expect("overflow when adding a duration to an instant"),
        )
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        Self(
            self.0
                .checked_sub(rhs)
                .expect("overflow when subtracting a duration from an instant"),
        )
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, earlier: Self) -> Duration {
        self.duration_since(earlier)
    }
}

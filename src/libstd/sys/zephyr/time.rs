use crate::time::Duration;
use zephyr::InstantMs;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(InstantMs);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

impl Instant {
    pub fn now() -> Instant {
        Instant(zephyr::any::k_uptime_get_ms())
    }

    pub const fn zero() -> Instant {
        Instant(InstantMs::zero())
    }

    pub fn actually_monotonic() -> bool {
        true
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0).map(Into::into)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_add_duration(other).map(Instant)
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_sub_duration(other).map(Instant)
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        unimplemented!()
    }

    pub fn sub_time(&self, other: &SystemTime)
                    -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}

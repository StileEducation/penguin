use std::sync::{
    atomic::{AtomicU32, Ordering},
    LazyLock,
};

/// Global counter for differentiating IDs within the same second. Per the spec,
/// it is initialised to a random value, then incremented from there.
static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(rand::random()));

/// A unique machine ID (really a process ID). This is generated once per
/// process and used in all IDs.
static MACHINE_ID: LazyLock<[u8; 5]> = LazyLock::new(|| {
    let mut machine_id = [0; 5];
    rand::fill(&mut machine_id);
    machine_id
});

/// Maximum value of the counter. It can be 24 bits before we wrap it back to 0.
const COUNTER_MAX: u32 = (2 << 23) - 1;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid hex string: {0}")]
    InvalidHexString(#[from] hex::FromHexError),
    #[error("invalid hex ID must be 12 bytes, got {0}")]
    InvalidHexIdLength(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ObjectId {
    timestamp: [u8; 4],
    machine_id: [u8; 5],
    counter: [u8; 3],
}

impl ObjectId {
    /// Generate a new, unique, object ID for the current time.
    pub fn new() -> Self {
        let mut t = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        assert_eq!(
            // This is significantly more performant that using the safe
            // `std::time::SystemTime::now` (26% based on benchmarking).
            unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut t) },
            0
        );
        Self::from_time(t.tv_sec, true)
    }

    /// Generate an, optionally unique, object ID for the given time.
    pub fn from_time(t: i64, unique: bool) -> Self {
        let counter = if unique {
            // We don't care that there is an ordering between threads, just
            // that each ID gets a unique value.
            let counter = COUNTER.fetch_add(1, Ordering::Relaxed) & COUNTER_MAX;
            let bs = counter.to_be_bytes();
            [bs[1], bs[2], bs[3]]
        } else {
            [0, 0, 0]
        };

        // Ensure the timestamp is 4 bytes. For timestamps far in the future
        // this will truncate them but it's how the spec is defined. Note: if
        // `t` is bigger than u32::MAX, the value will be truncated. This is
        // expected.
        let timestamp = (t as u32).to_be_bytes();
        let machine_id = *MACHINE_ID;

        Self {
            timestamp,
            machine_id,
            counter,
        }
    }

    /// Convert an object ID to its raw bytes.
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bs = [0u8; 12];
        bs[0..4].copy_from_slice(&self.timestamp);
        bs[4..9].copy_from_slice(&self.machine_id);
        bs[9..12].copy_from_slice(&self.counter);
        bs
    }

    pub fn timestamp(&self) -> i64 {
        u32::from_be_bytes(self.timestamp) as i64
    }

    pub fn machine_id(&self) -> u64 {
        let mut padded = [0u8; 8];
        padded[3..8].copy_from_slice(&self.machine_id);
        u64::from_be_bytes(padded)
    }

    pub fn counter(&self) -> u32 {
        u32::from_be_bytes([0, self.counter[0], self.counter[1], self.counter[2]])
    }
}

impl TryFrom<String> for ObjectId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bs = hex::decode(value)?;
        if bs.len() != 12 {
            return Err(Error::InvalidHexIdLength(bs.len()));
        }
        let timestamp = [bs[0], bs[1], bs[2], bs[3]];
        let machine_id = [bs[4], bs[5], bs[6], bs[7], bs[8]];
        let counter = [bs[9], bs[10], bs[11]];
        Ok(Self {
            timestamp,
            machine_id,
            counter,
        })
    }
}

impl PartialOrd for ObjectId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::Ordering,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{ObjectId, COUNTER, COUNTER_MAX};

    #[test]
    fn from_time_truncates_timestamp() {
        let t = i64::MAX;
        let id = ObjectId::from_time(t, false);
        assert_eq!(id.timestamp(), u32::MAX as i64);
    }

    #[test]
    fn eq_and_ord_agree() {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let id1 = ObjectId::from_time(t, false);
        let id2 = ObjectId::from_time(t, false);
        assert_eq!(id1, id2);
        assert_eq!(id1.cmp(&id2), std::cmp::Ordering::Equal);

        let id = ObjectId::new();
        assert_eq!(id, id);
        assert_eq!(id.cmp(&id), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_counter_overflow() {
        COUNTER.store(COUNTER_MAX, Ordering::Relaxed);

        let first = ObjectId::from_time(0, true);
        let second = ObjectId::from_time(0, true);
        let third = ObjectId::from_time(0, true);
        let fourth = ObjectId::from_time(0, true);

        assert_eq!(
            first.counter(),
            COUNTER_MAX,
            "first ID should have counter {COUNTER_MAX}"
        );
        assert_eq!(second.counter(), 0, "second ID should have counter 0");
        assert_eq!(third.counter(), 1, "third ID should have counter 1");
        assert_eq!(fourth.counter(), 2, "fourth ID should have counter 2");
    }
}

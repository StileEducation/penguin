use std::{
    sync::{
        LazyLock,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(rand::random()));

static MACHINE_ID: LazyLock<[u8; 5]> = LazyLock::new(|| {
    let mut machine_id = [0; 5];
    rand::fill(&mut machine_id);
    machine_id
});

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid hex string: {0}")]
    InvalidHexString(#[from] hex::FromHexError),
    #[error("invalid hex ID must be 12 bytes, got {0}")]
    InvalidHexIdLength(usize),
}

#[derive(Debug)]
pub struct ObjectId {
    timestamp: [u8; 4],
    machine_id: [u8; 5],
    counter: [u8; 3],
}

impl ObjectId {
    pub fn new() -> Self {
        Self::from_time(SystemTime::now(), true)
    }

    pub fn from_time(t: SystemTime, unique: bool) -> Self {
        let counter = if unique {
            let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
            let bs = counter.to_be_bytes();
            [bs[1], bs[2], bs[3]]
        } else {
            [0, 0, 0]
        };
        let timestamp = (t
            .duration_since(UNIX_EPOCH)
            .expect("time is after unix epoch; that is impossible")
            .as_secs() as u32)
            .to_be_bytes();
        let machine_id = *MACHINE_ID;

        Self {
            timestamp,
            machine_id,
            counter,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bs = [0u8; 12];
        bs[0..4].copy_from_slice(&self.timestamp);
        bs[4..9].copy_from_slice(&self.machine_id);
        bs[9..12].copy_from_slice(&self.counter);
        bs
    }

    pub fn timestamp(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(u32::from_be_bytes(self.timestamp) as u64)
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

impl ToString for ObjectId {
    fn to_string(&self) -> String {
        let mut bs: [u8; 12] = [0; 12];
        bs[0..4].copy_from_slice(&self.timestamp);
        bs[4..9].copy_from_slice(&self.machine_id);
        bs[9..12].copy_from_slice(&self.counter);
        hex::encode(bs)
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

impl From<ObjectId> for u128 {
    fn from(value: ObjectId) -> Self {
        let bs = value.to_bytes();
        let mut padded_bs = [0u8; 16];
        padded_bs[0..12].copy_from_slice(&bs);
        u128::from_be_bytes(padded_bs)
    }
}

impl PartialEq for ObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for ObjectId {}

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

use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use magnus::{function, method, prelude::*, Error, Ruby};

static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(rand::random()));

static MACHINE_ID: LazyLock<[u8; 5]> = LazyLock::new(|| {
    let mut machine_id = [0; 5];
    rand::fill(&mut machine_id);
    machine_id
});

#[derive(Debug)]
#[magnus::wrap(class = "Penguin::ObjectId")]
struct ObjectId {
    timestamp: [u8; 4],
    machine_id: [u8; 5],
    counter: [u8; 3],
}

impl ObjectId {
    fn generate() -> Result<Self, Error> {
        Self::generate_from_time(SystemTime::now(), true)
    }

    fn generate_from_time(t: SystemTime, unique: bool) -> Result<Self, Error> {
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

        Ok(Self {
            timestamp,
            machine_id,
            counter,
        })
    }

    fn from_string(ruby: &Ruby, s: String) -> Result<Self, Error> {
        let bs =
            hex::decode(s).map_err(|e| Error::new(ruby.exception_arg_error(), e.to_string()))?;
        // TODO: Validate length
        let timestamp = [bs[0], bs[1], bs[2], bs[3]];
        let machine_id = [bs[4], bs[5], bs[6], bs[7], bs[8]];
        let counter = [bs[9], bs[10], bs[11]];
        Ok(Self {
            timestamp,
            machine_id,
            counter,
        })
    }

    fn to_s(&self) -> String {
        self.to_string()
    }

    fn to_bytes(&self) -> [u8; 12] {
        let mut bs = [0u8; 12];
        bs[0..4].copy_from_slice(&self.timestamp);
        bs[4..9].copy_from_slice(&self.machine_id);
        bs[9..12].copy_from_slice(&self.counter);
        bs
    }

    fn to_i(&self) -> u128 {
        let bs = self.to_bytes();
        let mut padded_bs = [0u8; 16];
        padded_bs[0..12].copy_from_slice(&bs);
        u128::from_be_bytes(padded_bs)
    }

    fn timestamp(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(u32::from_be_bytes(self.timestamp) as u64)
    }

    fn machine_id(&self) -> u64 {
        let mut padded = [0u8; 8];
        padded[3..8].copy_from_slice(&self.machine_id);
        u64::from_be_bytes(padded)
    }

    fn counter(&self) -> u32 {
        u32::from_be_bytes([0, self.counter[0], self.counter[1], self.counter[2]])
    }

    fn compare(&self, other: &Self) -> i8 {
        self.to_bytes().cmp(&other.to_bytes()) as i8
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

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let penguin_module = ruby.define_module("Penguin")?;
    let object_id_class = penguin_module.define_class("ObjectId", ruby.class_object())?;
    object_id_class.define_singleton_method("generate", function!(ObjectId::generate, 0))?;
    object_id_class.define_singleton_method(
        "generate_from_time",
        function!(ObjectId::generate_from_time, 2),
    )?;
    object_id_class.define_singleton_method("from_string", function!(ObjectId::from_string, 1))?;

    object_id_class.define_method("to_s", method!(ObjectId::to_s, 0))?;
    object_id_class.define_method("to_string", method!(ObjectId::to_s, 0))?;
    object_id_class.define_method("to_i", method!(ObjectId::to_i, 0))?;

    object_id_class.define_method("to_time", method!(ObjectId::timestamp, 0))?;
    object_id_class.define_method("timestamp", method!(ObjectId::timestamp, 0))?;
    object_id_class.define_method("machine_id", method!(ObjectId::machine_id, 0))?;
    object_id_class.define_method("counter", method!(ObjectId::counter, 0))?;

    object_id_class.define_method("==", method!(ObjectId::eq, 1))?;
    object_id_class.define_method("<=>", method!(ObjectId::compare, 1))?;

    Ok(())
}

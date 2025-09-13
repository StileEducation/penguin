use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use magnus::{function, method, prelude::*, Error, RHash, Ruby, Symbol};

static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(rand::random()));

static MACHINE_ID: LazyLock<[u8; 5]> = LazyLock::new(|| {
    let mut machine_id = [0; 5];
    rand::fill(&mut machine_id);
    machine_id
});

#[derive(Debug)]
#[magnus::wrap(class = "BSON::ObjectId")]
struct ObjectId {
    timestamp: [u8; 4],
    machine_id: [u8; 5],
    counter: [u8; 3],
}

impl ObjectId {
    fn new(ruby: &Ruby) -> Result<Self, Error> {
        let kwargs = ruby.hash_new_capa(1);
        kwargs
            .aset(ruby.to_symbol("unique"), true)
            .expect("failed to insert into hash we just created");
        Self::from_time(ruby, SystemTime::now(), kwargs)
    }

    fn from_time(ruby: &Ruby, t: SystemTime, kwargs: RHash) -> Result<Self, Error> {
        log::debug!("Generating ID from time: t={t:?}, kwargs={kwargs:?}");
        let unique = {
            let v = kwargs.get(ruby.to_symbol("unique"));
            if let Some(v) = v {
                log::debug!(
                    "Found unique in kwargs, converting to bool: {v:?}, as bool: {}",
                    v.to_bool()
                );
                v.to_bool()
            } else {
                log::debug!("No unique in kwargs, defaulting to false");
                false
            }
        };

        let counter = if unique {
            let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
            log::debug!("Generating unique ID via counter: counter={counter}");
            let bs = counter.to_be_bytes();
            [bs[1], bs[2], bs[3]]
        } else {
            log::debug!("Generating non-unique ID");
            [0, 0, 0]
        };
        let timestamp = (t
            .duration_since(UNIX_EPOCH)
            .expect("time is after unix epoch; that is impossible")
            .as_secs() as u32)
            .to_be_bytes();
        let machine_id = *MACHINE_ID;

        log::debug!(
            "Generated ID: timestamp={:?}, machine_id={:?}, counter={:?}",
            timestamp,
            machine_id,
            counter
        );
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

    let bson_module = ruby.define_module("BSON")?;
    let object_id_class = bson_module.define_class("ObjectId", ruby.class_object())?;
    object_id_class.define_singleton_method("new", function!(ObjectId::new, 0))?;
    object_id_class.define_singleton_method("from_time", function!(ObjectId::from_time, 2))?;
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

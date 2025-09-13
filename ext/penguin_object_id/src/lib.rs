use std::time::{Duration, SystemTime};

use magnus::{function, method, prelude::*, Error, Ruby, Time};

#[derive(Debug)]
#[magnus::wrap(class = "Penguin::ObjectId")]
struct ObjectId(object_id::ObjectId);

impl ObjectId {
    fn generate() -> Self {
        Self(object_id::ObjectId::new())
    }

    fn generate_from_time(t: Time, unique: bool) -> Self {
        Self(object_id::ObjectId::from_time(
            t.timespec().unwrap().tv_sec,
            unique,
        ))
    }

    fn from_string(ruby: &Ruby, s: String) -> Result<Self, Error> {
        Ok(Self(object_id::ObjectId::try_from(s).map_err(|e| {
            Error::new(ruby.exception_arg_error(), e.to_string())
        })?))
    }

    fn to_s(&self) -> String {
        self.0.to_string()
    }

    fn to_bytes(&self) -> [u8; 12] {
        self.0.to_bytes()
    }

    fn to_i(&self) -> u128 {
        let bs = self.to_bytes();
        let mut padded_bs = [0u8; 16];
        padded_bs[0..12].copy_from_slice(&bs);
        u128::from_be_bytes(padded_bs)
    }

    fn timestamp(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(self.0.timestamp() as u64)
    }

    fn machine_id(&self) -> u64 {
        self.0.machine_id()
    }

    fn counter(&self) -> u32 {
        self.0.counter()
    }

    fn compare(&self, other: &Self) -> i8 {
        self.0.cmp(&other.0) as i8
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let penguin_module = ruby.define_module("Penguin")?;
    let object_id_class = penguin_module.define_class("ObjectId", ruby.class_object())?;
    object_id_class.define_singleton_method("generate", function!(ObjectId::generate, 0))?;
    object_id_class.define_singleton_method("new", function!(ObjectId::generate, 0))?;
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

use std::time::{Duration, SystemTime};

use magnus::{function, method, prelude::*, Error, RString, Ruby, Time};

#[derive(Debug)]
#[magnus::wrap(class = "Penguin::ObjectId", size, free_immediately, frozen_shareable)]
struct ObjectId(object_id::ObjectId);

impl ObjectId {
    /// Generate a new, unique, object ID for the current time.
    fn generate() -> Self {
        Self(object_id::ObjectId::new())
    }

    /// Generate an, optionally unique, object ID for the given time.
    ///
    /// You generally don't want to use this function directly. Instead, use
    /// `Penguin::ObjectId.from_time` which provides a nicer interface with
    /// kwargs.
    fn generate_from_time(t: Time, unique: bool) -> Self {
        Self(object_id::ObjectId::from_time(
            t.timespec().unwrap().tv_sec,
            unique,
        ))
    }

    /// Parse an object ID from its hexadecimal representation.
    fn from_string(ruby: &Ruby, s: String) -> Result<Self, Error> {
        Ok(Self(object_id::ObjectId::try_from(s).map_err(|e| {
            Error::new(ruby.exception_arg_error(), e.to_string())
        })?))
    }

    /// Convert an object ID to its hexadecimal representation.
    fn to_s(ruby: &Ruby, obj: &Self) -> RString {
        let mut bs = [0u8; 24];
        hex::encode_to_slice(obj.0.to_bytes(), &mut bs)
            .expect("encoding should always succeed for 12-byte input");
        ruby.str_from_slice(&bs)
    }

    /// Get the timestamp component of an object ID.
    fn timestamp(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(self.0.timestamp() as u64)
    }

    /// Get the machine ID component of an object ID.
    fn machine_id(&self) -> u64 {
        self.0.machine_id()
    }

    /// Get the counter component of an object ID.
    fn counter(&self) -> u32 {
        self.0.counter()
    }

    /// Compare two object IDs.
    fn compare(&self, other: &Self) -> i8 {
        self.0.cmp(&other.0) as i8
    }

    /// Check if two object IDs are equal.
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
    object_id_class.define_method("to_str", method!(ObjectId::to_s, 0))?;

    object_id_class.define_method("to_time", method!(ObjectId::timestamp, 0))?;
    object_id_class.define_method("timestamp", method!(ObjectId::timestamp, 0))?;
    object_id_class.define_method("machine_id", method!(ObjectId::machine_id, 0))?;
    object_id_class.define_method("counter", method!(ObjectId::counter, 0))?;

    // With these two methods, we can just `include Comparable` in the Ruby and
    // get all the other comparison methods for free.
    object_id_class.define_method("==", method!(ObjectId::eq, 1))?;
    object_id_class.define_method("<=>", method!(ObjectId::compare, 1))?;

    Ok(())
}

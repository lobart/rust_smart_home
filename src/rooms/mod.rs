use crate::devices::device::Device;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;

pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Box<dyn Device>>,
}

impl Serialize for Room {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Room", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("devices", &self.devices)?;
        state.end()
    }
}

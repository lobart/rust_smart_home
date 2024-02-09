use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::smartthermometer::SmartThermometer;
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

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            devices: HashMap::<String, Box<dyn Device>>::new(),
        }
    }

    pub fn remove_device_by_id(&mut self, id: &String) {
        self.devices.remove_entry(id);
    }

    pub fn get_device_id(&self, name: &String) -> String {
        for (k, v) in self.devices.iter() {
            if v.get_name().unwrap() == *name {
                return String::from(k);
            }
        }
        String::from("")
    }

    pub fn remove_device_by_name(&mut self, name: &String) {
        let id = self.get_device_id(name);
        self.devices.remove_entry(&id);
    }

    pub fn add_device(&mut self, id: &String, name: String, description: String, typ: String) {
        let device: Box<dyn Device> = if typ == "SS" {
            Box::new(SmartSocket {
                name,
                description,
                ..Default::default()
            })
        } else {
            Box::new(SmartThermometer {
                name,
                description,
                ..Default::default()
            })
        };
        self.devices.insert(id.to_string(), device);
    }
}

#[cfg(test)]
mod tests {
    use super::Room;

    #[test]
    fn test_creation_removing() {
        let name1: String = "test1".to_string();
        let description1: String = "test_description1".to_string();
        let typ1: String = "SS".to_string();

        let name2: String = "test2".to_string();
        let description2: String = "test_description2".to_string();
        let typ2: String = "SSS".to_string();

        let mut room: Room = Room::new("test_room".to_string());

        room.add_device(&String::from("t1"), name1, description1, typ1);
        room.add_device(
            &String::from("t2"),
            String::from(&name2),
            description2,
            typ2,
        );

        assert_eq!(room.devices.len(), 2);

        room.remove_device_by_id(&String::from("t1"));
        assert_eq!(room.devices.len(), 1);
        assert_eq!(
            room.devices
                .get(&String::from("t2"))
                .unwrap()
                .get_name()
                .unwrap(),
            String::from(&name2)
        );

        room.remove_device_by_name(&name2);
        assert_eq!(room.devices.len(), 0);
    }
}

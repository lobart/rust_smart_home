use crate::config::{DeviceConf, RoomConf, SmartHouseConf};
use crate::devices::device::Device;
use crate::devices::smartsocket::SmartSocket;
use crate::devices::smartthermometer::SmartThermometer;
use crate::rooms::Room;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SmartHouse<Room> {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl Serialize for SmartHouse<Room> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("SmartHouse", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("rooms", &self.rooms)?;
        state.end()
    }
}

impl SmartHouse<Room> {
    pub fn new(config: &SmartHouseConf<RoomConf<DeviceConf>>) -> Self {
        let mut sh = SmartHouse {
            name: String::from(&config.name),
            rooms: HashMap::new(),
        };
        for (k, v) in config.rooms.iter() {
            sh.rooms.insert(
                String::from(k),
                Room {
                    name: String::from(&v.name),
                    devices: HashMap::new(),
                },
            );
            for (n, d) in v.devices.iter() {
                let temp: Box<dyn Device> = if d.typ == "SS" {
                    Box::new(SmartSocket {
                        name: String::from(&d.name),
                        description: String::from(&d.description),
                        ..Default::default()
                    })
                } else {
                    Box::new(SmartThermometer {
                        name: String::from(&d.name),
                        description: String::from(&d.description),
                        ..Default::default()
                    })
                };
                sh.rooms
                    .get_mut(&String::from(k))
                    .unwrap()
                    .devices
                    .insert((&n).to_string(), temp);
            }
        }

        sh
    }

    pub fn create_report(&self) -> String {
        let mut report: String = String::new();
        report.push_str(&format!(
            "\nОтчет об устройствах умного дома {0}.\n\n\n",
            self.name
        ));
        for room in self.rooms.values() {
            report.push_str(&format!(
                "В комнате {0} установлены следующие приборы: \n",
                room.name
            ));
            for device in room.devices.values() {
                report.push_str(&((*device.get_report()).to_string() + &String::from("\n")));
                report.push_str(&(String::from("\n")));
            }
            report.push_str(&(String::from("\n\n")));
        }
        report
    }
}

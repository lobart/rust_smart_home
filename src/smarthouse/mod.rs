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

    pub fn get_list_rooms(&self) -> Result<Vec<String>, &'static str> {
        let mut res: Vec<String> = Vec::new();
        if self.rooms.is_empty() {
            Err("List of rooms is empty!")
        } else {
            for v in self.rooms.values() {
                res.push(String::from(&v.name));
            }
            Ok(res)
        }
    }

    pub fn get_list_devices(&self) -> Result<Vec<String>, &'static str> {
        let mut res: Vec<String> = Vec::new();

        for v in self.rooms.values() {
            let begin: String = String::from(&v.name);
            for d in v.devices.values() {
                res.push(format!("{0}_{1}", begin, d.get_name().unwrap()));
            }
        }
        if res.is_empty() {
            Err("List of devices is empty!")
        } else {
            Ok(res)
        }
    }

    pub fn create_report(&self) -> Result<String, &'static str> {
        let mut report: String = String::new();
        report.push_str(&format!(
            "\nОтчет об устройствах умного дома {0}.\n\n\n",
            self.name
        ));
        match self.get_list_rooms() {
            Err(_) => Err::<String, &str>("В доме нет комнат."),
            Ok(_) => {
                for room in self.rooms.values() {
                    report.push_str(&format!(
                        "В комнате {0} установлены следующие приборы: \n",
                        room.name
                    ));
                    for device in room.devices.values() {
                        report.push_str(
                            &((*device.get_report().unwrap()).to_string() + &String::from("\n")),
                        );
                        report.push_str(&(String::from("\n")));
                    }
                    report.push_str(&(String::from("\n\n")));
                }
                Ok(report)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, hash::Hash};

    fn set_eq<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    #[test]
    fn test_house() {
        let cfg = r#"
	{
	    "name": "NewHouse1",
	    "rooms": {
		"room1" : {
		"name" : "SuperRoom1",
		"devices" : {
			"dev1" : {
				"typ": "SS",
				"name": "SmartSocket",
				"description": "This is SmartSocket"
			},
			"dev2" : {
				"typ": "ST",
				"name": "SmartThermometer",
				"description": "This is SmartThermometer"
			}
		}
		},
                "room2" : {
                "name" : "SuperRoom2",
                "devices" : {
                        "dev1" : {
                                "typ": "SS",
                                "name": "SmartSocket",
                                "description": "This is SmartSocket"
                        },
                        "dev2" : {
                                "typ": "ST",
                                "name": "SmartThermometer",
                                "description": "This is SmartThermometer"
                        }
                }
                }       
	    }
	}"#;
        let test_house_conf: SmartHouseConf<RoomConf<DeviceConf>> =
            serde_json::from_str(cfg).unwrap();
        let test_house: SmartHouse<Room> = SmartHouse::new(&test_house_conf);
        assert_eq!(test_house.name, "NewHouse1");
        assert!( set_eq(
            &test_house.get_list_rooms().unwrap(),
            &["SuperRoom1".to_string(), "SuperRoom2".to_string()]
        ));
        assert!(set_eq(
            &test_house.get_list_devices().unwrap(),
            &[
                "SuperRoom2_SmartSocket".to_string(),
                "SuperRoom2_SmartThermometer".to_string(),
                "SuperRoom1_SmartThermometer".to_string(),
                "SuperRoom1_SmartSocket".to_string()
            ]
        ));
    }
}

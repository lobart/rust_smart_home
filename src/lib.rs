pub mod lib {
	#[warn(unused_imports)]
	use serde::Deserialize;
	use serde::ser::{Serialize, Serializer, SerializeStruct};
	use std::collections::HashMap;
	
	const TEMPERATURE: i32 = 25;
	const CONSUMPTION: i32 = 100;

	#[derive(Deserialize, Debug)]
	pub struct DeviceConf {
	    typ: String,
	    name: String,
	    description: String,
	}

	#[derive(Deserialize, Debug)]
	pub struct RoomConf<DeviceConf> {
	    name: String,
	    devices: HashMap<String, DeviceConf>,
	}

	#[derive(Deserialize, Debug)]
	pub struct SmartHouseConf<RoomConf> {
	    name: String,
	    rooms: HashMap<String, RoomConf>,
	}

	pub struct Room {
	    name: String,
	    devices: HashMap<String, Box<dyn Device>>,
	}

	#[derive(Debug)]
	pub struct SmartHouse<Room> {
	    name: String,
	    rooms: HashMap<String, Room>,
	}

	pub struct SmartThermometer {
	    name: String,
	    temperature: i32,
	    description: String,
	}

	impl Default for SmartThermometer {
	    fn default() -> SmartThermometer {
		SmartThermometer {
		    name: String::from("DefaultName ST"),
		    temperature: TEMPERATURE,
		    description: String::from("Default description"),
		}
	    }
	}

	impl Device for SmartThermometer {
	    fn print(&self) {
		println!("{0}", self.description);
	    }
	    fn print_state(&self) {
		println!("Temperature is {0} Celsius", self.temperature);
	    }
	    fn get_name(&self) -> String {
		String::from(&self.name)
	    }
	    fn get_report(&self) -> String {
		format!("Устройство умный термометр.\n
			Имя: {0}
			Параметры:
			Температура: {1}
			Описание: {2}",
			self.get_name(), self.temperature.to_string(), self.description)
	    }
	}

	pub trait Device {
	    fn get_name(&self) -> String;
	    fn print(&self);
	    fn print_state(&self);
	    fn get_report(&self) -> String;
	}

	pub struct SmartSocket {
	    name: String,
	    state: bool,
	    power_consumption: i32,
	    description: String,
	}

	impl Default for SmartSocket {
	    fn default() -> SmartSocket {
		SmartSocket {
		    name: String::from("DefaultName SS"),
		    state: true,
		    power_consumption: CONSUMPTION,
		    description: String::from("Default description"),
		}
	    }
	}

	impl Device for SmartSocket {
	    fn print(&self) {
		println!("{0}", self.description);
	    }
	    fn print_state(&self) {
		println!("Power consumption is: {0}", self.power_consumption)
	    }
	    fn get_name(&self) -> String {
		String::from(&self.name)
	    }
	    fn get_report(&self) -> String {
		format!("Устройство умная розетка.\n
			Имя: {0}
			Параметры:
			Состояние: {1}
			Потребление: {2}
			Описание: {3}",
			self.get_name(), self.state.to_string(), self.power_consumption.to_string(), self.description)
	    }
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

	impl Serialize for SmartSocket {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_struct("SmartSocket", 4)?;
                state.serialize_field("name", &self.name)?;
                state.serialize_field("state", &self.state)?;
		state.serialize_field("power_consumption", &self.power_consumption)?;
                state.serialize_field("description", &self.description)?;
                state.end()
            }
        }

        impl Serialize for SmartThermometer {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {   
                let mut state = serializer.serialize_struct("SmartThermometer", 3)?;
                state.serialize_field("name", &self.name)?;
                state.serialize_field("temperature", &self.temperature)?;
                state.serialize_field("description", &self.description)?;
                state.end()
            }
        }

	impl Serialize for dyn Device {
	    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		    let s = serializer.serialize_struct("???", 3)?;
	/*	    match s.typ {
		    "SS" => {
		s.serialize_field("name", &self.name)?;
                s.serialize_field("state", &self.state)?;
                s.serialize_field("power_consumption", &self.power_consumption)?;
                s.serialize_field("description", &self.description)?;
			},
		    "ST" => {
		s.serialize_field("name", &self.name)?;
                s.serialize_field("temperature", &self.temperature)?;
                s.serialize_field("description", &self.description)?;	
			}
		    } */
		    s.end()
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

	    fn _get_rooms(&self) -> Vec<String> {
		// Размер возвращаемого массива можно выбрать самостоятельно
		let mut res: Vec<String> = Vec::new();
		for (_, room) in self.rooms.iter() {
		    res.push(room.name.to_string())
		}
		res
	    }

	    fn _devices(&self, room_name: &str) -> Vec<String> {
		// Размер возвращаемого массива можно выбрать самостоятельно
		let mut res: Vec<String> = Vec::new();
		for room in self.rooms.values() {
		    if room.name != room_name {
			continue;
		    };
		    for device in room.devices.values() {
			res.push(((*device).get_name()).to_string());
		    }
		}
		res
	    }

	    pub fn create_report(&self) -> String {
		let mut report: String = String::new();
		report.push_str(&format!("\nОтчет об устройствах умного дома {0}.\n\n\n", self.name));
		for room in self.rooms.values() {
		    report.push_str(&format!("В комнате {0} установлены следующие приборы: \n", room.name));
		    for device in room.devices.values() {
			report.push_str(&((*device.get_report()).to_string() + &String::from("\n")));
			report.push_str(&(String::from("\n")));
		    }
		    report.push_str(&(String::from("\n\n")));
		}
		report
	    }
	}


	#[cfg(test)]
	mod tests {
	    use super::*;

	    #[test]
	    fn empty_cfg() {
		let cfg = r#"
		{
		    "name": "New House 1",
		    "rooms": {
			"room" : {
			"name" : "SuperRoom",
			"devices" : {
				}
			}	
		}
		}"#;

		let mut house: SmartHouse<Room> = SmartHouse { name: "New House 1".to_string(), 
							   rooms: HashMap::new() };
		house.rooms.insert("room".to_string(), Room{ name: "SuperRoom".to_string(), devices: HashMap::new()});
		let house_ser = serde_json::to_string(&house).unwrap();
		//println!("Houses {:#?}", house_ser);
		let test_house: SmartHouse<Room> = SmartHouse::new(&(serde_json::from_str(cfg).unwrap()));
		let test_house_ser = serde_json::to_string(&test_house).unwrap();
		//println!("Test Houses {:#?}", test_house_ser);
		assert_eq!(house_ser, test_house_ser);
	    }
	}
}

pub mod lib {
	#[warn(unused_imports)]
	use serde::Deserialize;
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
}


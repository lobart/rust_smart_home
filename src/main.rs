// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым
#[warn(unused_imports)]
use serde::Deserialize;
use std::collections::HashMap;
const TEMPERATURE: i32 = 25;
const CONSUMPTION: i32 = 100;

#[derive(Deserialize, Debug)]
struct DeviceConf {
    typ: String,
    name: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct RoomConf<DeviceConf> {
    name: String,
    devices: HashMap<String, DeviceConf>,
}

#[derive(Deserialize, Debug)]
struct SmartHouseConf<RoomConf> {
    name: String,
    rooms: HashMap<String, RoomConf>,
}

struct Room {
    name: String,
    devices: HashMap<String, Box<dyn Device>>,
}

#[derive(Debug)]
struct SmartHouse<Room> {
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
        String::from("Устройство умный термометр. \nИмя: ")
            + &self.get_name()
            + &String::from("\n")
            + &String::from("Параметры: \n")
            + &String::from("Температура: ")
            + &String::from(&self.temperature.to_string())
            + &String::from("\n")
            + &String::from("Описание: ")
            + &String::from(&self.description)
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
        String::from("Устройство умная розетка. \nИмя: ")
            + &self.get_name()
            + &String::from("\n")
            + &String::from("Параметры: \n")
            + &String::from("Состояние: ")
            + &String::from(&self.state.to_string())
            + &String::from("\n")
            + &String::from("Потребление: ")
            + &String::from(&self.power_consumption.to_string())
            + &String::from("\n")
            + &String::from("Описание: ")
            + &String::from(&self.description)
    }
}

impl SmartHouse<Room> {
    fn new(config: &SmartHouseConf<RoomConf<DeviceConf>>) -> Self {
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

    fn create_report(&self) -> String {
        let mut report: String = String::new();
        report.push_str(
            &(String::from("\nОтчет об устройствах умного дома ")
                + &self.name
                + &String::from(".\n\n\n")),
        );
        for room in self.rooms.values() {
            report.push_str(
                &(String::from("В комнате ")
                    + &room.name
                    + &String::from(" установлены следующие приборы: \n")),
            );
            for device in room.devices.values() {
                report.push_str(&((*device.get_report()).to_string() + &String::from("\n")));
                report.push_str(&(String::from("\n")));
            }
            report.push_str(&(String::from("\n\n")));
        }
        report
    }
}

fn main() -> serde_json::Result<()> {
    let config = r#"
        {
            "name": "New House 1",
            "rooms": {
                "room1": {
                    "name": "Room 1",
                    "devices": {
                        "dev1": {
                            "name": "SmartSocket 1",
                            "typ": "SS",
                            "description": "This is SmartSocket"
                        },
                        "dev2": {
                            "name": "SmartThermometer 1",
                            "typ": "ST",
                            "description": "This is SmartThermometer"
                        }
                    }
                },
                "room2": { 
                    "name": "Room 2",
                    "devices": {
                        "dev3": {
                            "name": "SmartSocket 2",
                            "typ": "SS",
                            "description": "This is SmartSocket"
                        },
                        "dev4": {
                            "name": "SmartThermometer 2",
                            "typ": "ST",
                            "description": "This is SmartThermometer"
                        }
                    } 
                }
            }
        }"#;
    let config_map: SmartHouseConf<RoomConf<DeviceConf>> = serde_json::from_str(config)?;
    let sh: SmartHouse<Room> = SmartHouse::new(&config_map);
    let rep: String = sh.create_report();
    println!("{0}", rep);
    Ok(())
}

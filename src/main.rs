
use homework_3::lib::{SmartHouse, Room, SmartHouseConf, RoomConf, DeviceConf};

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

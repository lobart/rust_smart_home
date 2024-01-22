use rust_smart_house::config::{DeviceConf, RoomConf, SmartHouseConf};
use rust_smart_house::rooms::Room;
use rust_smart_house::smarthouse::SmartHouse;
use std::collections::HashMap;

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

        let mut house: SmartHouse<Room> = SmartHouse {
            name: "New House 1".to_string(),
            rooms: HashMap::new(),
        };
        house.rooms.insert(
            "room".to_string(),
            Room {
                name: "SuperRoom".to_string(),
                devices: HashMap::new(),
            },
        );
        let house_ser = serde_json::to_string(&house).unwrap();
        let test_house: SmartHouse<Room> = SmartHouse::new(&(serde_json::from_str(cfg).unwrap()));
        let test_house_ser = serde_json::to_string(&test_house).unwrap();
        assert_eq!(house_ser, test_house_ser);
    }
    #[test]
    fn config_test() {
        let cfg = r#"
		{
		    "name": "New House 1",
		    "rooms": {
			"room" : {
			"name" : "SuperRoom",
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
        let test_house: SmartHouseConf<RoomConf<DeviceConf>> = serde_json::from_str(cfg).unwrap();
        assert_eq!(test_house.name, "New House 1");
    }
}

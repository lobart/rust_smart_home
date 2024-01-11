use crate::devices::constants::CONSUMPTION;
use crate::devices::device::Device;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct SmartSocket {
    pub name: String,
    pub state: bool,
    pub power_consumption: i32,
    pub description: String,
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
        format!(
            "Устройство умная розетка.\n
		Имя: {0}
		Параметры:
		Состояние: {1}
		Потребление: {2}
		Описание: {3}",
            self.get_name(),
            self.state,
            self.power_consumption,
            self.description
        )
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

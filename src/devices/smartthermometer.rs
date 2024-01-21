use crate::devices::constants::TEMPERATURE;
use crate::devices::device::Device;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct SmartThermometer {
    pub name: String,
    pub temperature: i32,
    pub description: String,
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
    fn get_name(&self) -> Result<String, &'static str> {
        if self.name.is_empty() {
            Err("Name is empty!!!")
        } else {
            Ok(String::from(&self.name))
        }
    }
    fn get_report(&self) -> Result<String, &'static str> {
        let name = match self.get_name() {
            Ok(name) => name,
            Err(err) => panic!("{0}", err),
        };
        Ok(format!(
            "Устройство умный термометр.\n
		Имя: {0}
		Параметры:
		Температура: {1}
		Описание: {2}",
            name, self.temperature, self.description
        ))
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

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub trait Device {
    fn get_name(&self) -> String;
    fn print(&self);
    fn print_state(&self);
    fn get_report(&self) -> String;
}

impl Serialize for dyn Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serializer.serialize_struct("???", 3)?;
        /*          match s.typ {
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

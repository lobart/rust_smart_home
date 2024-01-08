#[warn(unused_imports)]
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct DeviceConf {
    pub typ: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct RoomConf<DeviceConf> {
    pub name: String,
    pub devices: HashMap<String, DeviceConf>,
}

#[derive(Deserialize, Debug)]
pub struct SmartHouseConf<RoomConf> {
    pub name: String,
    pub rooms: HashMap<String, RoomConf>,
}

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::Path;

use configparser::ini::Ini;
use prost::Message;

use googleplay_protobuf::{AndroidBuildProto, AndroidCheckinProto, DeviceConfigurationProto, DeviceFeature};

use bincode::{Encode, Decode};
include!("src/snippets/device_properties.rs");

fn main() {
    if !Path::new("src/device_properties.bin").exists() {
        let mut config = Ini::new();
        config
            .read(fs::read_to_string("device.properties").unwrap())
            .unwrap();

        let mut device_properties_map = HashMap::new();
        for section in config.sections() {
            let device_properties_encoded = DeviceProperties::parse(&config, &section).to_encoded();
            device_properties_map.insert(section, device_properties_encoded);
        }

        let devices_encoded: Vec<u8> = bincode::encode_to_vec(&device_properties_map, bincode::config::standard()).unwrap();

        let mut file = File::create("src/device_properties.bin").unwrap();
        file.write_all(&devices_encoded).unwrap();
    }
}

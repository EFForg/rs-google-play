#[derive(Encode, Decode,Debug)]
struct EncodedDeviceProperties {
    pub device_configuration: Vec<u8>,
    pub android_checkin: Vec<u8>,
    pub extra_info: HashMap<String, String>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DeviceProperties {
    pub device_configuration: DeviceConfigurationProto,
    pub android_checkin: AndroidCheckinProto,
    pub extra_info: HashMap<String, String>,
}

#[allow(dead_code)]
impl EncodedDeviceProperties {
    pub fn new(
        device_configuration: Vec<u8>,
        android_checkin: Vec<u8>,
        extra_info: HashMap<String, String>
    ) -> Self {
        Self {
            device_configuration,
            android_checkin,
            extra_info,
        }
    }

    pub fn to_decoded(self) -> DeviceProperties {
        DeviceProperties {
            device_configuration: DeviceConfigurationProto::decode(
                &mut Cursor::new(&self.device_configuration.clone())
            ).unwrap(),
            android_checkin: AndroidCheckinProto::decode(
                &mut Cursor::new(&self.android_checkin.clone())
            ).unwrap(),
            extra_info: self.extra_info,
        }
    }
}

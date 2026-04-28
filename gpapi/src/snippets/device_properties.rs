#[derive(Encode, Decode,Debug)]
struct EncodedDeviceProperties {
    pub device_configuration: Vec<u8>,
    pub android_checkin: Vec<u8>,
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

#[derive(Debug)]
#[allow(dead_code)]
struct DeviceProperties {
    pub device_configuration: DeviceConfigurationProto,
    pub android_checkin: AndroidCheckinProto,
    pub extra_info: HashMap<String, String>,
}

#[allow(dead_code)]
impl DeviceProperties {
    pub fn new(
        device_configuration: DeviceConfigurationProto,
        android_checkin: AndroidCheckinProto,
        extra_info: HashMap<String, String>
    ) -> Self {
        Self {
            device_configuration,
            android_checkin,
            extra_info,
        }
    }

    pub fn parse(config: &Ini, section: &str) -> Self {
        let mut extra_info = HashMap::new();
        extra_info.insert("Build.ID".to_string(), config.get(section, "Build.ID").unwrap_or_default());
        extra_info.insert("Vending.versionString".to_string(), config.get(section, "Vending.versionString").unwrap_or_default());
        extra_info.insert("Vending.version".to_string(), config.get(section, "Vending.version").unwrap_or_default());
        extra_info.insert("Build.VERSION.RELEASE".to_string(), config.get(section, "Build.VERSION.RELEASE").unwrap_or_default());
        if let Some(sim_operator) = config.get(section, "SimOperator") {
            extra_info.insert("SimOperator".to_string(), sim_operator);
        }
        let mut android_build = AndroidBuildProto::default();
        android_build.id = config.get(section, "Build.FINGERPRINT");
        android_build.product = config.get(section, "Build.HARDWARE");
        android_build.carrier = config.get(section, "Build.BRAND");
        android_build.radio = config.get(section, "Build.RADIO");
        android_build.bootloader = config.get(section, "Build.BOOTLOADER");
        android_build.device = config.get(section, "Build.DEVICE");
        android_build.sdk_version = config
            .getint(section, "Build.VERSION.SDK_INT")
            .unwrap()
            .map(|v| v as i32);
        android_build.model = config.get(section, "Build.MODEL");
        android_build.manufacturer = config.get(section, "Build.MANUFACTURER");
        android_build.build_product = config.get(section, "Build.PRODUCT");
        android_build.client = config.get(section, "Client");
        android_build.ota_installed = Some(false);
        android_build.google_services = config
            .getint(section, "GSF.version")
            .unwrap()
            .map(|v| v as i32);
        let mut android_checkin = AndroidCheckinProto::default();
        android_checkin.build = Some(android_build);
        android_checkin.last_checkin_msec = Some(0);
        android_checkin.cell_operator =
            config.get(section, "CellOperator");
        android_checkin.sim_operator = config.get(section, "SimOperator");
        android_checkin.roaming = config.get(section, "Roaming");
        android_checkin.user_number = Some(0);

        let mut device_configuration = DeviceConfigurationProto::default();
        device_configuration.touch_screen = config
            .getint(section, "TouchScreen")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.keyboard = config
            .getint(section, "Keyboard")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.navigation = config
            .getint(section, "Navigation")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.screen_layout = config
            .getint(section, "ScreenLayout")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.has_hard_keyboard =
            config.getbool(section, "HasHardKeyboard").unwrap();
        device_configuration.has_five_way_navigation =
            config.getbool(section, "HasFiveWayNavigation").unwrap();
        device_configuration.screen_density = config
            .getint(section, "Screen.Density")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.gl_es_version = config
            .getint(section, "GL.Version")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.system_shared_library = config
            .get(section, "SharedLibraries")
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        device_configuration.system_available_feature = config
            .get(section, "Features")
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        device_configuration.native_platform = config
            .get(section, "Platforms")
            .unwrap_or_default()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        device_configuration.screen_width = config
            .getint(section, "Screen.Width")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.screen_height = config
            .getint(section, "Screen.Height")
            .unwrap()
            .map(|v| v as i32);
        device_configuration.system_supported_locale = config
            .get(section, "Locales")
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        device_configuration.gl_extension = config
            .get(section, "GL.Extensions")
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        device_configuration.device_feature = config
            .get(section, "Features")
            .unwrap()
            .split(",")
            .map(|s| {
                let feature_name = String::from(s);
                let mut device_feature = DeviceFeature::default();
                device_feature.name = Some(feature_name);
                device_feature.value = Some(0);
                device_feature
            })
            .collect();

        Self::new(device_configuration, android_checkin, extra_info)
    }

    pub fn to_encoded(self) -> EncodedDeviceProperties {
        let mut device_configuration_encoded = Vec::new();
        device_configuration_encoded.reserve(self.device_configuration.encoded_len());
        self.device_configuration.encode(&mut device_configuration_encoded).unwrap();
        let mut android_checkin_encoded = Vec::new();
        android_checkin_encoded.reserve(self.android_checkin.encoded_len());
        self.android_checkin.encode(&mut android_checkin_encoded).unwrap();

        EncodedDeviceProperties {
            device_configuration: device_configuration_encoded,
            android_checkin: android_checkin_encoded,
            extra_info: self.extra_info,
        }
    }
}


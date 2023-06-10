use std::time::Duration;

const FAN_CURVE: &'static [u8] = &[55, 10, 60, 55, 65, 100]; //pairs temperature:fan%, order by temperature
const WAKE_UP_INTERVAL: u64 = 1500; //ms

pub struct Configuration {
    pub curve: Vec<u8>,
    pub interval: Duration
}

impl Configuration {

    #[cfg(not(feature = "configuration_file"))]
    pub fn load() -> Self {
        Self::default()
    }

    #[cfg(feature = "configuration_file")]
    pub fn load() -> Self {
        let data = load_yaml();
        if data.is_none() {
            eprintln!("Unable to load configuration file, fallback to build-in values.")
        }
        data.unwrap_or(Self::default())
    }

    fn default() -> Self {
        assert_eq!(FAN_CURVE.len() % 2, 0);
        Self {
            curve: FAN_CURVE.to_vec(),
            interval: Duration::from_millis(WAKE_UP_INTERVAL)
        }
    }
}

#[cfg(feature = "configuration_file")]
extern crate yaml_rust;
#[cfg(feature = "configuration_file")]
use yaml_rust::YamlLoader;
#[cfg(feature = "configuration_file")]
use std::fs;

#[cfg(feature = "configuration_file")]
fn load_yaml() -> Option<Configuration> {
    let file = fs::read_to_string("/etc/argon_fan_controller_cfg.yml").ok()?;
    let yaml = YamlLoader::load_from_str(&*file).ok()?;
    if yaml.len() != 1 {
        return None
    }
    let doc = &yaml[0];
    let interval = doc["interval"].as_i64()?;
    let curve_data = doc["fan"].as_vec()?;
    let curve: Vec<u8> = curve_data.into_iter().map(|e| e.as_i64().unwrap_or_default() as u8).collect();
    if curve.len() % 2 != 0 {
        return None
    }
    Some(Configuration {
        curve,
        interval: Duration::from_millis(interval as u64)
    })
}
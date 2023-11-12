pub struct Config {
    pub serial_rate: u32,
}

impl Config {
    pub fn default() -> Self {
        Self {
            serial_rate: 115_200,
        }
    }
}

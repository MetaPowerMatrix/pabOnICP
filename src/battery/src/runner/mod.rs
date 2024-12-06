#[derive(Debug)]
pub struct BatteryRunner {
    pub version: String,
    pub id: String,
    pub sleep_mode: bool,
    pub sn: i64,
}

impl BatteryRunner {
    pub fn new(id: String, sn: i64) -> Self {
        BatteryRunner {
            version: "0.1.0".to_string(),
            id,
            sleep_mode: false,
            sn,
        }
    }

    pub async fn run_loop(&mut self) {
        ic_cdk::println!("battery runner");
    }
}

use serde::{Deserialize, Serialize};

use super::{Address, Description};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Reading {
    power_meter_info: Option<PowerMeterInfo>,
    power_meter_state: PowerMeterEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PowerMeterInfo {
    Fixed(Address, PowerMeterNumber),
    Portable(Option<Description>, PowerMeterNumber),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PowerMeterEntry {
    start: PowerMeterState,
    end: Option<PowerMeterState>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
struct PowerMeterState {
    code_108: f64,
    code_104: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerMeterNumber;

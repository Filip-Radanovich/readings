#[derive(Default)]
pub struct Manifest<'a> {
    file: (
        Option<&'a std::path::Path>,
        Option<std::fs::File>
    ),
    entries: Option<Vec<ManifestEntry>>
}

#[derive(Default)]
pub struct ManifestEntry{
    location: String,
    date: (u32, u32, u32), // YEAR-MONTH-DAY
    power_meter_state: PowerMeterEntry,
}

#[derive(Default)]
struct PowerMeterEntry {
    start: Option<PowerMeterState>,
    end: Option<PowerMeterState>
}

struct PowerMeterState {
    code_108: f64,
    code_104:
}
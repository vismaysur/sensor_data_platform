use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use mock_edge_data::EdgeTelemetryTable;
use rusqlite::Connection;
use tabled::Table;

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Args {
    /// Limit query to listed sensors (defaults to all sensors)
    #[arg(short, long)]
    sensor_ids: Vec<u8>,

    /// Time range like "1h", "30m", "5s"
    #[arg(short, long, default_value_t = String::from("1h"))]
    last: String,
}

#[derive(Debug)]
struct UnsignedMillis(u64);

impl FromStr for UnsignedMillis {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit_idx = s
            .find(|c: char| !c.is_numeric())
            .ok_or_else(|| "Missing unit")?;

        let (num_part, unit) = s.split_at(unit_idx);

        let val: u64 = num_part.parse().map_err(|_| "Invalid number")?;

        let ms = match unit {
            "h" => val * 60 * 60 * 1000,
            "m" => val * 60 * 1000,
            "s" => val * 100,
            _ => return Err(format!("Unknown unit: {unit}")),
        };

        Ok(UnsignedMillis(ms))
    }
}

fn main() {
    let args = Args::parse();

    let table = EdgeTelemetryTable::new(
        Connection::open("edge_telemetry.db").expect("Underlying SQLite open call failed"),
    );

    let system_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime is before UNIX EPOCH")
        .as_millis() as i64;

    let time_range = args
        .last
        .parse::<UnsignedMillis>()
        .expect("Failed to parse time range");

    let results = table.get_readings((system_time - time_range.0 as i64, system_time));

    let results = Table::new(results);

    println!("{results}");
}

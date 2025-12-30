use clap::Parser;
use mock_edge_data::{EdgeTelemetryTable, SensorReading};
use rand::Rng;
use rusqlite::Connection;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const TEMP_MIN: f64 = 0.0;
const TEMP_MAX: f64 = 100.0;
const PRESSURE_MIN: f64 = 50.0;
const PRESSURE_MAX: f64 = 500.0;
const VIBRATION_MIN: f64 = 1.0;
const VIBRATION_MAX: f64 = 3000.0;
const READING_INTERVAL_MS: u64 = 50;

#[derive(Parser, Debug)]
struct Args {
    // ID of the sensor that will generate readings
    #[arg(short, long)]
    sensor_id: u8,
}

fn main() {
    let sensor_id = Args::parse().sensor_id as i32;

    let table = EdgeTelemetryTable::new(
        Connection::open("edge_telemetry.db").expect("Underlying SQLite open call failed"),
    )
    .expect("Failed to create edge telemetry table");

    let mut rng = rand::rng();

    loop {
        let sensor_reading = SensorReading::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime is before UNIX EPOCH")
                .as_millis() as i32,
            sensor_id,
            rng.random_range(TEMP_MIN..=TEMP_MAX),
            rng.random_range(PRESSURE_MIN..=PRESSURE_MAX),
            rng.random_range(VIBRATION_MIN..=VIBRATION_MAX),
        );

        table
            .insert_reading(sensor_reading)
            .expect("Failed to insert reading into table");

        thread::sleep(Duration::from_millis(READING_INTERVAL_MS));
    }
}

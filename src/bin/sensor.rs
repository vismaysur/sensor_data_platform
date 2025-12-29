use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::Rng;
use rusqlite::Connection;

use mock_edge_data::{EdgeTelemetryTable, SensorReading};

fn main() {
    let table = EdgeTelemetryTable::new(
        Connection::open("edge_telemetry.db").expect("Underlying SQLite open call failed"),
    );

    let mut rng = rand::rng();

    loop {
        let sensor_reading = SensorReading::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime is before UNIX EPOCH")
                .as_millis() as i64,
            rng.random_range(0.0..=100.0),
            rng.random_range(50.0..=500.0),
            rng.random_range(1.0..=3000.0),
        );

        table.insert(sensor_reading);

        thread::sleep(Duration::from_millis(50));
    }
}

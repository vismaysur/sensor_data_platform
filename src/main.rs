use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::Rng;
use rusqlite::Connection;

struct SensorReading {
    timestamp: i64,
    temperature: f64,
    pressure: f64,
    vibration: f64,
}

fn main() {
    let conn = Connection::open("edge_telemetry.db").expect("Underlying SQLite open call failed");

    conn.execute(
        "CREATE TABLE edge_telemetry (
            timestamp   BINARY(16) PRIMARY KEY,
            temperature DOUBLE,
            pressure    DOUBLE,
            vibration   DOUBLE
        )",
        (),
    )
    .expect("SQL could not be converted to C-compatible string / underlying SQLite call failed");

    let mut rng = rand::rng();

    loop {
        let sensor_reading = SensorReading {
            temperature: rng.random_range(0.0..=100.0),
            pressure: rng.random_range(50.0..=500.0),
            vibration: rng.random_range(1.0..=3000.0),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime is before UNIX EPOCH")
                .as_millis() as i64,
        };

        conn.execute(
            "INSERT INTO edge_telemetry (timestamp, temperature, pressure, vibration)
            VALUES (?1, ?2, ?3, ?4)",
            (
                &sensor_reading.timestamp,
                &sensor_reading.temperature,
                &sensor_reading.pressure,
                &sensor_reading.vibration,
            ),
        )
        .expect(
            "SQL could not be converted to C-compatible string / underlying SQLite call failed",
        );

        thread::sleep(Duration::from_millis(50));
    }
}

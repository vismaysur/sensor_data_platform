use tabled::Tabled;

#[derive(Tabled)]
pub struct SensorReading {
    pub timestamp: i64,
    pub sensor_id: i64,
    pub temperature: f64,
    pub pressure: f64,
    pub vibration: f64,
}

impl SensorReading {
    pub fn new(
        timestamp: i64,
        sensor_id: i64,
        temperature: f64,
        pressure: f64,
        vibration: f64,
    ) -> SensorReading {
        SensorReading {
            timestamp,
            sensor_id,
            temperature,
            pressure,
            vibration,
        }
    }
}

pub struct EdgeTelemetryTable {
    connection: rusqlite::Connection,
}

impl EdgeTelemetryTable {
    pub fn new(connection: rusqlite::Connection) -> EdgeTelemetryTable {
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS edge_telemetry (
                timestamp   BINARY(16) PRIMARY KEY,
                sensor_id   INT,
                temperature DOUBLE,
                pressure    DOUBLE,
                vibration   DOUBLE
                )",
                (),
            )
            .expect(
                "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            );

        EdgeTelemetryTable { connection }
    }

    pub fn insert_reading(&self, reading: SensorReading) {
        self.connection
            .execute(
                "INSERT INTO edge_telemetry (timestamp, sensor_id, temperature, pressure, vibration)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &reading.timestamp,
                    &reading.sensor_id,
                    &reading.temperature,
                    &reading.pressure,
                    &reading.vibration,
                ),
            )
            .expect(
                "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            );
    }

    pub fn get_readings(&self, time_range: (i64, i64)) -> Vec<SensorReading> {
        let mut readings = Vec::new();

        let mut stmt = self
            .connection
            .prepare(
                "SELECT * FROM edge_telemetry
                WHERE timestamp >= ?1 AND timestamp <= ?2",
            )
            .expect(
                "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            );

        let reading_iter = stmt
            .query_map([time_range.0, time_range.1], |row| {
                Ok(SensorReading {
                    timestamp: row.get(0).expect("Invalid column type/name/index"),
                    sensor_id: row.get(1).expect("Invalid column type/name/index"),
                    temperature: row.get(2).expect("Invalid column type/name/index"),
                    pressure: row.get(3).expect("Invalid column type/name/index"),
                    vibration: row.get(4).expect("Invalid column type/name/index"),
                })
            })
            .expect("Failed to bind parameters");

        for reading in reading_iter {
            readings.push(reading.expect("Failed to construct valid SensorReading"));
        }

        readings
    }
}

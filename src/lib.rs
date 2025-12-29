pub struct SensorReading {
    pub timestamp: i64,
    pub temperature: f64,
    pub pressure: f64,
    pub vibration: f64,
}

impl SensorReading {
    pub fn new(timestamp: i64, temperature: f64, pressure: f64, vibration: f64) -> SensorReading {
        SensorReading {
            timestamp,
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
                "CREATE TABLE edge_telemetry (
                timestamp   BINARY(16) PRIMARY KEY,
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

    pub fn insert(&self, reading: SensorReading) {
        self.connection
            .execute(
                "INSERT INTO edge_telemetry (timestamp, temperature, pressure, vibration)
            VALUES (?1, ?2, ?3, ?4)",
                (
                    &reading.timestamp,
                    &reading.temperature,
                    &reading.pressure,
                    &reading.vibration,
                ),
            )
            .expect(
                "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            );
    }
}

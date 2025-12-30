use std::vec;

use tabled::Tabled;

#[derive(Tabled)]
pub struct SensorReading {
    pub timestamp: i32,
    pub sensor_id: i32,
    pub temperature: f64,
    pub pressure: f64,
    pub vibration: f64,
}

impl SensorReading {
    pub fn new(
        timestamp: i32,
        sensor_id: i32,
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
    pub fn new(connection: rusqlite::Connection) -> Result<EdgeTelemetryTable, String> {
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS edge_telemetry (
                timestamp   INT PRIMARY KEY,
                sensor_id   INT,
                temperature DOUBLE,
                pressure    DOUBLE,
                vibration   DOUBLE
                )",
                (),
            )
            .map_err(
                |_| "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            )?;

        Ok(EdgeTelemetryTable { connection })
    }

    pub fn insert_reading(&self, reading: SensorReading) -> Result<(), String> {
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
            .map_err(
                |_| "SQL could not be converted to C-compatible string / underlying SQLite call failed",
            )?;

        Ok(())
    }

    pub fn get_readings(&self, time_range: (i32, i32)) -> Result<Vec<SensorReading>, String> {
        self.query(time_range, vec![])
    }

    pub fn get_readings_by_sensors(
        &self,
        time_range: (i32, i32),
        sensor_ids: Vec<i32>,
    ) -> Result<Vec<SensorReading>, String> {
        self.query(time_range, sensor_ids)
    }

    fn query(
        &self,
        time_range: (i32, i32),
        sensor_ids: Vec<i32>,
    ) -> Result<Vec<SensorReading>, String> {
        let sensor_filter = if sensor_ids.is_empty() {
            String::from("")
        } else {
            format!(
                "AND sensor_id IN ({})",
                sensor_ids
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        };

        println!("{sensor_filter}");

        let sql_query = format!(
            "SELECT * FROM edge_telemetry
                WHERE timestamp >= ?1 AND timestamp <= ?2 {}",
            sensor_filter
        );

        let mut stmt = self.connection.prepare(&sql_query).map_err(
            |_| "SQL could not be converted to C-compatible string / underlying SQLite call failed",
        )?;

        let reading_iter = stmt
            .query_map([time_range.0, time_range.1], |row| {
                Ok(SensorReading {
                    timestamp: row.get(0)?,
                    sensor_id: row.get(1)?,
                    temperature: row.get(2)?,
                    pressure: row.get(3)?,
                    vibration: row.get(4)?,
                })
            })
            .map_err(|_| "Failed to bind parameters")?;

        let readings = reading_iter
            .collect::<Result<Vec<SensorReading>, _>>()
            .map_err(|_| "Failed to construct valid SensorReading")?;

        Ok(readings)
    }
}

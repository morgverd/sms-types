//! GNSS position report types.

use serde::{Deserialize, Serialize};

/// GNSS (Global Navigation Satellite System) fix status.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FixStatus {
    /// GNSS fix status is unknown.
    Unknown,

    /// No GNSS fix.
    NotFix,

    /// 2D GNSS fix (latitude and longitude only).
    Fix2D,

    /// 3D GNSS fix (latitude, longitude, and altitude).
    Fix3D,
}
impl TryFrom<&str> for FixStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "Location Unknown" | "Unknown" => Ok(FixStatus::Unknown),
            "Location Not Fix" | "Not Fix" => Ok(FixStatus::NotFix),
            "Location 2D Fix" | "2D Fix" => Ok(FixStatus::Fix2D),
            "Location 3D Fix" | "3D Fix" => Ok(FixStatus::Fix3D),
            _ => Err(format!("Invalid GNSS fix status: '{value}'")),
        }
    }
}
impl From<u8> for FixStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => FixStatus::NotFix,
            1 => FixStatus::Fix2D,
            2 => FixStatus::Fix3D,
            _ => FixStatus::Unknown,
        }
    }
}

/// Represents a GNSS position report with optional fields for satellite info.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionReport {
    /// Indicates whether the GNSS receiver is currently running.
    pub run_status: bool,

    /// Whether a valid fix has been obtained.
    pub fix_status: bool,

    /// UTC time of the position report in ISO 8601 format.
    pub utc_time: String,

    /// Latitude in decimal degrees.
    pub latitude: Option<f64>,

    /// Longitude in decimal degrees.
    pub longitude: Option<f64>,

    /// Mean sea level altitude in meters.
    pub msl_altitude: Option<f64>,

    /// Ground speed in meters per second.
    pub ground_speed: Option<f32>,

    /// Ground course in degrees.
    pub ground_course: Option<f32>,

    /// Fix mode indicating 2D/3D fix or unknown.
    pub fix_mode: FixStatus,

    /// Horizontal Dilution of Precision.
    pub hdop: Option<f32>,

    /// Position Dilution of Precision.
    pub pdop: Option<f32>,

    /// Vertical Dilution of Precision.
    pub vdop: Option<f32>,

    /// Number of GPS satellites in view.
    pub gps_in_view: Option<u8>,

    /// Number of GNSS satellites used in the fix.
    pub gnss_used: Option<u8>,

    /// Number of GLONASS satellites in view.
    pub glonass_in_view: Option<u8>,
}
impl TryFrom<Vec<&str>> for PositionReport {
    type Error = String;

    fn try_from(fields: Vec<&str>) -> Result<Self, Self::Error> {
        if fields.len() < 15 {
            return Err(format!(
                "Insufficient GNSS data fields got {}",
                fields.len()
            ));
        }

        // Based on: https://simcom.ee/documents/SIM868/SIM868_GNSS_Application%20Note_V1.00.pdf (2.3)
        Ok(Self {
            run_status: fields[0] == "1",
            fix_status: fields[1] == "1",
            utc_time: fields[2].to_string(),
            latitude: fields[3].parse().ok(),
            longitude: fields[4].parse().ok(),
            msl_altitude: fields[5].parse().ok(),
            ground_speed: fields[6].parse().ok(),
            ground_course: fields[7].parse().ok(),
            fix_mode: FixStatus::from(fields[8].parse::<u8>().unwrap_or(0)),
            // Reserved1
            hdop: fields[10].parse().ok(),
            pdop: fields[11].parse().ok(),
            vdop: fields[12].parse().ok(),
            // Reserved2
            gps_in_view: fields[14].parse().ok(),
            gnss_used: fields[15].parse().ok(),
            glonass_in_view: fields[16].parse().ok(),
        })
    }
}
impl std::fmt::Display for PositionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn convert_opt<T: std::fmt::Display>(opt: Option<&T>) -> String {
            match opt {
                Some(value) => value.to_string(),
                None => "None".to_string(),
            }
        }

        write!(
            f,
            "Lat: {}, Lon: {}, Alt: {}, Speed: {}, Course: {}",
            convert_opt(self.latitude.as_ref()),
            convert_opt(self.longitude.as_ref()),
            convert_opt(self.msl_altitude.as_ref()),
            convert_opt(self.ground_speed.as_ref()),
            convert_opt(self.ground_course.as_ref())
        )
    }
}

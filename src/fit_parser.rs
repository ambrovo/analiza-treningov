use fitparser::{from_reader, Value};
use fitparser::profile::MesgNum;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use chrono::{DateTime, Local};

/// Strongly typed FIT record with optional values
#[derive(Debug)]
pub struct FitRecord {
    pub timestamp: Option<DateTime<Local>>,
    pub heart_rate: Option<u8>,
    pub cadence: Option<u8>,
    pub fractional_cadence: Option<f64>,
    pub distance: Option<f64>,
    pub power: Option<u16>,
    pub accumulated_power: Option<u32>,
    pub enhanced_altitude: Option<f64>,
    pub enhanced_respiration_rate: Option<f64>,
    pub enhanced_speed: Option<f64>,
    pub position_lat: Option<i32>,
    pub position_long: Option<i32>,
    pub temperature: Option<i8>,
    pub unknown_field_107: Option<u8>,
    pub unknown_field_134: Option<u8>,
    pub unknown_field_137: Option<u8>,
    pub unknown_field_138: Option<u8>,
    pub unknown_field_144: Option<u8>,
}

/// Parse FIT (or FIT.gz) file into Vec<FitRecord>
pub fn parse_fit_file(path: &str) -> Result<Vec<FitRecord>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader: Box<dyn std::io::Read> = if path.ends_with(".gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut reader = BufReader::new(reader);

    let mut records: Vec<FitRecord> = Vec::new();

    for record in from_reader(&mut reader)? {
        
        if record.kind() == MesgNum::Record {
            let mut rec = FitRecord {
          
                timestamp: None,
                heart_rate: None,
                cadence: None,
                fractional_cadence: None,
                distance: None,
                power: None,
                accumulated_power: None,
                enhanced_altitude: None,
                enhanced_respiration_rate: None,
                enhanced_speed: None,
                position_lat: None,
                position_long: None,
                temperature: None,
                unknown_field_107: None,
                unknown_field_134: None,
                unknown_field_137: None,
                unknown_field_138: None,
                unknown_field_144: None,
            };

            for field in record.fields() {
                match field.name() {
                    "timestamp" => if let Value::Timestamp(dt) = field.value() { rec.timestamp = Some(*dt); },
                    "heart_rate" => if let Value::UInt8(v) = field.value() { rec.heart_rate = Some(*v); },
                    "cadence" => if let Value::UInt8(v) = field.value() { rec.cadence = Some(*v); },
                    "fractional_cadence" => if let Value::Float64(v) = field.value() { rec.fractional_cadence = Some(*v); },
                    "distance" => if let Value::Float64(v) = field.value() { rec.distance = Some(*v); },
                    "power" => if let Value::UInt16(v) = field.value() { rec.power = Some(*v); },
                    "accumulated_power" => if let Value::UInt32(v) = field.value() { rec.accumulated_power = Some(*v); },
                    "enhanced_altitude" => if let Value::Float64(v) = field.value() { rec.enhanced_altitude = Some(*v); },
                    "enhanced_respiration_rate" => if let Value::Float64(v) = field.value() { rec.enhanced_respiration_rate = Some(*v); },
                    "enhanced_speed" => if let Value::Float64(v) = field.value() { rec.enhanced_speed = Some(*v); },
                    "position_lat" => if let Value::SInt32(v) = field.value() { rec.position_lat = Some(*v); },
                    "position_long" => if let Value::SInt32(v) = field.value() { rec.position_long = Some(*v); },
                    "temperature" => if let Value::SInt8(v) = field.value() { rec.temperature = Some(*v); },
                    "unknown_field_107" => if let Value::UInt8(v) = field.value() { rec.unknown_field_107 = Some(*v); },
                    "unknown_field_134" => if let Value::UInt8(v) = field.value() { rec.unknown_field_134 = Some(*v); },
                    "unknown_field_137" => if let Value::UInt8(v) = field.value() { rec.unknown_field_137 = Some(*v); },
                    "unknown_field_138" => if let Value::UInt8(v) = field.value() { rec.unknown_field_138 = Some(*v); },
                    "unknown_field_144" => if let Value::UInt8(v) = field.value() { rec.unknown_field_144 = Some(*v); },
                    _ => {}
                }

            }
   

            records.push(rec);
        }
    }

    Ok(records)
}
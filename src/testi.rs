use crate::averages_and_totals::FitData;
use crate::fit_parser::FitRecord;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_record(power: u16, accumulated: u32) -> FitRecord {
        FitRecord {
            timestamp: None,
            heart_rate: None,
            cadence: None,
            fractional_cadence: None,
            distance: None,
            power: Some(power),
            accumulated_power: Some(accumulated),
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
        }
    }

    #[test]
    fn test_total_work() {
        let records = vec![
            create_mock_record(100, 100),
            create_mock_record(200, 300),
            create_mock_record(300, 600),
        ];
        let fit_data = FitData{data: records};
        assert_eq!(fit_data.total_work(), 0);

        let records_large = vec![
            create_mock_record(1000, 1000),
            create_mock_record(1000, 2000),
        ];
        let fit_data_large = FitData{data: records_large};
        assert_eq!(fit_data_large.total_work(), 2);
    }

    #[test]
    fn test_normalized_power_steady() {
        
    }
}
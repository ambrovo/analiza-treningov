use crate::averages_and_totals::*;
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

    fn create_record_with_hr(power: u16, hr: u8, accumulated: u32) -> FitRecord {
        let mut r = create_mock_record(power, accumulated);
        r.heart_rate = Some(hr);
        r
    }

    fn create_record_with_altitude(power: u16, alt: f64, accumulated: u32) -> FitRecord {
        let mut r = create_mock_record(power, accumulated);
        r.enhanced_altitude = Some(alt);
        r
    }

    // --- total_work ---
    #[test]
    fn test_total_work() {
        let records = vec![
            create_mock_record(100, 100),
            create_mock_record(200, 300),
            create_mock_record(300, 600),
        ];
        // 100 + 200 + 300 = 600 J = 0 kJ (integer division 600/1000)
        assert_eq!(total_work(&records), 0);

        let records_large = vec![
            create_mock_record(1000, 1000),
            create_mock_record(1000, 2000),
        ];
        // 2000 J = 2 kJ
        assert_eq!(total_work(&records_large), 2);
    }

    // --- normalized_power ---
    #[test]
    fn test_normalized_power_steady() {
        // 60 seconds of steady 200W -> NP should be 200
        let records: Vec<FitRecord> = (0..60)
            .map(|i| create_mock_record(200, 200 * (i + 1)))
            .collect();
        assert_eq!(normalized_power(&records), 200);
    }

    #[test]
    fn test_normalized_power_too_short() {
        let records: Vec<FitRecord> = (0..10)
            .map(|i| create_mock_record(200, 200 * (i + 1)))
            .collect();
        assert_eq!(normalized_power(&records), 0);
    }

    // --- average_power_of / average_hr_of ---
    #[test]
    fn test_average_power() {
        let records = vec![
            create_mock_record(100, 100),
            create_mock_record(200, 300),
            create_mock_record(300, 600),
        ];
        let avg = average_power_of(&records);
        assert!((avg - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_average_hr() {
        let records = vec![
            create_record_with_hr(200, 120, 200),
            create_record_with_hr(200, 140, 400),
            create_record_with_hr(200, 160, 600),
        ];
        let avg = average_hr_of(&records);
        assert!((avg - 140.0).abs() < 0.01);
    }

    // --- aerobic_efficiency ---
    #[test]
    fn test_aerobic_efficiency() {
        let records = vec![
            create_record_with_hr(200, 100, 200),
            create_record_with_hr(200, 100, 400),
        ];
        let ae = aerobic_efficiency(&records);
        assert!((ae - 2.0).abs() < 0.01); // 200 / 100
    }

    // --- aerobic_decoupling ---
    #[test]
    fn test_aerobic_decoupling_no_drift() {
        // same power and HR in both halves -> 0% decoupling
        let records: Vec<FitRecord> = (0..100)
            .map(|i| create_record_with_hr(200, 150, 200 * (i + 1)))
            .collect();
        let dc = aerobic_decoupling(&records);
    
        assert!((dc - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_aerobic_decoupling_with_drift() {
        // first half: 200W @ 100bpm, second half: 200W @ 120bpm
        let mut records = Vec::new();
        for i in 0..50 {
            records.push(create_record_with_hr(200, 100, 200 * (i + 1)));
        }
        for i in 50..100 {
            records.push(create_record_with_hr(200, 120, 200 * (i + 1)));
        }
        let dc = aerobic_decoupling(&records);
        // EF1 = 200/100 = 2.0, EF2 = 200/120 = 1.667
        // decoupling = (2.0 - 1.667) / 2.0 * 100 = 16.67%
        assert!((dc - 16.67).abs() < 0.5);
    }

    // --- hr_drift_rate ---
    #[test]
    fn test_hr_drift_rate_flat() {
        // constant HR -> slope near 0
        let records: Vec<FitRecord> = (0..120)
            .map(|i| create_record_with_hr(200, 150, 200 * (i + 1)))
            .collect();
        let slope = hr_drift_rate(&records);
        assert!(slope.abs() < 0.01);
    }

    #[test]
    fn test_hr_drift_rate_rising() {
        // HR rises over time -> positive slope
        let records: Vec<FitRecord> = (0..120)
            .map(|i| {
                let hr = 120 + (i as u8 / 4); // slowly rising
                create_record_with_hr(200, hr, 200 * (i + 1))
            })
            .collect();
        let slope = hr_drift_rate(&records);
        assert!(slope > 0.0);
    }

    // --- intensity_factor ---
    #[test]
    fn test_intensity_factor() {
        let ifact = intensity_factor(250.0, 250.0);
        assert!((ifact - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_intensity_factor_zero_ftp() {
        assert_eq!(intensity_factor(0.0, 250.0), 0.0);
    }

    // --- training_stress_score ---
    #[test]
    fn test_tss_one_hour_at_ftp() {
        // 1 hour at FTP -> TSS = 100
        let tss = training_stress_score(250.0, 250.0, 3600);
        assert!((tss - 100.0).abs() < 0.1);
    }

    // --- variability_index ---
    #[test]
    fn test_variability_index_steady() {
        let records: Vec<FitRecord> = (0..60)
            .map(|i| create_mock_record(200, 200 * (i + 1)))
            .collect();
        let vi = variability_index(&records, 200.0);
        assert!((vi - 1.0).abs() < 0.01); // NP == avg -> VI = 1.0
    }

    // --- power_zone_distribution ---
    #[test]
    fn test_power_zones() {
        let ftp = 200;
        let records = vec![
            create_mock_record(100, 100),  // 50% -> Z1
            create_mock_record(160, 260),  // 80% -> Z3
            create_mock_record(220, 480),  // 110% -> Z5
        ];
        let zones = power_zone_distribution(&records, ftp);
        assert_eq!(*zones.get(&1).unwrap_or(&0), 1);
        assert_eq!(*zones.get(&3).unwrap_or(&0), 1);
        assert_eq!(*zones.get(&5).unwrap_or(&0), 1);
    }

    // --- heart_rate_zone_distribution ---
    #[test]
    fn test_hr_zones() {
        let max_hr = 200;
        let records = vec![
            create_record_with_hr(200, 100, 200),  // 50% -> Z1
            create_record_with_hr(200, 150, 400),  // 75% -> Z3
            create_record_with_hr(200, 185, 600),  // 92.5% -> Z5
        ];
        let zones = heart_rate_zone_distribution(&records, max_hr);
        assert_eq!(*zones.get(&1).unwrap_or(&0), 1);
        assert_eq!(*zones.get(&3).unwrap_or(&0), 1);
        assert_eq!(*zones.get(&5).unwrap_or(&0), 1);
    }

    // --- severe_domain_seconds ---
    #[test]
    fn test_severe_domain() {
        let records = vec![
            create_mock_record(200, 200),
            create_mock_record(300, 500),
            create_mock_record(100, 600),
        ];
        assert_eq!(severe_domain_seconds(&records, 250), 1); // only 300W
    }

    // --- total_power_seconds ---
    #[test]
    fn test_total_power_seconds() {
        let mut records = vec![
            create_mock_record(200, 200),
            create_mock_record(300, 500),
        ];
        // add one with no power
        let mut no_power = create_mock_record(0, 0);
        no_power.power = None;
        records.push(no_power);

        assert_eq!(total_power_seconds(&records), 2);
    }

    // --- power_hr_slope ---
    #[test]
    fn test_power_hr_slope_positive() {
        // higher power -> higher HR -> positive slope
        let records = vec![
            create_record_with_hr(100, 100, 100),
            create_record_with_hr(200, 130, 300),
            create_record_with_hr(300, 160, 600),
        ];
        let slope = power_hr_slope(&records);
        assert!(slope > 0.0);
    }
}
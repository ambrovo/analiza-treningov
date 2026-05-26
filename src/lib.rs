pub mod averages_and_totals;
pub mod aggregate;
pub mod fit_parser;
pub mod print_result;
mod testi;

// Re-export the public API used by Tauri and other consumers
pub use self::main_logic::{SingleAnalysisResult, TotalAnalysisResult, TotalResult, analyze_one, analyze_all};

mod main_logic {
    use std::fs;
    use rayon::prelude::*;
    use serde::Serialize;
    use crate::averages_and_totals::*;
    use crate::fit_parser::FitRecord;

    #[derive(Serialize)]
    pub struct TotalAnalysisResult {
        pub filename: String,
        pub workout_date: Option<String>,  // ISO date from first FIT record timestamp
        pub duration_seconds: u32,
        pub total_work_kj: u32,
        pub total_power_seconds: u32,
        pub avg_power: MetricFloat,
        pub avg_hr: MetricFloat,
        pub normalized_power: u32,
        pub intensity_factor: MetricFloat,
        pub tss: MetricFloat,
        pub variability_index: MetricFloat,
        pub aerobic_efficiency: MetricFloat,
        pub aerobic_decoupling: MetricFloat,
        pub aerobic_quality: MetricFloat,
        pub hr_drift_rate: MetricFloat,
        pub power_hr_slope: MetricFloat,
        pub severe_seconds: u32,
        pub extreme_seconds: u32,
        pub power_zones: PowerZoneThresholds,
        pub hr_zones: HrZoneTresholds,
        pub pdc: MetricMap,
        pub fatigued_pdc: NestedMetricMap,
        pub peak_vam: MetricMap,
        pub w_balance: MetricMap,
        pub w_recovery_kj: MetricFloat,
        pub total_distance_km: MetricFloat,
        pub total_elevation_gain: MetricFloat,
        pub avg_cadence: MetricFloat,
        pub avg_speed: MetricFloat,
    }
    pub struct SingleAnalysisResult {
        pub filename: String,
        pub workout_date: Option<String>,  // ISO date from first FIT record timestamp
        pub duration_seconds: u32,
        pub total_work_kj: u32,
        pub total_power_seconds: u32,
        pub avg_power: MetricFloat,
        pub avg_hr: MetricFloat,
        pub normalized_power: u32,
        pub intensity_factor: MetricFloat,
        pub tss: MetricFloat,
        pub variability_index: MetricFloat,
        pub aerobic_efficiency: MetricFloat,
        pub aerobic_decoupling: MetricFloat,
        pub aerobic_quality: MetricFloat,
        pub hr_drift_rate: MetricFloat,
        pub power_hr_slope: MetricFloat,
        pub severe_seconds: u32,
        pub extreme_seconds: u32,
        pub power_zones: PowerZoneThresholds,
        pub hr_zones: HrZoneTresholds,
        pub pdc: MetricMap,
        pub fatigued_pdc: NestedMetricMap,
        pub peak_vam: MetricMap,
        pub w_balance: MetricMap,
        pub w_recovery_kj: MetricFloat,
        pub total_distance_km: MetricFloat,
        pub total_elevation_gain: MetricFloat,
        pub avg_cadence: MetricFloat,
        pub avg_speed: MetricFloat,
        pub power_graph: Graph,
        pub hr_graph: Graph,
        pub altitude_graph: Graph,
        pub speed_graph: Graph,
        pub cadence_graph: Graph,
        pub w_balance_graph: Graph,
    }

    #[derive(Serialize)]
    pub struct TotalResult {
        pub total_workouts: u32,
        pub total_duration_hours: MetricFloat,
        pub total_work_kj: u32,
        pub total_distance_km: MetricFloat,
        pub total_elevation_gain: MetricFloat,
        pub best_pdc: MetricMap,
        pub ctl: Graph,
        pub atl: Graph,
        pub tsb: Graph,
        pub weekly_tss: Graph,
        pub weekly_work_kj: Graph,
        pub weekly_hours: Graph,
        pub aerobic_efficiency_trend: Graph,
        pub np_trend: Graph,
        pub total_power_zones: PowerZoneThresholds,
        pub total_hr_zones: HrZoneTresholds,
    }


    pub fn analyze_one(path: &str, ftp: u32, hr_zone_tresholds: &HrZoneTresholds, w_prime_j: u32, cp: u32, power_zone_thresholds: &PowerZoneThresholds) -> Result<SingleAnalysisResult, Box<dyn std::error::Error>> {
        let data: Vec<FitRecord> = crate::fit_parser::parse_fit_file(path)?;

        let workout_date = data.iter()
            .find_map(|r| r.timestamp.as_ref())
            .map(|dt| dt.format("%Y-%m-%d").to_string());

        let avg_p = average_power_of(&data);
        let avg_h = average_hr_of(&data);
        let np = normalized_power(&data);
        let vi = variability_index(&data, np as f64);
        let wbal_arr = compute_wbal_array(&data, cp, w_prime_j);

        let z2_low = ftp as f64 * 0.55;
        let z2_high = ftp as f64 * 0.75;

        Ok(SingleAnalysisResult {
            filename: path.to_string(),
            workout_date,
            duration_seconds: data.len() as u32,
            total_work_kj: total_work(&data),
            total_power_seconds: total_power_seconds(&data),
            avg_power: avg_p,
            avg_hr: avg_h,
            normalized_power: np,
            intensity_factor: intensity_factor(ftp as f64, np as f64),
            tss: training_stress_score(ftp as f64, np as f64, data.len() as u32),
            variability_index: vi,
            aerobic_efficiency: aerobic_efficiency(&data),
            aerobic_decoupling: aerobic_decoupling(&data),
            aerobic_quality: aerobic_quality_score(&data, vi, z2_low, z2_high),
            hr_drift_rate: hr_drift_rate(&data),
            power_hr_slope: power_hr_slope(&data),
            severe_seconds: severe_domain_seconds(&data, ftp),
            extreme_seconds: extreme_domain_seconds(&data, ftp),
            power_zones: power_zone_distribution(&data, power_zone_thresholds),
            hr_zones: heart_rate_zone_distribution(&data, hr_zone_tresholds),
            pdc: power_duration_curve(&data, 0),
            fatigued_pdc: fatigued_pdc(&data),
            peak_vam: peak_vam(&data),
            w_balance: w_balance(&wbal_arr, w_prime_j),
            w_recovery_kj: w_recovery(&wbal_arr, 300),
            total_distance_km: total_distance(&data),
            total_elevation_gain: total_elevation_gain(&data),
            avg_cadence: average_cadence(&data),
            avg_speed: average_speed(&data),
            power_graph: power_time_series(&data),
            hr_graph: hr_time_series(&data),
            altitude_graph: altitude_time_series(&data),
            speed_graph: speed_time_series(&data),
            cadence_graph: cadence_time_series(&data),
            w_balance_graph: w_balance_graph(&wbal_arr, w_prime_j),
        })
    }


    pub fn analyze_one_for_total(path: &str, ftp: u32, hr_zone_tresholds: &HrZoneTresholds, w_prime_j: u32, cp: u32, power_zone_thresholds: &PowerZoneThresholds) -> Result<TotalAnalysisResult, Box<dyn std::error::Error>> {
        let data: Vec<FitRecord> = crate::fit_parser::parse_fit_file(path)?;

        let workout_date = data.iter()
            .find_map(|r| r.timestamp.as_ref())
            .map(|dt| dt.format("%Y-%m-%d").to_string());

        let avg_p = average_power_of(&data);
        let avg_h = average_hr_of(&data);
        let np = normalized_power(&data);
        let vi = variability_index(&data, np as f64);
        let wbal_arr = compute_wbal_array(&data, cp, w_prime_j);

        let z2_low = ftp as f64 * 0.55;
        let z2_high = ftp as f64 * 0.75;

        Ok(TotalAnalysisResult {
            filename: path.to_string(),
            workout_date,
            duration_seconds: data.len() as u32,
            total_work_kj: total_work(&data),
            total_power_seconds: total_power_seconds(&data),
            avg_power: avg_p,
            avg_hr: avg_h,
            normalized_power: np,
            intensity_factor: intensity_factor(ftp as f64, np as f64),
            tss: training_stress_score(ftp as f64, np as f64, data.len() as u32),
            variability_index: vi,
            aerobic_efficiency: aerobic_efficiency(&data),
            aerobic_decoupling: aerobic_decoupling(&data),
            aerobic_quality: aerobic_quality_score(&data, vi, z2_low, z2_high),
            hr_drift_rate: hr_drift_rate(&data),
            power_hr_slope: power_hr_slope(&data),
            severe_seconds: severe_domain_seconds(&data, ftp),
            extreme_seconds: extreme_domain_seconds(&data, ftp),
            power_zones: power_zone_distribution(&data, power_zone_thresholds),
            hr_zones: heart_rate_zone_distribution(&data, hr_zone_tresholds),
            pdc: power_duration_curve(&data, 0),
            fatigued_pdc: fatigued_pdc(&data),
            peak_vam: peak_vam(&data),
            w_balance: w_balance(&wbal_arr, w_prime_j),
            w_recovery_kj: w_recovery(&wbal_arr, 300),
            total_distance_km: total_distance(&data),
            total_elevation_gain: total_elevation_gain(&data),
            avg_cadence: average_cadence(&data),
            avg_speed: average_speed(&data),
        })
    }
    

    pub fn analyze_all(folder: &str, ftp: u32, hr_zones: &HrZoneTresholds, w_prime_j: u32, cp: u32, power_zones: &PowerZoneThresholds) -> TotalResult {
        use crate::aggregate::*;

        let files: Vec<_> = fs::read_dir(folder)
            .expect("Cannot read folder")
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                let ext = path.extension().map(|x| x.to_string_lossy().to_string());
                ext == Some("gz".to_string()) || ext == Some("FIT".to_string()) || ext == Some("fit".to_string())
            })
            .collect();

        let mut results: Vec<TotalAnalysisResult> = files.par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                analyze_one_for_total(&path.to_string_lossy(), ftp, hr_zones, w_prime_j, cp, power_zones).ok()
            })
            .collect();

        // Razvrsti po datumu za zaporedne izračune (CTL/ATL/TSB)
        results.sort_by(|a, b| a.workout_date.cmp(&b.workout_date));

        TotalResult {
            total_workouts:        results.len() as u32,
            total_duration_hours:  results.iter().map(|r| r.duration_seconds as f64 / 3600.0).sum(),
            total_work_kj:         results.iter().map(|r| r.total_work_kj).sum(),
            total_distance_km:     results.iter().map(|r| r.total_distance_km).sum(),
            total_elevation_gain:  results.iter().map(|r| r.total_elevation_gain).sum(),
            best_pdc:              best_pdc(&results),
            ctl:                   ctl_series(&results),
            atl:                   atl_series(&results),
            tsb:                   tsb_series(&results),
            weekly_tss:            weekly_tss(&results),
            weekly_work_kj:        weekly_work_kj(&results),
            weekly_hours:          weekly_hours(&results),
            aerobic_efficiency_trend: aerobic_efficiency_trend(&results),
            np_trend:              np_trend(&results),
            total_power_zones:     total_power_zone_distribution(&results),
            total_hr_zones:        total_hr_zone_distribution(&results),
        }
    }
}

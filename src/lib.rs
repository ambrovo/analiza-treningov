pub mod averages_and_totals;
pub mod fit_parser;
pub mod print_result;
mod testi;

// Re-export the public API used by Tauri and other consumers
pub use self::main_logic::{AnalysisResult, analyze, analyze_all};

mod main_logic {
    use std::fs;
    use rayon::prelude::*;
    use serde::Serialize;
    use crate::averages_and_totals::*;
    use crate::fit_parser::FitRecord;

    #[derive(Serialize)]
    pub struct AnalysisResult {
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
        pub power_zones: MetricMap,
        pub hr_zones: MetricMap,
        pub pdc: MetricMap,
        pub fatigued_pdc: NestedMetricMap,
        pub peak_vam: MetricMap,
        pub w_balance: MetricMap,
        pub w_recovery_kj: MetricFloat,
    }

    pub fn analyze(path: &str, ftp: u32, max_hr: u32, w_prime_j: u32, cp: u32, zone_thresholds: &[u32]) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
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

        Ok(AnalysisResult {
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
            power_zones: power_zone_distribution(&data, zone_thresholds),
            hr_zones: heart_rate_zone_distribution(&data, max_hr),
            pdc: power_duration_curve(&data, 0),
            fatigued_pdc: fatigued_pdc(&data),
            peak_vam: peak_vam(&data),
            w_balance: w_balance(&wbal_arr, w_prime_j),
            w_recovery_kj: w_recovery(&wbal_arr, 300),
        })
    }

    pub fn analyze_all(folder: &str, ftp: u32, max_hr: u32, w_prime_j: u32, cp: u32, zone_thresholds: &[u32]) -> Vec<AnalysisResult> {
        let files: Vec<_> = fs::read_dir(folder)
            .expect("Cannot read folder")
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                let ext = path.extension().map(|x| x.to_string_lossy().to_string());
                ext == Some("gz".to_string()) || ext == Some("FIT".to_string()) || ext == Some("fit".to_string())
            })
            .collect();

        files.par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                analyze(&path.to_string_lossy(), ftp, max_hr, w_prime_j, cp, zone_thresholds).ok()
            })
            .collect()
    }
}

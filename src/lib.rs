pub mod averages_and_totals;
pub mod fit_parser;
pub mod print_result;
mod testi;

pub use self::main_logic::{analyze_all, analyze_one, AnalysisResult, TotalResult, TrainingParams};

mod main_logic {
    use crate::averages_and_totals::*;
    use crate::fit_parser::FitRecord;
    use rayon::prelude::*;
    use serde::Serialize;
    use std::fs;

    pub struct TrainingParams {
        pub from_date: String,
        pub ftp: u32,
        pub cp: u32,
        pub w_prime_j: u32,
        pub hr_zones: HrZoneTresholds,
        pub power_zones: PowerZoneThresholds,
    }

    #[derive(Serialize)]
    pub struct AnalysisResult {
        pub filename: String,
        pub workout_date: Option<String>,
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
        pub hr_histogram: Vec<MetricInt>,
        pub power_histogram: Vec<MetricInt>,
        pub hr_seconds: MetricInt,
        pub power_seconds: MetricInt,
        pub durability_ratio: MetricFloat,
        pub power_coverage: MetricFloat,
        pub hr_coverage: MetricFloat,
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

    fn params_for_date<'a>(
        params_history: &'a [TrainingParams],
        workout_date: &str,
    ) -> &'a TrainingParams {
        params_history
            .iter()
            .rev()
            .find(|p| p.from_date.as_str() <= workout_date)
            .unwrap_or(&params_history[0])
    }

    fn analyze_data(
        path: &str,
        data: &[FitRecord],
        workout_date: &str,
        params: &TrainingParams,
    ) -> AnalysisResult {
        let avg_p = average_power_of(data);
        let avg_h = average_hr_of(data);
        let np = normalized_power(data);
        let vi = variability_index(data, np as f64);
        let wbal_arr = compute_wbal_array(data, params.cp, params.w_prime_j);
        let power_histogram = power_density_histogram(data);
        let hr_histogram = hr_density_histogram(data);
        let power_seconds = total_power_seconds(data);
        let hr_seconds = total_hr_seconds(data);
        AnalysisResult {
            filename: path.to_string(),
            workout_date: Some(workout_date.to_string()),
            duration_seconds: data.len() as u32,
            total_work_kj: total_work(data),
            total_power_seconds: total_power_seconds(data),
            avg_power: avg_p,
            avg_hr: avg_h,
            normalized_power: np,
            intensity_factor: intensity_factor(params.ftp as f64, np as f64),
            tss: training_stress_score(params.ftp as f64, np as f64, data.len() as u32),
            variability_index: vi,
            aerobic_efficiency: aerobic_efficiency(data),
            aerobic_decoupling: aerobic_decoupling(data),
            aerobic_quality: aerobic_quality_score(
                data,
                vi,
                params.power_zones.zone_2 as MetricFloat,
                params.power_zones.zone_2a as MetricFloat,
            ),
            hr_drift_rate: hr_drift_rate(data),
            power_hr_slope: power_hr_slope(data),
            severe_seconds: severe_domain_seconds(data, params.ftp),
            extreme_seconds: extreme_domain_seconds(data, params.ftp),
            power_zones: power_zone_distribution(&params.power_zones, &power_histogram),
            hr_zones: heart_rate_zone_distribution(&params.hr_zones, &hr_histogram),
            pdc: power_duration_curve(data, 0),
            fatigued_pdc: fatigued_pdc(data),
            peak_vam: peak_vam(data),
            w_balance: w_balance(&wbal_arr, params.w_prime_j),
            w_recovery_kj: w_recovery(&wbal_arr, 300),
            total_distance_km: total_distance(data),
            total_elevation_gain: total_elevation_gain(data),
            avg_cadence: average_cadence(data),
            avg_speed: average_speed(data),
            power_graph: power_time_series(data),
            hr_graph: hr_time_series(data),
            altitude_graph: altitude_time_series(data),
            speed_graph: speed_time_series(data),
            cadence_graph: cadence_time_series(data),
            w_balance_graph: w_balance_graph(&wbal_arr, params.w_prime_j),
            hr_histogram,
            power_histogram,
            hr_seconds,
            power_seconds,
            durability_ratio: durability_ratio(data),
            power_coverage: power_coverage(data, power_seconds),
            hr_coverage: hr_coverage(data, hr_seconds),
        }
    }

    pub fn analyze_one(
        path: &str,
        workout_date: &str,
        params: &TrainingParams,
        with_cache: bool,
    ) -> AnalysisResult {
        let data: Vec<FitRecord> = if with_cache {
            crate::fit_parser::parse_fit_file_cached(path)
        } else {
            crate::fit_parser::parse_fit_file(path)
        }
        .unwrap_or_default();
        analyze_data(path, &data, workout_date, params)
    }

    pub fn analyze_all(
        folder: &str,
        params_history: &mut [TrainingParams],
        with_cache: bool,
    ) -> Vec<AnalysisResult> {
        params_history.sort_by(|a, b| a.from_date.cmp(&b.from_date));

        let files: Vec<_> = fs::read_dir(folder)
            .expect("Cannot read folder")
            .filter_map(|e| e.ok())
            .filter(|e| {
                let ext = e
                    .path()
                    .extension()
                    .map(|x| x.to_string_lossy().to_lowercase());
                ext == Some("gz".to_string()) || ext == Some("fit".to_string())
            })
            .collect();

        let mut results: Vec<AnalysisResult> = files
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                let path_str = path.to_string_lossy();

                let data = if with_cache {
                    crate::fit_parser::parse_fit_file_cached(&path_str)
                } else {
                    crate::fit_parser::parse_fit_file(&path_str)
                }
                .ok()?;

                let date = data
                    .iter()
                    .find_map(|r| r.timestamp.as_ref())
                    .map(|dt| dt.format("%Y-%m-%d").to_string())?;

                let params = params_for_date(params_history, &date);
                Some(analyze_data(&path_str, &data, &date, params))
            })
            .collect();

        results.sort_by(|a, b| a.workout_date.cmp(&b.workout_date));
        results
    }
}

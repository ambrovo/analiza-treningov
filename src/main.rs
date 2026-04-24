mod fit_parser;
pub mod averages_and_totals;
mod testi;
pub mod print_result;

use std::fs;
use std::time::Instant;
use rayon::prelude::*;
use std::collections::HashMap;
use crate::averages_and_totals::*;
use crate::fit_parser::FitRecord;
use crate::print_result::print_result;

pub struct AnalysisResult {
    pub filename: String,
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

fn analyze(path: &str, ftp: u32, max_hr: u32, w_prime_j: u32, cp: u32) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let data: Vec<FitRecord> = fit_parser::parse_fit_file(path)?;

    let avg_p = average_power_of(&data);
    let avg_h = average_hr_of(&data);
    let np = normalized_power(&data);
    let vi = variability_index(&data, np as f64);
    let wbal_arr = compute_wbal_array(&data, cp, w_prime_j);

    // zone 2 boundaries for aerobic quality
    let z2_low = ftp as f64 * 0.55;
    let z2_high = ftp as f64 * 0.75;

    Ok(AnalysisResult {
        filename: path.to_string(),
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
        power_zones: power_zone_distribution(&data, ftp),
        hr_zones: heart_rate_zone_distribution(&data, max_hr),
        pdc: power_duration_curve(&data, 0),
        fatigued_pdc: fatigued_pdc(&data),
        peak_vam: peak_vam(&data),
        w_balance: w_balance(&wbal_arr, w_prime_j),
        w_recovery_kj: w_recovery(&wbal_arr, 300),
    })
}



fn analyze_all(folder: &str, ftp: u32, max_hr: u32, w_prime_j: u32, cp: u32) {
    let files: Vec<_> = fs::read_dir(folder)
        .expect("Cannot read folder")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            let ext = path.extension().map(|x| x.to_string_lossy().to_string());
            ext == Some("gz".to_string()) || ext == Some("FIT".to_string())
        })
        .collect();

    let results: Vec<AnalysisResult> = files.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            analyze(&path.to_string_lossy(), ftp, max_hr, w_prime_j, cp).ok()
        })
        .collect();

    println!("\nAnalyzed {} files\n", results.len());

    
    print_result(&results[0]);
    println!();

   
}

fn main() {
    let ftp: u32 = 250;
    let max_hr: u32 = 190;
    let cp: u32 = 240;         // critical power
    let w_prime_j: u32 = 20000; // W' in joules (20 kJ)

    let start = Instant::now();
    analyze_all("test", ftp, max_hr, w_prime_j, cp);
    println!("Total time: {:?}", start.elapsed());
}
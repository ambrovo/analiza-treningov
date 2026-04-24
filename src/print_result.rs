
use crate::AnalysisResult;
pub fn print_result(r: &AnalysisResult) {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  {}", r.filename);
    println!("╠══════════════════════════════════════════════════════╣");
    println!("║  DURATION & WORK");
    println!("║    Duration:            {} min", r.duration_seconds / 60);
    println!("║    Power Seconds:       {} s", r.total_power_seconds);
    println!("║    Total Work:          {} kJ", r.total_work_kj);
    println!("║");
    println!("║  POWER & HR");
    println!("║    Avg Power:           {:.1} W", r.avg_power);
    println!("║    Avg HR:              {:.1} bpm", r.avg_hr);
    println!("║    Normalized Power:    {} W", r.normalized_power);
    println!("║    Intensity Factor:    {:.3}", r.intensity_factor / 1000.0);
    println!("║    TSS:                 {:.1}", r.tss);
    println!("║    Variability Index:   {:.3}", r.variability_index);
    println!("║");
    println!("║  AEROBIC METRICS");
    println!("║    Aerobic Efficiency:  {:.3} W/bpm", r.aerobic_efficiency);
    println!("║    Aerobic Decoupling:  {:.2}%", r.aerobic_decoupling);
    println!("║    Aerobic Quality:     {:.3}", r.aerobic_quality);
    println!("║    HR Drift Rate:       {:.4} bpm/min", r.hr_drift_rate);
    println!("║    Power:HR Slope:      {:.4}", r.power_hr_slope);
    println!("║");
    println!("║  DOMAINS");
    println!("║    Severe (>FTP):       {} s ({} min)", r.severe_seconds, r.severe_seconds / 60);
    println!("║    Extreme (>150%FTP):  {} s ({} min)", r.extreme_seconds, r.extreme_seconds / 60);
    println!("║");
    println!("║  W' BALANCE");
    if let Some(min_pct) = r.w_balance.get(&1) {
        println!("║    Min W' Balance:     {}%", min_pct);
    }
    if let Some(t75) = r.w_balance.get(&2) {
        println!("║    Time <75% W':       {} s", t75);
    }
    if let Some(t50) = r.w_balance.get(&3) {
        println!("║    Time <50% W':       {} s", t50);
    }
    if let Some(t25) = r.w_balance.get(&4) {
        println!("║    Time <25% W':       {} s", t25);
    }
    if let Some(count) = r.w_balance.get(&5) {
        println!("║    Depletion Count:    {}", count);
    }
    println!("║    Best 5min Recovery: {:.1} kJ", r.w_recovery_kj);

    println!("║");
    println!("║  POWER ZONES:");
    let mut zones: Vec<_> = r.power_zones.iter().collect();
    zones.sort_by_key(|(k, _)| *k);
    let max_min = zones.iter().map(|(_, s)| **s / 60).max().unwrap_or(1).max(1);
    for (zone, secs) in &zones {
        let mins = **secs / 60;
        let bar_len = (mins as f64 / max_min as f64 * 30.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("║    Z{}: {:>5}s ({:>3}min) {}", zone, secs, mins, bar);
    }

    println!("║");
    println!("║  HR ZONES:");
    let mut zones: Vec<_> = r.hr_zones.iter().collect();
    zones.sort_by_key(|(k, _)| *k);
    let max_min = zones.iter().map(|(_, s)| **s / 60).max().unwrap_or(1).max(1);
    for (zone, secs) in &zones {
        let mins = **secs / 60;
        let bar_len = (mins as f64 / max_min as f64 * 30.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("║    Z{}: {:>5}s ({:>3}min) {}", zone, secs, mins, bar);
    }

    println!("║");
    println!("║  POWER DURATION CURVE:");
    let mut pdc: Vec<_> = r.pdc.iter().collect();
    pdc.sort_by_key(|(k, _)| *k);
    for (dur, watts) in &pdc {
        let label = match **dur {
            d if d < 60 => format!("{}s", d),
            d if d < 3600 => format!("{}min", d / 60),
            d => format!("{}h", d / 3600),
        };
        println!("║    {:>6}: {:>4} W", label, watts);
    }
    
    println!("║");
    println!("║  FATIGUED PDC:");
    let mut outer: Vec<_> = r.fatigued_pdc.iter().collect();
    outer.sort_by_key(|(k, _)| *k);
    for (kj, inner) in &outer {
        println!("║    After {}kJ:", kj);
        let mut pdc: Vec<_> = inner.iter().collect();
        pdc.sort_by_key(|(k, _)| *k);
        for (dur, watts) in &pdc {
            let label = match **dur {
                d if d < 60 => format!("{}s", d),
                d if d < 3600 => format!("{}min", d / 60),
                d => format!("{}h", d / 3600),
            };
            println!("║      {:>6}: {:>4} W", label, watts);
        }
    }
    if !r.peak_vam.is_empty() {
        println!("║");
        println!("║  PEAK VAM:");
        let mut vam: Vec<_> = r.peak_vam.iter().collect();
        vam.sort_by_key(|(k, _)| *k);
        for (dur, val) in &vam {
            println!("║    {:>3}min: {} m/h", **dur / 60, val);
        }
    }
    println!("╚══════════════════════════════════════════════════════╝");
}

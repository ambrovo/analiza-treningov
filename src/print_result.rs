use crate::AnalysisResult;

pub fn print_results(results: &[AnalysisResult]) {
    println!("=== Skupni rezultat ({} treningov) ===", results.len());
    println!(
        "Ure: {:.1}",
        results
            .iter()
            .map(|r| r.duration_seconds as f64 / 3600.0)
            .sum::<f64>()
    );
    println!(
        "Delo: {} kJ",
        results.iter().map(|r| r.total_work_kj).sum::<u32>()
    );
    println!(
        "Razdalja: {:.1} km",
        results.iter().map(|r| r.total_distance_km).sum::<f64>()
    );
    println!(
        "Vzpon: {:.0} m",
        results.iter().map(|r| r.total_elevation_gain).sum::<f64>()
    );

    // Skupne ure po conah — moč
    let pz1: f64 = results.iter().map(|r| r.power_zones.zone_1).sum::<u32>() as f64 / 3600.0;
    let pz2: f64 = results.iter().map(|r| r.power_zones.zone_2).sum::<u32>() as f64 / 3600.0;
    let pz2a: f64 = results.iter().map(|r| r.power_zones.zone_2a).sum::<u32>() as f64 / 3600.0;
    let pz3: f64 = results.iter().map(|r| r.power_zones.zone_3).sum::<u32>() as f64 / 3600.0;
    let pz4: f64 = results.iter().map(|r| r.power_zones.zone_4).sum::<u32>() as f64 / 3600.0;
    let pz5: f64 = results.iter().map(|r| r.power_zones.zone_5).sum::<u32>() as f64 / 3600.0;
    let pz6: f64 = results.iter().map(|r| r.power_zones.zone_6).sum::<u32>() as f64 / 3600.0;
    let pz7: f64 = results.iter().map(|r| r.power_zones.zone_7).sum::<u32>() as f64 / 3600.0;
    println!("\nMoč — cone (ure):");
    println!(
        "  Z1={:.1}h  Z2={:.1}h  Z2a={:.1}h  Z3={:.1}h  Z4={:.1}h  Z5={:.1}h  Z6={:.1}h  Z7={:.1}h",
        pz1, pz2, pz2a, pz3, pz4, pz5, pz6, pz7
    );

    // Skupne ure po conah — srčni utrip
    let hz1: f64 = results.iter().map(|r| r.hr_zones.zone_1).sum::<u32>() as f64 / 3600.0;
    let hz2: f64 = results.iter().map(|r| r.hr_zones.zone_2).sum::<u32>() as f64 / 3600.0;
    let hz2a: f64 = results.iter().map(|r| r.hr_zones.zone_2a).sum::<u32>() as f64 / 3600.0;
    let hz3: f64 = results.iter().map(|r| r.hr_zones.zone_3).sum::<u32>() as f64 / 3600.0;
    let hz4: f64 = results.iter().map(|r| r.hr_zones.zone_4).sum::<u32>() as f64 / 3600.0;
    let hz5: f64 = results.iter().map(|r| r.hr_zones.zone_5).sum::<u32>() as f64 / 3600.0;
    println!("\nSR — cone :");
    println!(
        "  Z1={:.1}h Z2={:.1}h  Z2a={:.1}h  Z3={:.1}h  Z4={:.1}  Z5={:.1}h",
        hz1, hz2, hz2a, hz3, hz4, hz5
    );

    println!("\n=== Zadnjih 10 treningov ===");
    for r in results.iter().rev().take(10) {
        println!(
            "{} | {:>4}W avg | {:>4}W NP | {:>5.1} TSS | {:>5.1} km | {:>5.1} bpm avg | {:>5.1} km/h avg | {:>5.1} rpm avg",
            r.workout_date.as_deref().unwrap_or("unknown"),
            r.avg_power as u32,
            r.normalized_power,
            r.tss,
            r.total_distance_km,
            r.avg_hr,
            r.avg_speed,
            r.avg_cadence,
        );
    }
}

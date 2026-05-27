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

    println!("\n=== Zadnjih 10 treningov ===");
    for r in results.iter().rev().take(10) {
        println!(
            "{} | {:>4}W avg | {:>4}W NP | {:>5.1} TSS | {:>5.1} km",
            r.workout_date.as_deref().unwrap_or("unknown"),
            r.avg_power as u32,
            r.normalized_power,
            r.tss,
            r.total_distance_km,
        );
    }
}

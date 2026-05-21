use analiza_treningov::analyze_all;
use analiza_treningov::print_result::print_result;
use std::time::Instant;

fn main() {
    let ftp: u32 = 250;
    let max_hr: u32 = 190;
    let cp: u32 = 240;
    let w_prime_j: u32 = 20000;

    // CP-based zone thresholds (7 upper bounds, zone 8 = above last)
    let zone_thresholds: Vec<u32> = vec![
        (cp as f64 * 0.45) as u32,
        (cp as f64 * 0.70) as u32,
        (cp as f64 * 0.77) as u32,
        (cp as f64 * 0.95) as u32,
        (cp as f64 * 1.05) as u32,
        (cp as f64 * 1.20) as u32,
        (cp as f64 * 1.50) as u32,
    ];

    let start = Instant::now();
    let results = analyze_all("test", ftp, max_hr, w_prime_j, cp, &zone_thresholds);
    println!("\nAnalyzed {} files\n", results.len());
    if !results.is_empty() {
        print_result(&results[0]);
    }
    println!("Total time: {:?}", start.elapsed());
}
use analiza_treningov::averages_and_totals::{HrZoneTresholds, PowerZoneThresholds};
use analiza_treningov::print_result::print_results;
use analiza_treningov::{analyze_all, TrainingParams};
use std::time::Instant;

const FOLDER: &str = "test";

fn main() {
    let mut params_history = vec![
        TrainingParams {
            from_date: "2023-01-01".to_string(),
            ftp: 250,
            cp: 240,
            w_prime_j: 20000,
            hr_zones: HrZoneTresholds {
                zone_1: 0,
                zone_2a: 110,
                zone_2b: 130,
                zone_3: 150,
                zone_4: 165,
                zone_5: 175,
            },
            power_zones: PowerZoneThresholds {
                zone_1: 0,
                zone_2a: 100,
                zone_2b: 150,
                zone_3: 200,
                zone_4: 250,
                zone_5: 300,
                zone_6: 350,
                zone_7: 400,
            },
        },
        TrainingParams {
            from_date: "2024-01-01".to_string(),
            ftp: 280,
            cp: 270,
            w_prime_j: 22000,
            hr_zones: HrZoneTresholds {
                zone_1: 0,
                zone_2a: 110,
                zone_2b: 130,
                zone_3: 150,
                zone_4: 165,
                zone_5: 175,
            },
            power_zones: PowerZoneThresholds {
                zone_1: 0,
                zone_2a: 180,
                zone_2b: 200,
                zone_3: 250,
                zone_4: 280,
                zone_5: 300,
                zone_6: 400,
                zone_7: 500,
            },
        },
    ];

    let t_total = Instant::now();
    let total = analyze_all(FOLDER, &mut params_history, true);
    println!(
        "Analizirano {} treningov v {:?}",
        total.len(),
        t_total.elapsed()
    );

    print_results(&total);
}

use analiza_treningov::{analyze_all, analyze_one, combine_and_analyze_all};
use analiza_treningov::print_result::{print_result, print_single_result};
use analiza_treningov::averages_and_totals::{PowerZoneThresholds, HrZoneTresholds};
use std::time::Instant;

const FOLDER: &str = "test";
const SINGLE: &str = "test/tp-4447306.2026-05-20-17-05-03-743Z.GarminPing.AAAAAGoN6b8FUxX7.FIT.gz";
fn main() {
    let ftp: u32 = 250;
    let cp: u32 = 240;
    let w_prime_j: u32 = 20000;

    let power_zones = PowerZoneThresholds {
        zone_1:  0,
        zone_2a: 100,
        zone_2b: 150,
        zone_3:  200,
        zone_4:  250,
        zone_5:  300,
        zone_6:  350,
        zone_7:  400,
    };

    let hr_zones = HrZoneTresholds {
        zone_1:  0,
        zone_2a: 110,
        zone_2b: 130,
        zone_3:  150,
        zone_4:  165,
        zone_5:  175,
    };

    let t_total = Instant::now();

    let total = combine_and_analyze_all(FOLDER, ftp, &hr_zones, w_prime_j, cp, &power_zones, true);
    println!("Analizirano {} treningov v {:?}", total.total_workouts, t_total.elapsed());
    println!("Skupno trajanje: {:.1} h", total.total_duration_hours);
    println!("Skupno delo:     {} kJ", total.total_work_kj);
    print_result(&total);
     let t_single = Instant::now();
    match analyze_one(SINGLE, ftp, &hr_zones, w_prime_j, cp, &power_zones, true) {
      Ok(single) => print_single_result(&single),
      Err(e) => println!("Napaka: {}", e),
  }
    println!("Analiziran 1 trening v {:?}", t_single.elapsed());
}
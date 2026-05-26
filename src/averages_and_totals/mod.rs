

use crate::fit_parser::FitRecord;
use std::collections::HashMap;
use serde::Serialize;

pub type MetricInt = u32;
pub type MetricFloat = f64;
pub type MetricMap = HashMap<MetricInt, MetricInt>;
pub type NestedMetricMap = HashMap<MetricInt, MetricMap>;
#[derive(Serialize)]
pub struct Graph {
    pub name: String,
    pub x_axis: Axis,
    pub y_axis: Axis,
    pub series: HashMap<String, Vec<(f64, f64)>>, // name -> [(x, y)]
}

#[derive(Serialize)]
pub struct Axis {
    pub label: String,
    pub unit: Unit,
}

#[derive(Serialize)]
pub enum Unit {
    Watts,
    Bpm,
    Seconds,
    Minutes,
    Hours,
    Meters,
    Kilometers,
    Percent,
    Rpm,
    Celsius,
    MetersPerSecond,
    Custom(String),
}
pub struct FitData {
    pub data: Vec<FitRecord>,
}
#[derive(Serialize)]
pub struct PowerZoneThresholds {
        pub zone_1 : u32,
        pub zone_2a : u32,
        pub zone_2b : u32,
        pub zone_3 : u32,
        pub zone_4 : u32,
        pub zone_5 : u32,
        pub zone_6 : u32,
        pub zone_7 : u32,
    }
#[derive(Serialize)]
pub struct HrZoneTresholds {
        pub zone_1 : u32,
        pub zone_2a : u32,
        pub zone_2b : u32,
        pub zone_3 : u32,
        pub zone_4 : u32,
        pub zone_5 : u32,
       
    }

pub fn power_duration_curve(data: &[FitRecord], start_index: usize) -> MetricMap {
    // maksimalna moč glede na standardna časovna okna
    let durations: [MetricInt; 12] = [1, 3, 5, 30, 60, 120, 300, 600, 1200,1800, 3600, 7200];
    
    let mut res: MetricMap = HashMap::new();
    
    for d in durations {
        
        
        
        let mut m = 0;
        let start = start_index + (d as usize);
        for i   in start..(data.len())  {
            if let (Some(curr), Some(prev)) = (
                data[i as usize].accumulated_power, 
                data[(i - (d as usize)) as usize].accumulated_power
            ) {
                let diff = curr.saturating_sub(prev);
                if diff > m {
                    m = diff;
                }
            }
        }
        if m != 0 {
            res.insert(d, m/d);
        }
    }
    return res;
    
    
}


pub fn fatigued_pdc(data: &[FitRecord]) -> NestedMetricMap {
    //Maksimalne moči dobimo po različno opravljenem delu

    //let power_acumulations = [1 ,1000 ,2000 ,3000 ];
    //let mut res: NestedMetricMap = HashMap::new();
    //for ac_power in power_acumulations {
    //    for j in 0..data.len() {
    //        if let Some(ac) = data[j].accumulated_power  {
    //            if ac > ac_power*1000 {
    //                res.insert(ac_power, power_duration_curve(data, j));
    //                break;
    //            }
    //        }
    //    }
    //}
    //return res; 

    let power_acumulations = [1 ,1000 ,2000 ,3000 ];

    power_acumulations
        .into_iter()
        .filter_map(|ac_power| {
            data.iter()
                .position(|d| {
                    d.accumulated_power
                        .map_or(false, |ac| ac > ac_power * 1000)
                })
                .map(|idx| (ac_power, power_duration_curve(data, idx)))
        })
        .collect()
    
}

pub fn normalized_power(data: &[FitRecord]) -> MetricInt {
    //Fiziološko prilagojena moč, ki upošteva variabilnst treninga
    //Formula ki je bila uporabljena je $NP = (\frac{1}{N} \sum_{i=1}^{N}(P_{30s, i})^4)^{\frac{1}{4}}$, 
    //kjer je P_{30s, i} povprečna moč v zadnjih 30 sekundah pri času i, N pa število 30-sekundnih povprečij.
    let time = 30;

     if data.len() < time {
        return 0;
    }

     let power: Vec<f32> = data.iter().map(|r| r.power.unwrap_or(0) as f32).collect();

     let mut rolling_avg = Vec::new();
    let mut sum: f32 = power[..time].iter().sum();

     for i in time..power.len() {
        rolling_avg.push(sum / time as f32);
        sum += power[i];
        sum -= power[i - time];
    }
    rolling_avg.push(sum / time as f32);

     let mean_4th: f32 = rolling_avg.iter().map(|p: &f32| p.powi(4)).sum::<f32>() / rolling_avg.len() as f32;

     mean_4th.powf(0.25).round() as u32
}
    
pub fn intensity_factor(ftp: MetricFloat, np : MetricFloat) -> MetricFloat {
    //Relativna intenziteta glede na FTP (NP®/FTP)
    // $IF = \frac{NP}{FTP}$
    if ftp == 0.0 {
        return 0.0;
    }    
    (np * 1000.0) / ftp
}

pub fn training_stress_score(ftp: MetricFloat, np : MetricFloat, duration: MetricInt) -> MetricFloat {
    //Obremenitev treninga (1 ura pri FTP = 100 TSS®)
    if ftp == 0.0 {
        return 0.0;
    }
    let tss = (duration as f64 * np * np) / (ftp * ftp * 3600.0) * 100.0;
    tss
}

pub fn variability_index(data: &[FitRecord], np : MetricFloat) -> MetricFloat {
    //Mera variabilnosti moči (NP®/avg_power) - ali je trening "steady"
    // $VI = \frac{NP}{average power}$
    let mut sum: u32 = 0;
    let mut count: u32 = 0;

    for r in data {
        if let Some(p) = r.power {
            sum += p as u32;
            count += 1;
        }
    }

     if count == 0 {
        return 0.0;
    }

    let avg_power = sum as f64 / count as f64;
       

    if avg_power == 0.0 {
        return 0.0;
    }

    let vi = np / avg_power;

    vi
}
    
pub fn power_zone_distribution(data: &[FitRecord], z: &PowerZoneThresholds) -> PowerZoneThresholds {
  
    const MAX_POWER: usize = 3001; 

    // vektor za vsako moč : število sekund na tej moči
    let mut histogram = vec![0u32; MAX_POWER];
    for record in data.iter() {
        let power = record.power.unwrap_or(0) as usize;
        histogram[power] += 1;
    }

    //  
    let mut accumulative_histogram = vec![0u32; MAX_POWER + 1];
    accumulative_histogram[0] = histogram[0];
    for i in 0..MAX_POWER {
        accumulative_histogram[i + 1] = accumulative_histogram[i] + histogram[i];
    }

    // pomožna funkcija za vsoto glede na cone
    let range_sum = |lo: usize, hi: usize| -> u32 {
        let hi = hi.min(MAX_POWER);
        accumulative_histogram[hi] - accumulative_histogram[lo]
    };

    //rezultat v formatu, ki vsebuje imena con
  
    let res = PowerZoneThresholds {
        zone_1:  range_sum(0,             z.zone_2a as usize),
        zone_2a: range_sum(z.zone_2a as usize, z.zone_2b as usize),
        zone_2b: range_sum(z.zone_2b as usize, z.zone_3 as usize),
        zone_3:  range_sum(z.zone_3 as usize,  z.zone_4 as usize),
        zone_4:  range_sum(z.zone_4 as usize,  z.zone_5 as usize),
        zone_5:  range_sum(z.zone_5 as usize,  z.zone_6 as usize),
        zone_6:  range_sum(z.zone_6 as usize,  z.zone_7 as usize),
        zone_7:  range_sum(z.zone_7 as usize,  MAX_POWER),
    };

    res
}
    
pub fn heart_rate_zone_distribution(data: &[FitRecord], z: &HrZoneTresholds) -> HrZoneTresholds {
    //Čas (sekunde) v vsaki srčni coni (Z1-Z5)

    const MAX_HR: usize = 301; 

    // vektor za vsako moč : število sekund na tej moči
    let mut histogram = vec![0u32; MAX_HR];
    for record in data.iter() {
        let hr = record.heart_rate.unwrap_or(0) as usize;
        histogram[hr] += 1;
    }

    //  
    let mut accumulative_histogram = vec![0u32; MAX_HR + 1];
    accumulative_histogram[0] = histogram[0];
    for i in 0..MAX_HR {
        accumulative_histogram[i + 1] = accumulative_histogram[i] + histogram[i];
    }

    // pomožna funkcija za vsoto glede na cone
    let range_sum = |lo: usize, hi: usize| -> u32 {
        let hi = hi.min(MAX_HR);
        accumulative_histogram[hi] - accumulative_histogram[lo]
    };

    //rezultat v formatu, ki vsebuje imena con
  
    let res = HrZoneTresholds {
        zone_1:  range_sum(0,             z.zone_2a as usize),
        zone_2a: range_sum(z.zone_2a as usize, z.zone_2b as usize),
        zone_2b: range_sum(z.zone_2b as usize, z.zone_3 as usize),
        zone_3:  range_sum(z.zone_3 as usize,  z.zone_4 as usize),
        zone_4:  range_sum(z.zone_4 as usize,  z.zone_5 as usize),
        zone_5:  range_sum(z.zone_5 as usize,  MAX_HR)
    };

    res
}
    
pub fn severe_domain_seconds(data: &[FitRecord], ftp: MetricInt) -> MetricInt {
    //Sekunde z močjo nad FTP
    if ftp == 0 {
        return 0;
    };

    let mut seconds: u32 = 0;

    for r in data {
        if let Some(p) = r.power {
            if p as u32 > ftp {
                seconds += 1
            }
        }
    };

    seconds
}
    
pub fn extreme_domain_seconds(data: &[FitRecord], ftp: MetricInt) -> MetricInt {
    //Sekunde z močjo nad 150% FTP (nevromuskularna cona)
    if ftp == 0 {
        return 0;
    };

    let mut seconds: u32 = 0;

    for r in data {
        if let Some(p) = r.power {
            if p as f32 > (ftp as f32 * 3.0) / 2.0 {
                seconds += 1
            }
        }
    };

    seconds
}
    
pub fn total_power_seconds(data: &[FitRecord]) -> MetricInt {
    //Skupno število sekund z veljavnimi podatki o moči
    let mut seconds: u32 = 0;

    for r in data {
        if r.power.is_some() {
            seconds += 1
        }
    };

    seconds
}
    
pub fn total_work(data: &[FitRecord]) -> MetricInt {
    //Skupno mehansko delo proizvedeno med treningom
    let total_joules: u32 = data
    .iter()
    .filter_map(|r| r.power)
    .map(|p| p as u32)
    .sum();

    (total_joules / 1000) as u32
}
    
pub fn peak_vam(data: &[FitRecord]) -> MetricMap {
    //Maksimalna hitrost vzpenjanja (m/h) na 5min, 10min, 20min
    // $VAM = \frac{\delta h}{\delta t} * 3600$
    let duration: [MetricInt; 3] = [300, 600, 1200];
    let mut res: MetricMap = HashMap::new();

    for d in duration {
        let mut max_vam = 0.0;

        if data.len() < d as usize {
            continue;
        }

        for i in d as usize..data.len() {
            if let (Some(curr_alt), Some(prev_alt)) = (
                data[i].enhanced_altitude,
                data[i - d as usize].enhanced_altitude,
            ) {
                let delta_h = curr_alt - prev_alt;

                if delta_h > 0.0 {
                    let vam = (delta_h / d as f64) * 3600.0;

                    if vam > max_vam {
                        max_vam = vam;
                    }
                }
            }
        }

        if max_vam > 0.0 {
            res.insert(d, max_vam.round() as u32);
        }
    }

    res
}
    
    
    
    
pub fn fatigue_resistance_drops(data: &[FitRecord])  {
 
}
    
pub fn fatigue_resistance_index(data: &[FitRecord])   {
    
}
    
pub fn average_power_of(data: &[FitRecord]) -> MetricFloat {
    let mut total: u32 = 0; 
    for i in 0..data.len() {
        total += data[i].power.unwrap_or(0) as u32
    }; 
    total as f64/data.len()   as f64
}

pub fn average_hr_of(data: &[FitRecord]) -> MetricFloat {
    let mut total: f64 = 0.0; 
    for i in 0..data.len() {
        total += data[i].heart_rate.unwrap_or(0) as f64
    }; 
    (total)/(data.len() as f64)  
}

pub fn aerobic_efficiency(data: &[FitRecord]) -> MetricFloat {
    //Povprečna moč : Povprečni srčni utrip.
    average_power_of(data)/average_hr_of(data) 
}
    
pub fn aerobic_decoupling(data: &[FitRecord]) -> MetricFloat {
    let mid = data.len() / 2;
    let ef1 = average_power_of(&data[..mid]) / average_hr_of(&data[..mid]);
    let ef2 = average_power_of(&data[mid..]) / average_hr_of(&data[mid..]);
    ((ef1 - ef2) / ef1)  * 100.0
}
    
pub fn hr_drift_rate(data: &[FitRecord]) -> MetricFloat{
    let mut n = 0.0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;

    let start = 0;

    for i in 0..data.len() {
        let r = &data[i];
        if let Some(hr) = r.heart_rate {
            let x = (i as i32 - start)as f64  / 60 as f64;
            let y = hr as f64;
            n += 1.0;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }
    }

    if n < 2.0 { return 0.0; }

    // slope = (n * Σxy - Σx * Σy) / (n * Σx² - (Σx)²)
    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);

    slope
}
    
pub fn power_hr_slope(data: &[FitRecord]) -> MetricFloat{
    let mut n = 0.0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;

    for r in data {                                     
      if let (Some(power), Some(hr)) = (r.power, r.heart_rate) {
        let x = power as f64;
        let y = hr as f64;                                        
        n += 1.0;
        sum_x += x;                                               
        sum_y += y;
        sum_xy += x * y;
        sum_xx += x * x;
        }
    }

    if n < 2.0 { return 0.0; }

   
    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);

    slope
}
    
   
pub fn aerobic_quality_score(data: &[FitRecord], variability_index: MetricFloat, zone_low: MetricFloat, zone_high : MetricFloat) -> MetricFloat {                                                
    let duration_factor = (data.len() as f64 / 7200.0).min(1.0);
    let steadiness = (1.0 - (variability_index - 1.0) * 10.0).max(0.0);

    let avg_p = average_power_of(data);
    let intensity_mid = (zone_low + zone_high) / 2.0;
    let intensity_match = (1.0 - (avg_p - intensity_mid).abs() / (zone_high - zone_low + 1.0)).max(0.0);

    duration_factor * 0.3 + steadiness * 0.4 + intensity_match * 0.3
}

// Skiba differential model: computes W' balance for each second.
// Above CP: drains by (power - CP) joules. Below CP: recovers exponentially toward W'.                              
pub fn compute_wbal_array(data: &[FitRecord], cp: MetricInt, w_prime_j: MetricInt) -> Vec<f64> {
    let wp = w_prime_j as f64;
    let cp_f = cp as f64;                                                                                             
    let mut wpbal = wp;
    let mut arr = Vec::with_capacity(data.len());                                                                     
                  
    for r in data {
        let p = r.power.unwrap_or(0) as f64;
        if p > cp_f {
            wpbal -= p - cp_f;
        } else {
            let dcp = (cp_f - p).max(1.0);
            let tau = 546.0 * (-0.01 * dcp).exp() + 316.0;
            wpbal += (wp - wpbal) * (1.0 - (-1.0 / tau).exp());
        }
        wpbal = wpbal.max(0.0).min(wp);
        arr.push(wpbal);
    }
    arr
}

// Summary of W' depletion during the workout: how deep the athlete went and how often.
pub fn w_balance(wbal: &[f64], w_prime_j: MetricInt) -> MetricMap {
    let mut res: MetricMap = HashMap::new();
    if wbal.is_empty() || w_prime_j == 0 {
        return res;
    }

    let wp = w_prime_j as f64;
    let mut min_wpbal = wp;
    let mut time_below_75: u32 = 0;
    let mut time_below_50: u32 = 0;
    let mut time_below_25: u32 = 0;
    let mut depletion_count: u32 = 0;
    let mut was_below_50 = false;

    for &w in wbal {
        if w < min_wpbal { min_wpbal = w; }
        let frac = w / wp;
        if frac < 0.75 { time_below_75 += 1; }
        if frac < 0.50 {
            time_below_50 += 1;
            if !was_below_50 { depletion_count += 1; was_below_50 = true; }
        } else {
            was_below_50 = false;
        }
        if frac < 0.25 { time_below_25 += 1; }
    }

    res.insert(1, (min_wpbal / wp * 100.0).round() as u32);
    res.insert(2, time_below_75);
    res.insert(3, time_below_50);
    res.insert(4, time_below_25);
    res.insert(5, depletion_count);
    res
}

// W' balance over time as a percentage — shows when and how deep the athlete depleted their anaerobic reserve.
pub fn w_balance_graph(wbal: &[f64], w_prime_j: MetricInt) -> Graph {
    let wp = w_prime_j as f64;
    let points = wbal.iter().enumerate()
        .filter(|(i, _)| i % 10 == 0)
        .map(|(i, &w)| (i as f64 / 60.0, (w / wp) * 100.0))
        .collect();

    let mut series = HashMap::new();
    series.insert("W' Balance".to_string(), points);

    Graph {
        name: "W' Balance".to_string(),
        x_axis: Axis { label: "Time".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "W' Balance".to_string(), unit: Unit::Percent },
        series,
    }
}

// Best W' recovery (kJ) over a given time window — how well the athlete recovered between hard efforts.
pub fn w_recovery(wbal: &[f64], window_s: usize) -> MetricFloat {
    let mut best: f64 = 0.0;
    for i in 0..wbal.len().saturating_sub(window_s) {
        let gain = wbal[i + window_s] - wbal[i];
        if gain > best { best = gain; }
    }
    best / 1000.0 // kJ
}
  
    
pub fn power_time_series(data: &[FitRecord]) -> Graph {
    // Graf moči skozi čas (W vs čas v minutah).

    Graph {
        name: "Moč".to_string(),
        x_axis: Axis { label: "Čas".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "Moč".to_string(), unit: Unit::Watts },
        series: HashMap::new(),
    }
}

pub fn hr_time_series(data: &[FitRecord]) -> Graph {
    // Graf srčnega utripa skozi čas (bpm vs čas v minutah).

    Graph {
        name: "Srčni utrip".to_string(),
        x_axis: Axis { label: "Čas".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "Srčni utrip".to_string(), unit: Unit::Bpm },
        series: HashMap::new(),
    }
}

pub fn altitude_time_series(data: &[FitRecord]) -> Graph {
    // Graf nadmorske višine skozi čas (m vs čas v minutah).

    Graph {
        name: "Višina".to_string(),
        x_axis: Axis { label: "Čas".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "Višina".to_string(), unit: Unit::Meters },
        series: HashMap::new(),
    }
}

pub fn speed_time_series(data: &[FitRecord]) -> Graph {
    // Graf hitrosti skozi čas (km/h vs čas v minutah).

    Graph {
        name: "Hitrost".to_string(),
        x_axis: Axis { label: "Čas".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "Hitrost".to_string(), unit: Unit::MetersPerSecond },
        series: HashMap::new(),
    }
}

pub fn cadence_time_series(data: &[FitRecord]) -> Graph {
    // Graf kadence skozi čas (obr/min vs čas v minutah).

    Graph {
        name: "Kadenca".to_string(),
        x_axis: Axis { label: "Čas".to_string(), unit: Unit::Minutes },
        y_axis: Axis { label: "Kadenca".to_string(), unit: Unit::Rpm },
        series: HashMap::new(),
    }
}

pub fn total_distance(data: &[FitRecord]) -> MetricFloat {
    // Skupna razdalja treninga v kilometrih.

    0.0
}

pub fn total_elevation_gain(data: &[FitRecord]) -> MetricFloat {
    // Skupno višinsko pridobivanje v metrih.

    0.0
}

pub fn average_cadence(data: &[FitRecord]) -> MetricFloat {
    // Povprečna kadenca v obr/min med aktivno vožnjo.

    0.0
}

pub fn average_speed(data: &[FitRecord]) -> MetricFloat {
    // Povprečna hitrost v km/h med aktivno vožnjo (brez postankov).

    0.0
}

pub fn power_density_histogram(data: &[FitRecord]) {}
    
pub fn hr_density_histogram(data: &[FitRecord]) {}
    
pub fn compound_score(data: &[FitRecord]) {}
    
pub fn durability_ratio(data: &[FitRecord]) {}
    
pub fn power_coverage(data: &[FitRecord]) {}
    
pub fn hr_coverage(data: &[FitRecord]) {}
    
pub fn power_spike_count(data: &[FitRecord]) {}
    
pub fn hr_dropout_seconds(data: &[FitRecord]) {}
    
pub fn data_quality_score(data: &[FitRecord]) {}
    
pub fn load_ayes(data: &[FitRecord]) {}
    
pub fn workout_archetype(data: &[FitRecord]) {}
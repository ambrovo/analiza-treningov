use crate::fit_parser::FitRecord;
use std::collections::HashMap;

pub type MetricInt = u32;
pub type MetricMap = HashMap<MetricInt, MetricInt>;
pub type NestedMetricMap = HashMap<MetricInt, MetricMap>;


pub fn power_duration_curve(data: &[FitRecord]) -> MetricMap {
    // maksimalna moč glede na standardna časovna okna
    let durations: [MetricInt; 11] = [1, 3, 5, 30, 60, 120, 300, 600, 1800, 3600, 7200];
    
    let mut res: MetricMap = HashMap::new();

    for d in durations {
        
    

        let mut m = 0;
        for i   in d..(data.len() as MetricInt)  {
            if let (Some(curr), Some(prev)) = (data[i as usize].accumulated_power, data[(i - d) as usize].accumulated_power) {
                let diff = curr - prev;
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
    let power_acumulations = [1 ,1000 ,2000 ,3000 ];
     let mut res: NestedMetricMap = HashMap::new();
    for ac_power in power_acumulations {
        for j in 0..data.len() {
            if let Some(ac) = data[j].accumulated_power  {
                if ac > ac_power*1000 {
                    res.insert(ac_power,power_duration_curve(&data[j..]) );
                    break;
                }
            }
        }
    }
    return res; 

}

pub fn normalized_power(data: &[FitRecord]) {}

pub fn intensity_factor(data: &[FitRecord]) {}

pub fn training_stress_score(data: &[FitRecord]) {}

pub fn variability_index(data: &[FitRecord]) {}

pub fn power_zone_distribution(data: &[FitRecord]) {}

pub fn heart_rate_zone_distribution(data: &[FitRecord]){}

pub fn severe_domain_seconds(data: &[FitRecord]) {}

pub fn extreme_domain_seconds(data: &[FitRecord]) {}

pub fn total_power_seconds(data: &[FitRecord]) {}

pub fn total_work(data: &[FitRecord]) {}

pub fn peak_vam(data: &[FitRecord]) {}




pub fn fatigue_resistance_drops(data: &[FitRecord]){}

pub fn fatigue_resistance_index(data: &[FitRecord]) {}

pub fn aerobic_efficiency(data: &[FitRecord]) {}

pub fn aerobic_decoupling(data: &[FitRecord]) {}

pub fn hr_drift_rate(data: &[FitRecord]) {}

pub fn power_hr_slope(data: &[FitRecord]) {}

pub fn aerobic_quality_score(data: &[FitRecord]) {}

pub fn w_balance(data: &[FitRecord]) {}

pub fn w_recovery(data: &[FitRecord]) {}

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
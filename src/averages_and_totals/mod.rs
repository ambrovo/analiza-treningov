use crate::fit_parser::FitRecord;
use std::collections::HashMap;

pub type MetricInt = u32;
pub type MetricMap = HashMap<MetricInt, MetricInt>;
pub type NestedMetricMap = HashMap<MetricInt, MetricMap>;

pub struct FitData {
    pub data: Vec<FitRecord>,
}

impl FitData {
    pub fn power_duration_curve(&self, start_index: usize) -> MetricMap {
        // maksimalna moč glede na standardna časovna okna
        let durations: [MetricInt; 11] = [1, 3, 5, 30, 60, 120, 300, 600, 1800, 3600, 7200];
        
        let mut res: MetricMap = HashMap::new();
        
        for d in durations {
            
            
            
            let mut m = 0;
            let start = start_index + (d as usize);
            for i   in start..(self.data.len())  {
                if let (Some(curr), Some(prev)) = (
                    self.data[i as usize].accumulated_power, 
                    self.data[(i - (d as usize)) as usize].accumulated_power
                ) {
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
    
    
    pub fn fatigued_pdc(&self) -> NestedMetricMap {
        //Maksimalne moči dobimo po različno opravljenem delu
        let power_acumulations = [1 ,1000 ,2000 ,3000 ];
        let mut res: NestedMetricMap = HashMap::new();
        for ac_power in power_acumulations {
            for j in 0..self.data.len() {
                if let Some(ac) = self.data[j].accumulated_power  {
                    if ac > ac_power*1000 {
                        res.insert(ac_power, self.power_duration_curve(j));
                        break;
                    }
                }
            }
        }
        return res; 
        
    }
    
    pub fn normalized_power(&self) -> MetricInt {
        //Fiziološko prilagojena moč, ki upošteva variabilnst treninga
        //Formula ki je bila uporabljena je $NP = (\frac{1}{N} \sum_{i=1}^{N}(P_{30s, i})^4)^{\frac{1}{4}}$, 
        //kjer je P_{30s, i} povprečna moč v zadnjih 30 sekundah pri času i, N pa število 30-sekundnih povprečij.
        let time = 30;

        if self.data.len() < time {
            return 0;
        }

        let power: Vec<f32> = self.data.iter().map(|r| r.power.unwrap_or(0) as f32).collect();

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
    
    pub fn intensity_factor(&self, ftp: MetricInt) -> MetricInt {
        //Relativna intenziteta glede na FTP (NP®/FTP)
        // $IF = \frac{NP}{FTP}$
        if ftp == 0 {
            return 0;
        }

        let np = self.normalized_power();
        (np * 1000) / ftp
    }
    
    pub fn training_stress_score(&self, ftp: MetricInt) -> MetricInt {
        //Obremenitev treninga (1 ura pri FTP = 100 TSS®)
        // $TSS = \frac{duration * NP^2}{FTP^2 * 3600} * 100$
        if ftp == 0 {
            return 0;
        }

        let duration = self.data.len() as f32;
        let np = self.normalized_power() as f32;
        let ftp = ftp as f32;

        let tss = (duration * np * np) / (ftp * ftp * 3600.0) * 100.0;

        (tss * 10.0).round() as u32
    }
    
    pub fn variability_index(&self) -> MetricInt {
        //Mera variabilnosti moči (NP®/avg_power) - ali je trening "steady"
        // $VI = \frac{NP}{average power}$
        let mut sum: u32 = 0;
        let mut count: u32 = 0;

        for r in &self.data {
            if let Some(p) = r.power {
                sum += p as u32;
                count += 1;
            }
        }

        if count == 0 {
            return 0;
        }

        let avg_power = sum as f32 / count as f32;
        let np = self.normalized_power() as f32;

        if avg_power == 0.0 {
            return 0;
        }

        let vi = np / avg_power;

        (vi * 1000.0).round() as u32
    }
    
    pub fn power_zone_distribution(&self, ftp: MetricInt) -> MetricMap {
        //Čas (sekunde) v vsaki moč coni (Z1-Z7)
        let mut res: MetricMap = HashMap::new();

        if ftp == 0 {
            return res;
        }

        for r in &self.data {
            if let Some(p) = r.power {
                let ratio = p as f32 / ftp as f32;
                let zone = if ratio < 0.55 {
                    1
                } else if ratio < 0.75 {
                    2
                } else if ratio < 0.90 {
                    3
                } else if ratio < 1.05 {
                    4
                } else if ratio < 1.20 {
                    5
                } else if ratio < 1.50 {
                    6
                } else {
                    7
                };

                *res.entry(zone).or_insert(0) += 1;
            }
        };
        
        res
    }
    
    pub fn heart_rate_zone_distribution(&self, max_hr: MetricInt) -> MetricMap {
        //Čas (sekunde) v vsaki srčni coni (Z1-Z5)
        let mut res: MetricMap = HashMap::new();

        if max_hr == 0 {
            return res;
        }

        for r in &self.data {
            if let Some(hr) = r.heart_rate {
                let ratio = hr as f32 / max_hr as f32;
                let zone = if ratio < 0.60 {
                    1
                } else if ratio < 0.70 {
                    2
                } else if ratio < 0.80 {
                    3
                } else if ratio < 0.90 {
                    4
                } else {
                    5
                };

                *res.entry(zone).or_insert(0) += 1;
            }
        }

        res
    }
    
    pub fn severe_domain_seconds(&self, ftp: MetricInt) -> MetricInt {
        //Sekunde z močjo nad FTP
        if ftp == 0 {
            return 0;
        };

        let mut seconds: u32 = 0;

        for r in &self.data {
            if let Some(p) = r.power {
                if p as u32 > ftp {
                    seconds += 1
                }
            }
        };

        seconds
    }
    
    pub fn extreme_domain_seconds(&self, ftp: MetricInt) -> MetricInt {
        //Sekunde z močjo nad 150% FTP (nevromuskularna cona)
        if ftp == 0 {
            return 0;
        };

        let mut seconds: u32 = 0;

        for r in &self.data {
            if let Some(p) = r.power {
                if p as f32 > (ftp as f32 * 3.0) / 2.0 {
                    seconds += 1
                }
            }
        };

        seconds
    }
    
    pub fn total_power_seconds(&self) -> MetricInt {
        //Skupno število sekund z veljavnimi podatki o moči
        let mut seconds: u32 = 0;

        for r in &self.data {
            if r.power.is_some() {
                seconds += 1
            }
        };

        seconds
    }
    
    pub fn total_work(&self) -> MetricInt {
        //Skupno mehansko delo proizvedeno med treningom
        let total_joules: u32 = self.data
        .iter()
        .filter_map(|r| r.power)
        .map(|p| p as u32)
        .sum();

        (total_joules / 1000) as u32
    }
    
    pub fn peak_vam(&self) -> MetricMap {
        //Maksimalna hitrost vzpenjanja (m/h) na 5min, 10min, 20min
        // $VAM = \frac{\delta h}{\delta t} * 3600$
        let duration: [MetricInt; 3] = [300, 600, 1200];
        let mut res: MetricMap = HashMap::new();

        for d in duration {
            let mut max_vam = 0.0;

            if self.data.len() < d as usize {
                continue;
            }

            for i in d as usize..self.data.len() {
                if let (Some(curr_alt), Some(prev_alt)) = (
                    self.data[i].enhanced_altitude,
                    self.data[i - d as usize].enhanced_altitude,
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
    
    
    
    
    pub fn fatigue_resistance_drops(&self) -> NestedMetricMap {
        let mut res:NestedMetricMap = HashMap::new();
        let fresh
    }
    
    pub fn fatigue_resistance_index(&self) -> MetricInt {
        to_do()
    }
    
    pub fn aerobic_efficiency(&self) -> MetricInt {}
    
    pub fn aerobic_decoupling(&self) {}
    
    pub fn hr_drift_rate(&self) {}
    
    pub fn power_hr_slope(&self) {}
    
    pub fn aerobic_quality_score(&self) {}
    
    pub fn w_balance(&self) {}
    
    pub fn w_recovery(&self) {}
    
    pub fn power_density_histogram(&self) {}
    
    pub fn hr_density_histogram(&self) {}
    
    pub fn compound_score(&self) {}
    
    pub fn durability_ratio(&self) {}
    
    pub fn power_coverage(&self) {}
    
    pub fn hr_coverage(&self) {}
    
    pub fn power_spike_count(&self) {}
    
    pub fn hr_dropout_seconds(&self) {}
    
    pub fn data_quality_score(&self) {}
    
    pub fn load_ayes(&self) {}
    
    pub fn workout_archetype(&self) {}
}
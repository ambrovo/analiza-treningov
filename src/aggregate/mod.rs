use crate::averages_and_totals::{MetricFloat, MetricMap, Graph, Axis, Unit, PowerZoneThresholds, HrZoneTresholds};
use crate::TotalAnalysisResult;
use std::collections::HashMap;

// Najboljša krivulja moč-čas čez vse treninge
pub fn best_pdc(results: &[TotalAnalysisResult]) -> MetricMap {
    HashMap::new()
}

// Kronična obremenitev (CTL) — 42-dnevno eksponentno povprečje TSS (fitnes)
pub fn ctl_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "CTL".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "CTL".to_string(), unit: Unit::Custom("TSS/dan".to_string()) },
        series: HashMap::new(),
    }
}

// Akutna obremenitev (ATL) — 7-dnevno eksponentno povprečje TSS (utrujenost)
pub fn atl_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "ATL".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "ATL".to_string(), unit: Unit::Custom("TSS/dan".to_string()) },
        series: HashMap::new(),
    }
}

// Forma (TSB = CTL - ATL) — pozitivno = spočit, negativno = utrujen
pub fn tsb_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "TSB".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "TSB".to_string(), unit: Unit::Custom("TSS".to_string()) },
        series: HashMap::new(),
    }
}

//  TSS — graf
pub fn tss_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "Tedenski TSS".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "TSS".to_string(), unit: Unit::Custom("TSS".to_string()) },
        series: HashMap::new(),
    }
}

// volumen v kJ
pub fn work_kj_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "Tedensko delo".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "Delo".to_string(), unit: Unit::Custom("kJ".to_string()) },
        series: HashMap::new(),
    }
}

// volumen v urah
pub fn hours_series(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "Ure".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "Ure".to_string(), unit: Unit::Hours },
        series: HashMap::new(),
    }
}

// Trend aerobne učinkovitosti (EF) skozi čas
pub fn aerobic_efficiency_trend(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "Aerobna učinkovitost".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "EF".to_string(), unit: Unit::Custom("W/bpm".to_string()) },
        series: HashMap::new(),
    }
}

// Skupna porazdelitev časa po conah moči čez vse treninge
pub fn total_power_zone_distribution(results: &[TotalAnalysisResult]) -> PowerZoneThresholds {
    PowerZoneThresholds { zone_1: 0, zone_2a: 0, zone_2b: 0, zone_3: 0, zone_4: 0, zone_5: 0, zone_6: 0, zone_7: 0 }
}

// Skupna porazdelitev časa po conah srčnega utripa čez vse treninge
pub fn total_hr_zone_distribution(results: &[TotalAnalysisResult]) -> HrZoneTresholds {
    HrZoneTresholds { zone_1: 0, zone_2a: 0, zone_2b: 0, zone_3: 0, zone_4: 0, zone_5: 0 }
}

// Trend normalizirane moči skozi čas
pub fn np_trend(results: &[TotalAnalysisResult]) -> Graph {
    Graph {
        name: "NP trend".to_string(),
        x_axis: Axis { label: "Datum".to_string(), unit: Unit::Custom("dan".to_string()) },
        y_axis: Axis { label: "NP".to_string(), unit: Unit::Watts },
        series: HashMap::new(),
    }
}

// Skupna prevožena razdalja in višinska razlika
pub fn total_distance_and_elevation(results: &[TotalAnalysisResult]) -> (MetricFloat, MetricFloat) {
    // Seštej total_distance_km in total_elevation_gain čez vse rezultate
    (0.0, 0.0)
}

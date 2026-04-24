mod fit_parser; // assuming fit_parser.rs is in the same folder
pub mod averages_and_totals;
mod testi;
use std::fs;
use std::time::Instant;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::averages_and_totals::{NestedMetricMap, fatigued_pdc , power_duration_curve};


pub struct GraphData {
    pub data_fatigued_pdc : NestedMetricMap,
}

fn test(path: &str) -> Result<GraphData, Box<dyn std::error::Error>> {
    let data: Vec<fit_parser::FitRecord> = fit_parser::parse_fit_file(path)?;

   
    let result = GraphData  {
        data_fatigued_pdc : fatigued_pdc(&data)
    };
    Ok(result)
    
}

fn test_all(folder: &str) -> Result<NestedMetricMap, Box<dyn std::error::Error>> {
    let files: Vec<_> = fs::read_dir(folder)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "gz"))
        .collect();

    let results: Vec<GraphData> = files.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            test(&path.to_string_lossy()).ok()
        })
        .collect();

    let mut merged: NestedMetricMap = HashMap::new();
    for result in results {
        for (outer_key, inner_map) in result.data_fatigued_pdc {
            let entry = merged.entry(outer_key).or_insert_with(HashMap::new);
            for (inner_key, value) in inner_map {
                entry.entry(inner_key)
                    .and_modify(|curr| *curr = (*curr).max(value))
                    .or_insert(value);
            }
        }
    }

    // print results
    let mut outer_keys: Vec<_> = merged.keys().collect();
    outer_keys.sort();
    for &outer in &outer_keys {
        println!("\nAfter {}kJ:", outer);
        let inner = &merged[&outer];
        let mut inner_keys: Vec<_> = inner.keys().collect();
        inner_keys.sort();
        for &duration in &inner_keys {
            println!("  {}s: {} W", duration, inner[&duration]);
        }
    }

    Ok(merged)
}

fn main() {
    let start = Instant::now();
    let _ = test_all("test");
    println!("Total time: {:?}", start.elapsed());
}
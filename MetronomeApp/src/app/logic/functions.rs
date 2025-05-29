use std::f64::consts::E;

use crate::app::{GrowthType, TempoParams};

pub fn calculate(growth_type: GrowthType, time: f64, params: TempoParams) -> f64 {
    match growth_type {
        GrowthType::Linear => linear(time, params),
        GrowthType::Sigmoidal => sigmoidal(time, params),
        GrowthType::Logarithmic => logarithmic(time, params),
        GrowthType::Exponential => exponential(time, params),
        GrowthType::Constant => constant(params),
    }
}

fn linear(x: f64, p: TempoParams) -> f64 {
    let normalized = x / p.length as f64;
    return (1.0 - normalized) * p.min as f64 + normalized * p.max as f64;
}

fn sigmoidal(x: f64, p: TempoParams) -> f64 {
    let normalized = x / p.length as f64;
    let shape = 1.0 / (1.0 + E.powf(-p.scaler * (normalized - 0.5)));
    interpolate(p.min, p.max, shape)
}

fn logarithmic(x: f64, p: TempoParams) -> f64 {
    let normalized = x / p.length as f64;
    let shape = normalized.powf(p.scaler); // value from 0 to 1
    interpolate(p.min, p.max, shape)
}

fn exponential(x: f64, p: TempoParams) -> f64 {
    let normalized = x / p.length as f64;
    let shape = normalized.powf(p.scaler); // value from 0 to 1
    interpolate(p.min, p.max, shape)
}

fn constant(p: TempoParams) -> f64 {
    return p.min as f64;
}

fn interpolate(start: u32, end: u32, t: f64) -> f64 {
    (1.0 - t) * start as f64 + t * end as f64
}

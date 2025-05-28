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
    let slope = ((p.max - p.min) / p.length) as f64;
    return slope * x + (p.min as f64);
}

fn sigmoidal(x: f64, p: TempoParams) -> f64 {
    return 0.0;
}

fn logarithmic(x: f64, p: TempoParams) -> f64 {
    return 0.0;
}

fn exponential(x: f64, p: TempoParams) -> f64 {
    let normalized = x / p.length as f64;
    return (1.0 - normalized).powf(p.scaler)
        + (p.max - p.min) as f64 * normalized.powf(p.scaler)
        + p.min as f64;
}

fn constant(p: TempoParams) -> f64 {
    return 0.0;
}

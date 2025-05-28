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
    return 0.0;
}

fn sigmoidal(x: f64, p: TempoParams) -> f64 {
    return 0.0;
}

fn logarithmic(x: f64, p: TempoParams) -> f64 {
    return 0.0;
}

fn exponential(x: f64, p: TempoParams) -> f64 {
    return 0.0;
}

fn constant(p: TempoParams) -> f64 {
    return 0.0;
}

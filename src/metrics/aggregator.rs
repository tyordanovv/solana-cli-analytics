use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::models::*;

pub trait TimeSeriesMetric {
    fn add_sample(&mut self, value: f64, timestamp: Instant);
    fn get_average(&self, window: Duration) -> Option<f64>;
    fn get_percentile(&self, percentile: f64, window: Duration) -> Option<f64>;
    fn get_trend(&self, window: Duration) -> Option<f64>;
    fn clear_old_samples(&mut self, older_than: Duration);
}

pub struct NumericTimeSeries {
    samples: VecDeque<(f64, Instant)>,
    max_samples: usize,
}

impl NumericTimeSeries {
    pub fn new(max_samples: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }
}

impl TimeSeriesMetric for NumericTimeSeries {
    fn add_sample(&mut self, value: f64, timestamp: Instant) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back((value, timestamp));
    }
    
    fn get_average(&self, window: Duration) -> Option<f64> {
        todo!()
    }
    
    fn get_percentile(&self, percentile: f64, window: Duration) -> Option<f64> {
        todo!()
    }
    
    fn get_trend(&self, window: Duration) -> Option<f64> {
        todo!()
    }
    
    fn clear_old_samples(&mut self, older_than: Duration) {
        let now = Instant::now();
        while let Some((_, timestamp)) = self.samples.front() {
            if now.duration_since(*timestamp) > older_than {
                self.samples.pop_front();
            } else {
                break;
            }
        }
    }
}
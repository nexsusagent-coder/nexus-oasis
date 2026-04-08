//! Time Series Analysis

use serde::{Deserialize, Serialize};

/// Time series data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries<T> {
    data: Vec<T>,
    max_size: usize,
}

impl<T: Clone> TimeSeries<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.data.len() >= self.max_size {
            self.data.remove(0);
        }
        self.data.push(value);
    }

    pub fn latest(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl TimeSeries<f64> {
    /// Calculate statistics
    pub fn statistics(&self) -> SeriesStats {
        if self.data.is_empty() {
            return SeriesStats::default();
        }

        let n = self.data.len();
        let sum: f64 = self.data.iter().sum();
        let mean = sum / n as f64;

        let variance = if n > 1 {
            self.data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };

        SeriesStats {
            mean,
            stddev: variance.sqrt(),
            count: n,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeriesStats {
    pub mean: f64,
    pub stddev: f64,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_series() {
        let mut ts = TimeSeries::new(10);
        ts.push(1.0);
        ts.push(2.0);
        ts.push(3.0);
        
        assert_eq!(ts.len(), 3);
        assert_eq!(ts.latest(), Some(&3.0));
    }

    #[test]
    fn test_statistics() {
        let mut ts = TimeSeries::new(10);
        for i in 1..=10 {
            ts.push(i as f64);
        }
        
        let stats = ts.statistics();
        assert!(stats.mean > 0.0);
    }
}

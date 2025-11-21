
#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
    window_size: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64, window_size: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
            window_size,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn period(&self) -> f64 {
        self.period
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let adjusted_x = self.x - self.window_size;
        let point = if self.x < 0.0 {
            (self.x, 0.0)
        } else {
            (self.x, (adjusted_x * 2.0 * std::f64::consts::PI / self.period).sin() * self.scale)
        };
        self.x += self.interval;
        Some(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin_signal() {
        const WINDOW_SIZE: f64 = 1800.0;
        let mut signal = SinSignal::new(1.0, 360.0, 10.0, WINDOW_SIZE);
        let first_point = signal.next().unwrap();
        assert_eq!(first_point.0, 0.0);
        assert!((first_point.1 - 0.0).abs() < 1e-9);

        // We subtract WINDOW_SIZE in the implementation, so we need to account for that.
        let x = 90.0;
        let adjusted_x = x - WINDOW_SIZE;
        let expected_y = (adjusted_x * 2.0 * std::f64::consts::PI / 360.0).sin() * 10.0;

        let point_90 = signal.by_ref().skip(89).next().unwrap();
        assert_eq!(point_90.0, 90.0);
        assert!((point_90.1 - expected_y).abs() < 1e-9);
    }
}

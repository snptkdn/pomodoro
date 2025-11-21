
use crate::domain::signal::SinSignal;

const WINDOW_SIZE: usize = 1800;

pub struct App {
    pub signal1: SinSignal,
    pub data1: Vec<(f64, f64)>,
    pub signal2: SinSignal,
    pub data2: Vec<(f64, f64)>,
    pub signal3: SinSignal,
    pub data3: Vec<(f64, f64)>,
    pub window: [f64; 2],
    pub y_bounds: [f64; 2],
}

impl App {
    pub fn new() -> App {
        let one_minutes = 60.0;
        let mut signal1 = SinSignal::new(1.0, one_minutes*5.0, 18.0, WINDOW_SIZE as f64); // 5min
        let mut signal2 = SinSignal::new(1.0, one_minutes * 25.0, 15.0, WINDOW_SIZE as f64); // 25min
        let mut signal3 = SinSignal::new(1.0, one_minutes * 30.0, 10.0, WINDOW_SIZE as f64); // 30min
        let data1 = signal1.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        let data3 = signal3.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        App {
            signal1,
            data1,
            signal2,
            data2,
            signal3,
            data3,
            window: [0.0, WINDOW_SIZE as f64],
            y_bounds: [-20.0, 20.0],
        }
    }

    pub fn on_tick(&mut self) {
        self.data1.remove(0);
        self.data1.extend(self.signal1.by_ref().take(1));
        self.data2.remove(0);
        self.data2.extend(self.signal2.by_ref().take(1));
        self.data3.remove(0);
        self.data3.extend(self.signal3.by_ref().take(1));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert_eq!(app.window[0], 0.0);
        assert_eq!(app.window[1], WINDOW_SIZE as f64);
        assert_eq!(app.y_bounds[0], -20.0);
        assert_eq!(app.y_bounds[1], 20.0);
        assert_eq!(app.data1.len(), WINDOW_SIZE);
        assert_eq!(app.data2.len(), WINDOW_SIZE);
        assert_eq!(app.data3.len(), WINDOW_SIZE);
    }
}

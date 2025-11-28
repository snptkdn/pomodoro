use crate::domain::pomodoro::{Pomodoro, PomodoroPhase, SinSignal};

const WINDOW_SIZE: usize = 1800;

pub struct App {
    pub pomodoro: Pomodoro,
    pub signal1: SinSignal,
    pub data1: Vec<(f64, f64)>,
    pub signal2: SinSignal,
    pub data2: Vec<(f64, f64)>,
    pub signal3: SinSignal,
    pub data3: Vec<(f64, f64)>,
    pub window: [f64; 2],
    pub cycle_markers: Vec<(f64, f64)>,
}

impl App {
    pub fn new() -> App {
        let one_minute = 60.0;
        let pomodoro = Pomodoro::new();
        let mut signal1 = SinSignal::new(1.0, one_minute * 5.0, 18.0, WINDOW_SIZE); // 5min
        let mut signal2 = SinSignal::new(1.0, one_minute * 25.0, 15.0, WINDOW_SIZE); // 25min
        let mut signal3 = SinSignal::new(1.0, one_minute * 30.0, 10.0, WINDOW_SIZE); // 30min
        let data1 = signal1.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        let data3 = signal3.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        App {
            pomodoro,
            signal1,
            data1,
            signal2,
            data2,
            signal3,
            data3,
            window: [0.0, WINDOW_SIZE as f64],
            cycle_markers: vec![],
        }
    }

    pub fn on_tick(&mut self) {
        if self.pomodoro.timer <= 1.0 {
            let data_to_mark = match self.pomodoro.phase {
                PomodoroPhase::Work => &self.data2,
                PomodoroPhase::ShortBreak => &self.data1,
                PomodoroPhase::LongBreak => &self.data3,
            };
            if let Some(last_point) = data_to_mark.last() {
                self.cycle_markers.push(*last_point);
            }
        }

        self.pomodoro.tick();

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

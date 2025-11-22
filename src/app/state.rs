use crate::domain::pomodoro::{Pomodoro, PomodoroPhase, SinSignal};

const WINDOW_SIZE: usize = 1800;

pub struct App {
    pub pomodoro: Pomodoro,
    pub signal: SinSignal,
    pub data: Vec<(f64, f64)>,
    pub window: [f64; 2],
    pub cycle_markers: Vec<(f64, f64)>,
    current_phase: PomodoroPhase,
}

impl App {
    pub fn new() -> App {
        let one_minute = 60.0;
        let pomodoro = Pomodoro::new();
        let mut signal = SinSignal::new(1.0, one_minute * 25.0, 15.0, WINDOW_SIZE); // Default to 25min work
        let data = signal.by_ref().take(WINDOW_SIZE).collect::<Vec<(f64, f64)>>();
        App {
            pomodoro,
            signal,
            data,
            window: [0.0, WINDOW_SIZE as f64],
            cycle_markers: vec![],
            current_phase: PomodoroPhase::Work,
        }
    }

    pub fn on_tick(&mut self) {
        if self.pomodoro.timer <= 1.0 {
            if let Some(last_point) = self.data.last() {
                self.cycle_markers.push(*last_point);
            }
        }

        self.pomodoro.tick();

        if self.current_phase != self.pomodoro.phase {
            let one_minute = 60.0;
            let (period, scale) = match self.pomodoro.phase {
                PomodoroPhase::Work => (one_minute * 25.0, 15.0),
                PomodoroPhase::ShortBreak => (one_minute * 5.0, 18.0),
                PomodoroPhase::LongBreak => (one_minute * 30.0, 10.0),
            };
            self.signal = SinSignal::new(1.0, period, scale, WINDOW_SIZE);
            self.current_phase = self.pomodoro.phase;
        }

        self.data.remove(0);
        self.data.extend(self.signal.by_ref().take(1));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

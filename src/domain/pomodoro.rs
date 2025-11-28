#[derive(Clone, Copy, PartialEq)]
pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct Pomodoro {
    pub phase: PomodoroPhase,
    pub timer: f64,
    pub work_duration: f64,
    pub short_break_duration: f64,
    pub long_break_duration: f64,
    work_sessions_before_long_break: u32,
    work_sessions_completed: u32,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro {
            phase: PomodoroPhase::Work,
            timer: 25.0 * 60.0, // Default to 25 minutes
            work_duration: 25.0 * 60.0,
            short_break_duration: 5.0 * 60.0,
            long_break_duration: 30.0 * 60.0,
            work_sessions_before_long_break: 4,
            work_sessions_completed: 0,
        }
    }

    pub fn tick(&mut self) {
        self.timer -= 1.0;
        if self.timer <= 0.0 {
            self.next_phase();
        }
    }

    fn next_phase(&mut self) {
        match self.phase {
            PomodoroPhase::Work => {
                self.work_sessions_completed += 1;
                if self.work_sessions_completed >= self.work_sessions_before_long_break {
                    self.phase = PomodoroPhase::LongBreak;
                    self.timer = self.long_break_duration;
                    self.work_sessions_completed = 0;
                } else {
                    self.phase = PomodoroPhase::ShortBreak;
                    self.timer = self.short_break_duration;
                }
            }
            PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
                self.phase = PomodoroPhase::Work;
                self.timer = self.work_duration;
            }
        }
    }
}


#[derive(Clone)]
pub struct SinSignal {
    pub x: f64,
    interval: f64,
    period: f64,
    scale: f64,
    window_size: usize,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64, window_size: usize) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
            window_size,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let adjusted_x = self.x - self.window_size as f64;
        let point = if self.x < 0.0 {
            (self.x, 0.0)
        } else {
            (
                self.x,
                (adjusted_x * 2.0 * std::f64::consts::PI / self.period).sin() * self.scale,
            )
        };
        self.x += self.interval;
        Some(point)
    }
}

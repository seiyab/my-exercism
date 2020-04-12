use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, min) = cyclic_int(minutes, 60);
        let (_, hr) = cyclic_int(hours+h, 24);
        Clock {
            hours: hr,
            minutes: min,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

fn cyclic_int(n: i32, period: i32) -> (i32, i32) {
    let phase = modulo(n, period);
    let cycle = div(n, period);
    (cycle, phase)
}

fn modulo(n: i32, m: i32) -> i32 {
    (n % m + m) % m
}

fn div(n: i32, m: i32) -> i32 {
    (n - modulo(n, m)) / m
}

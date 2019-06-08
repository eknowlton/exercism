use std::fmt;

pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = hours * 60 + minutes;
        Clock { minutes: minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: self.minutes + minutes,
        }
    }

    pub fn to_hours(&self) -> i32 {
        (self.minutes / 60) % 24
    }

    pub fn to_minutes(&self) -> i32 {
        let hours = self.minutes as f32 / 60.0;
        let minutes_left = hours - hours.floor();

        let minutes = minutes_left * 60.0;

        minutes.round() as i32
    }

    pub fn to_time(&self) -> (i32, i32) {
        let mut hours = self.to_hours();
        let minutes = self.to_minutes();

        if hours < 0 {
            hours = 23 + hours;
        }

        (hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes) = self.to_time();
        write!(
            f,
            "{}:{}",
            format!("{:0>2}", hours),
            format!("{:0>2}", minutes)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_time() == other.to_time()
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clock {{ {} }}", self.to_string())
    }
}

use std::fmt;

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = hours * HOUR + minutes;
        Clock { minutes }
    }

    pub fn hour(&self) -> i32 {
        let minutes = self.minutes % DAY;

        match minutes {
            minutes if minutes >= 0 => minutes / HOUR,
            minutes => (DAY + minutes) / HOUR,
        }
    }

    pub fn minute(&self) -> i32 {
        let minutes = self.minutes % 60;

        match minutes {
            m if m < 0 => 60 + minutes,
            m => m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: self.minutes + minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            format!("{:0>2}", self.hour()),
            format!("{:0>2}", self.minute())
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hour(), self.minute()) == (other.hour(), other.minute())
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clock {{ {} }}", self.to_string())
    }
}

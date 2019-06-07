use std::fmt;

pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = match hours {
            h if h > 0 => h + minutes / 60,
            h if h > -23 => 24 + (h + minutes / 60),
            h => (24 + (h + minutes / 60)) * -1,
        };
        let minutes = minutes % 60;

        Clock(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours: i32 = minutes / 60 + self.0;
        let minutes: i32 = minutes % 60 + self.1;

        let new_hours = hours + minutes / 60;
        let new_minutes = minutes % 60;

        Clock(new_hours, new_minutes)
    }

    pub fn hour_digits(&self) -> i32 {
        match self {
            Clock(h, _) if h > &24 => h % 24,
            Clock(h, _) if h > &12 => 24 - h,
            Clock(h, _) => *h,
        }
    }

    pub fn minute_digits(&self) -> i32 {
        match self {
            Clock(_, m) if m > &60 => m % 60,
            Clock(_, m) => *m,
        }
    }

    pub fn to_digits(&self) -> (i32, i32) {
        match self {
            Clock(h, m) if h > &24 => (h % 24, *m),
            Clock(h, m) if h > &12 => (24 - h, *m),
            Clock(h, m) => (*h, *m),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes) = self.to_digits();
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
        match self.to_digits() {
            (h, m) if m == other.minute_digits() && h == other.hour_digits() => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes) = self.to_digits();
        write!(f, "Clock {{ hours: {}, minutes: {} }}", hours, minutes)
    }
}

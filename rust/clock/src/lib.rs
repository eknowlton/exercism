use std::fmt;

pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const TWENTY_FOUR_HOURS: i32 = 24 * 60;
        let days_in_hours = hours / 24 + 1;
        let minutes = (hours * 60) + minutes + (TWENTY_FOUR_HOURS * days_in_hours);
        Clock { minutes: minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: self.minutes + minutes,
        }
    }

    pub fn to_time(&self) -> (i32, i32) {
        let mut hour = (self.minutes / 60) % 24;
        let minutes = {
            let hours = self.minutes as f32 / 60.0;
            let minutes_left = hours - hours.floor();

            (minutes_left * 60.0).round()
        } as i32;

        // test_negative_hour_and_minutes_both_roll_over
        if hour < 0 {
            hour = 23 + hour;
        }

        (hour, minutes)
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

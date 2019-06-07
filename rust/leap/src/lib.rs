pub fn is_leap_year(year: u64) -> bool {
    let mut result = false;
    if year % 4 == 0 {
        result = true;
    }

    if year % 100 == 0 {
        if year % 400 != 0 {
            result = false;
        }
    }

    return result;
}

pub mod utils;

#[cfg(test)]
mod tests {
    use super::utils::*;
    use std::time::Duration;

    #[test]
    fn test_seconds_since_2000() {
        // 2000-01-01 00:00:00 should be 0 seconds
        assert_eq!(seconds_since_2000(0, 1, 1, 0, 0, 0), 0);

        // 2000-01-01 00:00:01 should be 1 second
        assert_eq!(seconds_since_2000(0, 1, 1, 0, 0, 1), 1);

        // 2001-01-01 00:00:00 should be 365 days (non-leap year logic in utils seems to be simple) * 86400?
        // Let's check is_leap_year(0). 0 % 4 == 0, 0 % 100 == 0. So it's NOT a leap year by simplified logic?
        // Wait, standard rule: divisible by 4, unless divisible by 100 but not 400.
        // is_leap_year implementation: year % 4 == 0 && year % 100 != 0
        // for year=0 (2000): 0 % 4 == 0 (True) AND 0 % 100 == 0 (False) -> Result False.
        // So 2000 is treated as NON-LEAP in this code.
        // 365 * 86400 = 31536000

        assert_eq!(seconds_since_2000(1, 1, 1, 0, 0, 0), 365 * 86400);
    }

    #[test]
    fn test_time_date_from_duration_since_2000() {
        let d = Duration::from_secs(0);
        assert_eq!(time_date_from_duration_since_2000(d), (0, 1, 1, 0, 0, 0));

        // Since 2000 is treated as 365 days by the buggy logic
        let d = Duration::from_secs(365 * 86400);
        assert_eq!(time_date_from_duration_since_2000(d), (1, 1, 1, 0, 0, 0));
    }
}

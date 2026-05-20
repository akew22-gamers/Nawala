use chrono::{DateTime, Duration, Utc};

pub fn next_lock_until(failed_attempts: i64, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    match failed_attempts {
        0..=4 => None,
        5..=9 => Some(now + Duration::minutes(5)),
        10..=14 => Some(now + Duration::minutes(30)),
        _ => Some(now + Duration::hours(24)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn locks_after_five_failures() {
        let now = Utc::now();
        
        // 4 failures should not lock
        let result_4 = next_lock_until(4, now);
        assert!(result_4.is_none(), "Expected no lock for 4 failures");
        
        // 5 failures should lock for 5 minutes
        let result_5 = next_lock_until(5, now);
        assert!(result_5.is_some(), "Expected lock for 5 failures");
        
        let expected = now + Duration::minutes(5);
        assert_eq!(result_5.unwrap(), expected, "Expected 5 minute lock");
    }
}

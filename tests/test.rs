#[cfg(test)]
use rdspy::RdsGroupIterator;

mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use std::io::Cursor;

    // Helper to create a reader from a string
    fn from_str(s: &str) -> RdsGroupIterator<Cursor<Vec<u8>>> {
        RdsGroupIterator::new(Cursor::new(s.as_bytes().to_vec()))
    }

    #[test]
    fn test_normal_line() {
        let input = "C6A8 012A E0CD 6572 @2019/05/05 09:31:10.85\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();

        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(item.b, Some(0x012A));
        assert_eq!(item.c, Some(0xE0CD));
        assert_eq!(item.d, Some(0x6572));

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5).unwrap());
        assert_eq!(
            item.time,
            NaiveTime::from_hms_milli_opt(9, 31, 10, 850).unwrap()
        );

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_some_missing_values() {
        let input = "C6A8 ---- E0CD ---- @2019/05/05 09:31:10.96\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();

        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(item.b, None);
        assert_eq!(item.c, Some(0xE0CD));
        assert_eq!(item.d, None);

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5).unwrap());
        assert_eq!(
            item.time,
            NaiveTime::from_hms_milli_opt(9, 31, 10, 960).unwrap()
        );
    }

    #[test]
    fn test_all_missing() {
        let input = "---- ---- ---- ---- @2020/12/31 23:59:59.999\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();

        assert_eq!(item.a, None);
        assert_eq!(item.b, None);
        assert_eq!(item.c, None);
        assert_eq!(item.d, None);

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2020, 12, 31).unwrap());
        assert_eq!(
            item.time,
            NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap()
        );
    }

    #[test]
    fn test_invalid_hex() {
        let input = "ZZZZ 012A E0CD 6572 @2019/05/05 09:31:10.85\n";
        let mut iter = from_str(input);

        let err = iter.next().unwrap().unwrap_err();
        assert!(err.to_string().contains("invalid hex")); // adjust based on your error message
    }

    #[test]
    fn test_malformed_timestamp() {
        let input = "C6A8 012A E0CD 6572 @2019-13-45 25:70:99.123\n";
        let mut iter = from_str(input);

        let err = iter.next().unwrap().unwrap_err();
        assert!(err.to_string().contains("date") || err.to_string().contains("time"));
    }

    #[test]
    fn test_empty_line_skipped() {
        let input = r#"
C6A8 012A E0CD 6572 @2019/05/05 09:31:10.85

---- ---- ---- ---- @2019/05/05 09:31:11.00
"#;
        let mut iter = from_str(input);

        // First valid line
        let item1 = iter.next().unwrap().unwrap();
        assert_eq!(item1.a, Some(0xC6A8));

        // Second valid line
        let item2 = iter.next().unwrap().unwrap();
        assert_eq!(item2.a, None);

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_trailing_garbage_ignored() {
        let input = "C6A8 012A E0CD 6572 @2019/05/05 09:31:10.85 some comment\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();
        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(
            item.time,
            NaiveTime::from_hms_milli_opt(9, 31, 10, 850).unwrap()
        );
    }

    #[test]
    fn test_multiple_lines() {
        let input = r#"
C6A8 012A E0CD 6572 @2019/05/05 09:31:10.85
---- ---- ---- ---- @2019/05/05 09:31:11.00
AABB CCDD EEFF 0011 @2019/05/05 09:31:12.50
"#;

        let iter = from_str(input);
        let groups: Vec<_> = iter.collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].a, Some(0xC6A8));
        assert_eq!(groups[1].a, None);
        assert_eq!(groups[2].a, Some(0xAABB));
    }
}

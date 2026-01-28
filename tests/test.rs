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

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5));
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(9, 31, 10, 850));

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

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5));
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(9, 31, 10, 960));
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

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2020, 12, 31));
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(23, 59, 59, 999));
    }

    #[test]
    fn test_invalid_hex() {
        let input = "ZZZZ 012A E0CD 6572 @2019/05/05 09:31:10.85\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();

        assert_eq!(item.a, None);
        assert_eq!(item.b, Some(0x012A));
        assert_eq!(item.c, Some(0xE0CD));
        assert_eq!(item.d, Some(0x6572));

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5));
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(9, 31, 10, 850));
    }

    #[test]
    fn test_malformed_date() {
        let input = "C6A8 012A E0CD 6572 @2019-13-45 22:14:32.123\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();
        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(item.b, Some(0x012A));
        assert_eq!(item.c, Some(0xE0CD));
        assert_eq!(item.d, Some(0x6572));

        assert_eq!(item.date, None);
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(22, 14, 32, 123));
    }

    #[test]
    fn test_malformed_time() {
        let input = "C6A8 012A E0CD 6572 @2019/05/05 25:14:32.123\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();
        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(item.b, Some(0x012A));
        assert_eq!(item.c, Some(0xE0CD));
        assert_eq!(item.d, Some(0x6572));

        assert_eq!(item.date, NaiveDate::from_ymd_opt(2019, 5, 5));
        assert_eq!(item.time, None);
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
        assert_eq!(item1.b, Some(0x012A));
        assert_eq!(item1.c, Some(0xE0CD));
        assert_eq!(item1.d, Some(0x6572));

        // Second valid line
        let item2 = iter.next().unwrap().unwrap();
        assert_eq!(item2.a, None);
        assert_eq!(item2.b, None);
        assert_eq!(item2.c, None);
        assert_eq!(item2.d, None);

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_trailing_garbage_ignored() {
        let input = "C6A8 012A E0CD 6572 @2019/05/05 09:31:10.85 some comment\n";
        let mut iter = from_str(input);

        let item = iter.next().unwrap().unwrap();
        assert_eq!(item.a, Some(0xC6A8));
        assert_eq!(item.time, NaiveTime::from_hms_milli_opt(9, 31, 10, 850));
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

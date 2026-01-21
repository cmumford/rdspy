use chrono::{NaiveDate, NaiveTime};
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RdsGroup {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub date: NaiveDate,
    pub time: NaiveTime,
}

pub struct RdsGroupIterator<R: BufRead> {
    lines: R,
}

impl<R: BufRead> RdsGroupIterator<R> {
    pub fn new(reader: R) -> Self {
        Self { lines: reader }
    }
}

impl<R: BufRead> Iterator for RdsGroupIterator<R> {
    type Item = io::Result<RdsGroup>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line_buf = String::new();
            match self.lines.read_line(&mut line_buf) {
                Ok(0) => return None, // EOF
                Ok(_) => {
                    let line = line_buf.trim();

                    if line.is_empty()
                        || line.starts_with('%')
                        || line.starts_with('<')
                        || !line.starts_with(|c: char| c.is_ascii_hexdigit())
                    {
                        continue;
                    }

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() < 6 {
                        eprintln!("Warning: skipping short line: {}", line);
                        continue;
                    }

                    let parse_hex = |s: &str| -> Option<u16> {
                        u16::from_str_radix(s, 16)
                            .map_err(|e| {
                                eprintln!("Hex parse error on '{}': {}", s, e);
                                e
                            })
                            .ok()
                    };

                    let date_clean = parts[4].trim_start_matches('@').trim();
                    let date = NaiveDate::parse_from_str(date_clean, "%Y/%m/%d").unwrap();
                    let time = NaiveTime::parse_from_str(parts[5], "%H:%M:%S%.f").unwrap();

                    match (
                        parse_hex(parts[0]),
                        parse_hex(parts[1]),
                        parse_hex(parts[2]),
                        parse_hex(parts[3]),
                    ) {
                        (Some(a), Some(b), Some(c), Some(d)) => {
                            return Some(Ok(RdsGroup {
                                a,
                                b,
                                c,
                                d,
                                date: date.into(),
                                time: time.into(),
                            }));
                        }
                        _ => {
                            eprintln!("Skipping invalid hex line: {}", line);
                            continue;
                        }
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

use chrono::{NaiveDate, NaiveTime};
use log::warn;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RdsGroup {
    pub a: Option<u16>,
    pub b: Option<u16>,
    pub c: Option<u16>,
    pub d: Option<u16>,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
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

                    if line.is_empty() || line.starts_with('%') || line.starts_with('<') {
                        continue;
                    }

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() < 4 {
                        warn!("Warning: skipping short line: {}", line);
                        continue;
                    }

                    let parse_hex = |s: &str| -> Option<u16> {
                        let trimmed = s.trim();
                        if trimmed.len() != 4 || trimmed == "----" {
                            return None;
                        }
                        u16::from_str_radix(trimmed, 16).ok()
                    };

                    let mut date = None;
                    let mut time = None;

                    match parts.len() {
                        5 => {
                            // Some log files have a fifth parameter of the form `@0564`.
                            // No idea what this is, so skipping for the time being.
                        }
                        6.. => {
                            let date_clean = parts[4].trim_start_matches('@').trim();
                            date = NaiveDate::parse_from_str(date_clean, "%Y/%m/%d").ok();
                            time = NaiveTime::parse_from_str(parts[5], "%H:%M:%S%.f").ok();
                            // Ignore any extra fields.
                        }
                        _ => {}
                    }

                    match (
                        parse_hex(parts[0]),
                        parse_hex(parts[1]),
                        parse_hex(parts[2]),
                        parse_hex(parts[3]),
                    ) {
                        (a, b, c, d) => {
                            return Some(Ok(RdsGroup {
                                a,
                                b,
                                c,
                                d,
                                date,
                                time,
                            }));
                        }
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

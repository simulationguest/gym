use crate::data::*;
use chrono::{Duration, NaiveDateTime};
use regex::{Match, Regex};
use std::cell::LazyCell;

type Line<'s> = Vec<&'s str>;

pub fn parse_line(line: &str) -> Data {
    let line = line.replace(['\u{00a0}', '\u{feff}', '\t'], " ");
    let line = line.trim();

    if line.is_empty() {
        return Data::Nothing;
    }

    let line: Vec<_> = line.split(';').map(trim).collect();
    if line.len() > 3 {
        return Data::Unknown;
    }

    [parse_nothing, parse_set, parse_exercise, parse_day]
        .iter()
        .find_map(|f| f(&line))
        .unwrap_or(Data::Unknown)
}

fn parse_nothing(line: &Line) -> Option<Data> {
    if line[0] == "#" {
        Some(Data::Nothing)
    } else {
        None
    }
}

fn parse_set(line: &Line) -> Option<Data> {
    let is_numeric = line.iter().find(|p| !is_numeric(&p)).is_none();
    if !is_numeric {
        return None;
    }
    Some(Data::Set(if line.len() == 2 {
        Set {
            number: parse_number(&line[0]),
            reps: parse_number(&line[1]),
            kilos: None,
        }
    } else {
        Set {
            number: parse_number(&line[0]),
            kilos: parse_number(&line[1]),
            reps: parse_number(&line[2]),
        }
    }))
}

fn parse_exercise(line: &Line) -> Option<Data> {
    if line.len() > 2 {
        return None;
    }

    const NAME_REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"^\d\..*").unwrap());

    let name = line
        .get(0)
        .map(|first| {
            if NAME_REGEX.is_match(first) {
                &first[2..]
            } else {
                first
            }
        })
        .map(str::to_string);

    Some(Data::Exercise(Exercise {
        sets: Vec::new(),
        name,
    }))
}

fn parse_day(line: &Line) -> Option<Data> {
    if line.len() != 3 {
        return None;
    }

    let date = line[1];
    let date = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M\u{a0}Uhr").or(Err(date.into()));

    Some(Data::Day(Day {
        info: Some(DayInfo {
            duration: parse_duration(&line[2]),
            name: line[0].into(),
            date,
        }),
        exercises: Vec::new(),
    }))
}

fn trim(str: &str) -> &str {
    let mut str = str.trim();
    while str.starts_with('"') || str.starts_with('+') {
        str = &str[1..];
    }
    if str.ends_with('"') {
        str = &str[..str.len() - 1];
    }
    str
}

fn parse_duration(input: &str) -> Option<Duration> {
    const DURATION_REGEXES: LazyCell<[Regex; 3]> = LazyCell::new(|| {
        [
            r"(?P<hours>\d\d?):(?P<minutes>\d?\d).*Std.",
            r"(?P<hours>\d\d?).*Std.",
            r"(?P<minutes>\d\d?).*Min.",
        ]
        .map(Regex::new)
        .map(Result::unwrap)
    });

    let captures = DURATION_REGEXES.iter().find_map(|r| r.captures(input))?;

    fn capture_value(m: Option<Match>) -> i64 {
        m.and_then(|s| s.as_str().parse::<i64>().ok()).unwrap_or(0)
    }

    let hours = capture_value(captures.name("hours"));
    let minutes = capture_value(captures.name("minutes"));

    Some(Duration::seconds(minutes * 60 + hours * 3600))
}

fn is_numeric(s: &str) -> bool {
    s.chars()
        .find(|&c| !c.is_numeric() && c != '-' && c != ',' && c != '+')
        .is_none()
}

fn parse_number(s: &str) -> Option<f32> {
    if s == "-" {
        return None;
    }
    s.replace(',', ".").parse().ok()
}

#[cfg(test)]
mod tests {
    use super::is_numeric;

    #[test]
    fn test_is_numeric() {
        assert!(!is_numeric("abc"));
        assert!(is_numeric("+3"));
        assert!(is_numeric("--"));
        assert!(!is_numeric("+++f"));
    }
}

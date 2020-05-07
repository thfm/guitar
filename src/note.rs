use crate::Interval;
use nom::{branch::alt, bytes::complete::tag, combinator::map};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Note {
    value: usize,
}

impl Note {
    pub fn new(value: usize) -> Self {
        Note { value }
    }

    pub(crate) fn disregard_octave(self) -> Self {
        Self {
            value: self.value % 12,
        }
    }
}

impl FromStr for Note {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, name) = alt((
            map(tag("C"), |_| 0),
            map(tag("Db"), |_| 1),
            map(tag("D"), |_| 2),
            map(tag("Eb"), |_| 3),
            map(tag("E"), |_| 4),
            map(tag("F"), |_| 5),
            map(tag("Gb"), |_| 6),
            map(tag("G"), |_| 7),
            map(tag("Ab"), |_| 8),
            map(tag("A"), |_| 9),
            map(tag("Bb"), |_| 10),
            map(tag("B"), |_| 11),
        ))(s)
        .map_err(|_: nom::Err<(&str, nom::error::ErrorKind)>| {
            anyhow::anyhow!("failed to parse note name")
        })?;

        let octave = if s.is_empty() { 0 } else { usize::from_str(s)? };

        Ok(Self::new(name + octave * 12))
    }
}

#[cfg(test)]
#[test]
fn parsing() {
    assert_eq!(Note::from_str("C0").unwrap(), Note::new(0));
    assert_eq!(Note::from_str("Db3").unwrap(), Note::new(37));
    assert_eq!(Note::from_str("Bb10").unwrap(), Note::new(130));
    assert_eq!(Note::from_str("Ab").unwrap(), Note::new(8));

    assert!(Note::from_str("Cb2").is_err());
    assert!(Note::from_str("Gb-2").is_err());
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.value % 12 {
            0 => "C",
            1 => "Db",
            2 => "D",
            3 => "Eb",
            4 => "E",
            5 => "F",
            6 => "Gb",
            7 => "G",
            8 => "Ab",
            9 => "A",
            10 => "Bb",
            11 => "B",
            _ => unreachable!(),
        };

        if f.alternate() {
            write!(f, "{}", name)
        } else {
            let octave = self.value / 12;
            write!(f, "{}{}", name, octave)
        }
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn normal() {
        assert_eq!(Note::new(0).to_string(), "C0");
        assert_eq!(Note::new(37).to_string(), "Db3");
        assert_eq!(Note::new(76).to_string(), "E6");
    }

    #[test]
    fn alternate() {
        assert_eq!(format!("{:#}", Note::new(0)), "C");
        assert_eq!(format!("{:#}", Note::new(37)), "Db");
        assert_eq!(format!("{:#}", Note::new(76)), "E");
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, interval: Interval) -> Self::Output {
        Self {
            value: self.value + interval.semitones,
        }
    }
}

impl Sub<Interval> for Note {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self::Output {
        Self {
            value: self.value - interval.semitones,
        }
    }
}

#[cfg(test)]
#[test]
fn transposition() {
    assert_eq!(Note::new(10) + Interval::new(5), Note::new(15));
    assert_eq!(Note::new(42) + Interval::new(12), Note::new(54));
    assert_eq!(Note::new(10) - Interval::new(5), Note::new(5));
    assert_eq!(Note::new(42) - Interval::new(12), Note::new(30));
}

impl Sub for Note {
    type Output = Interval;

    fn sub(self, other: Self) -> Self::Output {
        match self.value.cmp(&other.value) {
            Ordering::Greater => Interval::new(self.value - other.value),
            Ordering::Less => Interval::new(other.value - self.value),
            Ordering::Equal => Interval::new(0),
        }
    }
}

#[cfg(test)]
#[test]
fn interval_calculation() {
    assert_eq!(Note::new(10) - Note::new(5), Interval::new(5));
    assert_eq!(Note::new(21) - Note::new(27), Interval::new(6));
    assert_eq!(Note::new(37) - Note::new(37), Interval::new(0));
}

impl IntoIterator for Note {
    type Item = Self;
    type IntoIter = NoteIter;

    fn into_iter(self) -> Self::IntoIter {
        NoteIter {
            note: self,
            first: true,
        }
    }
}

pub struct NoteIter {
    note: Note,
    first: bool,
}

impl Iterator for NoteIter {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        // Returns the original note if this is the first iteration
        if self.first {
            self.first = false;
            Some(self.note)
        } else {
            self.note.value += 1;
            Some(self.note)
        }
    }
}

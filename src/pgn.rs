use std::{collections::VecDeque, fs::File, io::BufRead, io::BufReader, path::Path};

use chrono::{NaiveDate, NaiveTime};
use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, Error)]
pub enum TagError {
    #[error("tag must start with '['")]
    NoOpeningSquareBracket,
    #[error("tag must end with ']'")]
    NoClosingSquareBracket,
    #[error("unknown tag {0}")]
    UnknownTag(String, String),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Tag {
    Event(String),
    Site(String),
    Date(NaiveDate),
    Round(String),
    White(String),
    Black(String),
    Result(String),
    UTCDate(NaiveDate),
    Eco(String),
    WhiteElo(u16),
    BlackElo(u16),
    Annotator(String),
    WhiteRatingDiff(i16),
    BlackRatingDiff(i16),
    Variant(String),
    TimeControl(String),
    Opening(String),
    Termination(String),
    EndTime(NaiveTime),
    UTCTime(NaiveTime),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Black(String),
    White(String),
}

#[derive(Debug)]
pub struct Pgn {
    tags: Vec<Result<Tag, TagError>>,
    moves: VecDeque<Move>,
}

impl Pgn {
    pub fn new<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
        Ok(Self {
            tags: parse_tags(&lines),
            moves: parse_moves(&lines),
        })
    }

    pub fn tags(&self) -> &[Result<Tag, TagError>] {
        self.tags.as_ref()
    }
}

impl TryFrom<&str> for Tag {
    type Error = TagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('[') {
            return Err(TagError::NoOpeningSquareBracket);
        };

        if !value.ends_with(']') {
            return Err(TagError::NoClosingSquareBracket);
        };

        let value = value.replace('[', "").replace(']', "");

        let data = value.split_once(' ').unwrap();

        match (data.0, data.1.replace('\"', "")) {
            ("Event", event) => Ok(Tag::Event(event)),
            ("Site", site) => Ok(Tag::Site(site)),
            ("Date", date) => Ok(Tag::Date(
                NaiveDate::parse_from_str(&date, "%Y.%m.%d").unwrap(),
            )),
            ("UTCDate", utc_date) => Ok(Tag::UTCDate(
                NaiveDate::parse_from_str(&utc_date, "%Y.%m.%d").unwrap(),
            )),
            ("Round", round) => Ok(Tag::Round(round)),
            ("White", white) => Ok(Tag::White(white)),
            ("Black", black) => Ok(Tag::Black(black)),
            ("Result", result) => Ok(Tag::Result(result)),
            ("WhiteElo", white_elo) => Ok(Tag::WhiteElo(white_elo.parse().unwrap())),
            ("BlackElo", black_elo) => Ok(Tag::BlackElo(black_elo.parse().unwrap())),
            ("ECO", eco) => Ok(Tag::Eco(eco)),
            ("Annotator", annotator) => Ok(Tag::Annotator(annotator)),
            ("WhiteRatingDiff", white_diff_rating) => {
                Ok(Tag::WhiteRatingDiff(white_diff_rating.parse().unwrap()))
            }
            ("BlackRatingDiff", black_diff_rating) => {
                Ok(Tag::BlackRatingDiff(black_diff_rating.parse().unwrap()))
            }
            ("Variant", variant) => Ok(Tag::Variant(variant)),
            ("TimeControl", time_control) => Ok(Tag::TimeControl(time_control)),
            ("Opening", opening) => Ok(Tag::Opening(opening)),
            ("Termination", termination) => Ok(Tag::Termination(termination)),
            ("EndTime", end_time) => Ok(Tag::EndTime(
                NaiveTime::parse_from_str(&end_time, "%H:%M:%S %Z").unwrap(),
            )),
            ("UTCTime", utc_time) => Ok(Tag::UTCTime(
                NaiveTime::parse_from_str(&utc_time, "%H:%M:%S").unwrap(),
            )),
            (_, unknown_data) => Err(TagError::UnknownTag(data.0.to_string(), unknown_data)),
        }
    }
}

impl From<(&str, &str)> for Move {
    fn from(value: (&str, &str)) -> Self {
        let value = (value.0.parse::<u16>().unwrap(), value.1);
        match (value.0 % 2 == 0, value.1) {
            (true, chess_move) => Move::Black(chess_move.to_string()),
            (false, chess_move) => Move::White(chess_move.to_string()),
        }
    }
}

fn parse_tags(lines: &[String]) -> Vec<Result<Tag, TagError>> {
    lines
        .iter()
        .filter(|line| line.starts_with('['))
        .map(|line| Tag::try_from(line.as_ref()))
        .collect()
}

fn parse_moves(line: &[String]) -> VecDeque<Move> {
    let line = line
        .iter()
        .filter(|line| !line.starts_with('[') && !line.is_empty())
        .map(|l| l.as_ref())
        .collect::<Vec<&str>>()
        .join(" ");

    let moves = line.split(pat)

    println!("{}", line);

    VecDeque::new()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_properly_assing_move_to_move() {
        let first_chess_move = ("1", "e4");
        let second_chess_move = ("2", "c6");
        let third_chess_move = ("3", "Nf3");
        let fourth_chess_move = ("4", "exd5");

        assert_eq!(Move::from(first_chess_move), Move::White("e4".to_string()));
        assert_eq!(Move::from(second_chess_move), Move::Black("c6".to_string()));
        assert_eq!(Move::from(third_chess_move), Move::White("Nf3".to_string()));
        assert_eq!(Move::from(fourth_chess_move), Move::Black("exd5".to_string()));
    }
}

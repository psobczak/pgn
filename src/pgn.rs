use std::{fs::File, io::BufRead, io::BufReader, path::Path};

use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, Error)]
pub enum TagError {
    #[error("tag must start with '['")]
    NoOpeningSquareBracket,
    #[error("tag must end with ']'")]
    NoClosingSquareBracket,
    #[error("tag {0} is not supported")]
    UnknownTag(String),
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
}

#[derive(Debug)]
pub struct Pgn {
    tags: Vec<Tag>,
}

impl Pgn {
    pub fn new<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(Self {
            tags: BufReader::new(file)
                .lines()
                .flatten()
                .flat_map(|s| Tag::try_from(s.as_ref()))
                .collect(),
        })
    }

    pub fn tags(&self) -> &[Tag] {
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
        let data = (data.0, data.1.replace('\"', ""));
        match data {
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
            _ => Err(TagError::UnknownTag(data.0.to_string())),
        }
    }
}

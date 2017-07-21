//! DerbyJSON parser, based on serde.


#![feature(custom_attribute)]
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::io::Read;

mod jamdata;
pub use jamdata::*;
mod teamdata;
pub use teamdata::*;

/// Version of DerbyJSON supported
pub const VERSION: &str = "0.2";


/// The root DerbyJSON object. This can store information about a game, or about
/// a league or a team. Which one it is is determined by the "objecttype" field
/// and determines which fields are valid.
#[derive(Serialize, Deserialize)]
pub struct DerbyJSON {
    pub version: Option<String>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "type")]
    pub objecttype: ObjectType,
    pub teams: HashMap<String, Team>,
    pub periods: Vec<Period>,
    pub ruleset: Option<Ruleset>,
    pub venue: Option<Venue>,
    #[serde(default)]
    pub uuid: Vec<String>,
    pub notes: Vec<Note>,
    pub date: String,
    pub time: String,
    pub end_time: String,
    pub leagues: Option<Vec<League>>,
    pub timers: Timers,
    pub tournament: Option<String>,
    #[serde(rename = "host-league")]
    pub host_league: Option<String>,
    pub expulsions: Vec<Expulsion>,
    pub suspensions: Vec<String>,
    pub signatures: Vec<serde_json::Value>, // XXX: spec says signature objects
    pub sanctioned: bool,
    pub association: Option<Association>,
}

impl DerbyJSON {
    /// Create an empty DerbyJSON structure corresponding to a game.
    /// It fills in default/empty values for almost everything and
    /// creates two periods.
    pub fn new_game(teams: HashMap<String, Team>) -> DerbyJSON {
        let timers = Timers {
            countdown: None,
            halftime: None,
            jam: None,
            period: Timer {
                duration: 30 * 60,
                counts_down: true,
                running: false,
            }
        };

        DerbyJSON {
            version: Some(VERSION.to_string()),
            objecttype: ObjectType::Game,
            metadata: serde_json::Map::new(),
            ruleset: None,
            venue: None,
            uuid: Vec::new(),
            notes: Vec::new(),
            leagues: None,
            tournament: None,
            host_league: None,
            expulsions: Vec::new(),
            suspensions: Vec::new(),
            signatures: Vec::new(),
            association: None,

            date: String::new(),
            time: String::new(),
            end_time: String::new(),

            sanctioned: false,
            teams: teams,

            periods: vec![Period::default(), Period::default()],
            timers: timers,
        }
    }
}

/// A subset of the general DerbyJSON object, just storing information on
/// team/league rosters.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rosters {
    pub version: Option<String>,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "type")]
    pub objecttype: ObjectType,
    pub teams: HashMap<String, Team>,
    #[serde(default)]
    pub uuid: Vec<String>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub leagues: Vec<League>,
}

impl Rosters {
    pub fn new(teams: HashMap<String, Team>) -> Rosters {
        Rosters {
            version: Some(VERSION.to_string()),
            metadata: None,
            objecttype: ObjectType::Rosters,
            teams: teams,
            uuid: vec!(),
            notes: vec!(),
            leagues: vec!(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ObjectType { Game, Rosters, Stats, League }

#[derive(Serialize, Deserialize)]
pub struct Expulsion {
    pub skater: String,
    pub suspension: bool,
    pub notes: Vec<Note>,
}

/// Information about the ruleset used for a game.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ruleset {
    pub version: String,
    pub period_count: u8,
    pub period: String,
    pub jam: String,
    pub lineup: String,
    pub timeout: String,
    pub timeout_count: u8,
    pub official_review_count: u8,
    pub official_review_retained: bool,
    pub official_review_maximum: u8,
    pub penalty: String,
    pub minors: bool,
    pub minors_per_major: u8,
    pub foulout: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Timers {
    pub countdown: Option<Timer>,
    pub period: Timer,
    pub halftime: Option<Timer>,
    pub jam: Option<Timer>,
}

#[derive(Serialize, Deserialize)]
pub struct Timer {
    pub duration: u16,
    pub counts_down: bool,
    pub running: bool,
}

/// A note about something that happened. These notes may be attached to
/// quite a few objects found elsewhere in the spec.
#[derive(Serialize,Deserialize)]
pub struct Note {
    pub note: String,
    pub author: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    UnexpectedType(String),
    UnexpectedVersion(String),
}

type SDE = serde_json::Error;
impl From<SDE> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e)
    }
}

/** Load a roster from the given input stream. This specifically checks that 
    the loaded DerbyJSON is a valid roster object. */
pub fn load_roster<R>(reader: R) -> Result<Rosters, Error>
    where R: Read
{
    let obj: Rosters = serde_json::from_reader(reader)?;
    if obj.objecttype != ObjectType::Rosters {
        let t = format!("{:?}", obj.objecttype);
        return Err(Error::UnexpectedType(t));
    }
    if let Some(ref version) = obj.version {
        if version != VERSION {
            return Err(Error::UnexpectedVersion(version.clone()));
        }
    }
    return Ok(obj);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    #[test]
    fn it_works() {
        let text = include_bytes!("rosters.json");
        let reader = Cursor::new(&text[..]);
        let res = super::load_roster(reader);
        println!("{:?}",res.as_ref().err());
        assert!(res.is_ok());
        let djson = res.unwrap();
        assert!(djson.teams.len() == 2);
    }
}

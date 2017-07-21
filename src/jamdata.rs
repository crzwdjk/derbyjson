use serde_json;
use super::Note;

/// This enum represents an event that happens during a game of derby, and is
/// the main container for storing game data. Points, penalties, lineups,
/// and just about anything else that is associated with a jam goes into
/// a Jam Event of some sort. Each event is tagged with an event type,
/// which determined what information it contains.
#[derive(Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum JamEvent {
    /// Information on one skater that has skated in a given jam.
    /// A typical jam will have 10 of these objects in the jam's events.
    #[serde(rename = "line up")]
    Lineup {
        skater: String,
        start_in_box: bool,
        position: Position,
    },
    #[serde(rename = "pack lap")]
    PackLap {
        timestamp: Option<Timestamp>,
        count: Option<u8>,
    },
    #[serde(rename = "penalty")]
    Penalty {
        timestamp: Option<Timestamp>,
        skater: String,
        penalty: String,
        severity: Option<PenaltySeverity>,
        rescinded: Option<bool>,
        involved: Option<Vec<Involved>>,
        cue: Option<String>,
    },
    #[serde(rename = "pass")]
    Pass {
        timestamp: Option<Timestamp>,
        completed: Option<bool>,
        number: u8,
        points: Option<u8>,
        skater: Option<String>,
        ghost_points: Option<Vec<GhostPoint>>
    },
    #[serde(rename = "star pass")]
    StarPass {
        timestamp: Option<Timestamp>,
        skater: Option<String>,
        team: Option<String>,
        completed: Option<bool>,
        failure: Option<String>,
    },
    #[serde(rename = "lead")]
    Lead {
        timestamp: Option<Timestamp>,
        skater: String,
    },
    #[serde(rename = "lost lead")]
    LostLead {
        timestamp: Option<Timestamp>,
        skater: String,
    },
    #[serde(rename = "call")]
    Call {
        timestamp: Option<Timestamp>,
        skater: Option<String>,
        team: Option<String>,
        official: Option<String>,
    },
    #[serde(rename = "enter box")]
    EnterBox {
        timestamp: Option<Timestamp>,
        skater: String,
        duration: Option<serde_json::Number>,
        substitute: Option<Substitute>,
        notes: Option<Vec<Note>>
    },
    #[serde(rename = "exit box", rename_all = "kebab-case")]
    ExitBox {
        timestamp: Option<Timestamp>,
        skater: String,
        duration: Option<serde_json::Number>,
        premature: Option<PrematureExitReason>,
        no_skater: Option<bool>
    },
    /// This object and its contents are not actually specified in the
    /// DerbyJSON spec
    #[serde(rename = "box time")]
    BoxTime { },
    
    #[serde(rename = "injury")]
    Injury {
        timestamp: Option<Timestamp>,
        skater: String,
    },
    #[serde(rename = "note")]
    Note {
        note: String,
        author: Option<String>,
        date: Option<String>,
        notes: Note,
    },

    #[serde(rename = "leave track", rename_all = "kebab-case")]
    LeaveTrack {
        timestamp: Option<Timestamp>,
        skater: String,
        reason: Option<LeaveTrackReason>,
        opposing_pass: u8,
    },

    #[serde(rename = "return track", rename_all = "kebab-case")]
    ReturnTrack {
        timestamp: Option<Timestamp>,
        skater: String,
        opposing_pass: u8,
    },
    // Action, Error,
}

/// A skater's position in a jam.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Position { Jammer, Pivot, Blocker }

/// Penalty severity.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PenaltySeverity { No, Minor, Major, Expulsion }

/// Reason why a skater left the penalty box early: due to officiating error,
/// skater leaving the box early, a rescinded penalty, or a skater who
/// mistakenly reported to the box.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrematureExitReason { Official, Skater, Rescinded, Mistake }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeaveTrackReason { Penalty, Injury, Malfuction, Other }
    
/// Represents a "ghost point" scored by means other than passing an
/// opponent's hips
#[derive(Serialize, Deserialize)]
pub struct GhostPoint {
    pub skater: Option<String>,
    pub ghost_point: GhostPointType,
}

/// Type of ghost point. Lap of jammer, Jammer in box, Blocker in box,
/// Pivot in box, Not on the track, Out of play, Ghost point of unknown causes
#[derive(Serialize, Deserialize)]
pub enum GhostPointType { L, J, B, P, N, O, G }

#[derive(Serialize, Deserialize)]
pub struct Involved {
    pub skater: String,
    #[serde(default)]
    pub notes: Vec<Note>,
}

#[derive(Serialize, Deserialize)]
pub struct Substitute {
    pub skater: String,
    pub reason: String,
}

/// Periods are broken into Jams, which are the basic unit of play for roller
/// derby.
#[derive(Serialize, Deserialize)]
pub struct Jam {
    pub number: u16,
    pub timestamp: Option<Timestamp>,
    pub duration: Option<u16>,
    pub events: Vec<JamEvent>,
    pub notes: Vec<Note>,
}

/// A stoppage of the game clock, whether for a team timeout, official review,
/// or official timeout.
#[derive(Serialize, Deserialize)]
pub struct Timeout {
    pub timeout: TeamType,
    #[serde(default)]
    pub notes: Vec<Note>,
    pub injury: Option<String>, // Skater
    pub duration: u32, // in seconds, including lineup time
    pub timestamp: Option<Timestamp>,
    pub review: Option<String>,
    pub resolution: Option<String>,
    pub retained: Option<bool>,
}

/// A thing that happens during a period, either a jam or a timeout, or possibly
/// just a note.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClockEvent {
    Note(Note),
    Jam(Jam),
    Timeout(Timeout),
}

#[derive(Serialize, Deserialize)]
pub enum TeamType { Home, Away, Officials }

#[derive(Serialize, Deserialize, Default)]
pub struct Period {
    pub timestamp: Option<Timestamp>,
    pub end: Option<Timestamp>,
    pub jams: Vec<ClockEvent>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Timestamp {
    Wall(String),
    Epoch(serde_json::Number),
    Period(String),
    Seconds(serde_json::Number),
    Jam(serde_json::Number),
    // TODO: Range, approximate
}

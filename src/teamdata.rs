use super::Note;

/// Information on a team (collection of skaters or officials)
#[derive(Serialize, Deserialize)]
pub struct Team {
    /// The name of the team. Required to be unique within the league.
    /// This can be the empty string for a team that is the only member
    /// of its league.
    pub name: String,
    /// The name of the team's league.
    pub league: Option<String>,
    pub abbreviation: Option<String>,
    /// An array of skaters (or refs) on the team.
    pub persons: Vec<Person>,
    /// Team's level
    pub level: Option<TeamLevel>,
    /// Date as of which this roster is current.
    pub date: Option<String>,
    /// Team color. The DerbyJSON spec
    pub color: Option<String>, // XXX: spec refers to a "coloring object"
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize)]
pub enum TeamLevel {
    #[serde(rename = "All Star")]
    AllStar,
    B, C, Rec, Officials, Home, Adhoc
}

/// Information on a league (collection of teams)
#[derive(Serialize, Deserialize)]
pub struct League {
    pub name: String,
    pub abbreviation: Option<String>,
    pub uuid: Option<Vec<String>>,
    pub venue: Option<Venue>,
    pub teams: Vec<Team>,
    pub logo: Option<Logo>,
}

/// Information about a person, whether a skater or official
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    /// Skater (or official) number. Required for skaters.
    pub number: Option<String>,
    pub league: Option<String>,
    pub certifications: Option<Vec<Certification>>,
    pub legal: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    pub skated: Option<bool>,
    pub uuid: Option<Vec<String>>,
    pub insurance: Option<Vec<String>>,
}

/// Information about a game venue.
#[derive(Serialize,Deserialize)]
pub struct Venue {
    pub name: String,
    pub city: String,
    pub state: String,
    pub url: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub fax: Option<String>,
    pub otheraddr: Option<String>,
    pub phone: Option<String>,
    pub pob: Option<String>,
    pub postcode: Option<String>,
    pub street: Option<String>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub uuid: Vec<String>,
    #[serde(default)]
    pub logo: Vec<Logo>,
}

#[derive(Serialize,Deserialize)]
pub struct Certification {
    pub association: Association,
    pub certification: String,
    pub level: Option<u8>,
    pub endorsement: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Association { WFTDA, MRDA, JRDA, Other }

/// Represents a team or league logo. Each field may contain a URL to
/// the appropriate size/style of team logo.
#[derive(Serialize,Deserialize)]
pub struct Logo {
    /// If there is only one logo variant, this field should contain its URL.
    pub url: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub small_dark: Option<String>,
    pub medium_dark: Option<String>,
    pub large_dark: Option<String>,
    pub small_light: Option<String>,
    pub medium_light: Option<String>,
    pub large_light: Option<String>,
    pub small_greyscale: Option<String>,
    pub medium_greyscale: Option<String>,
    pub large_greyscale: Option<String>,
}

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Radiant,
    Dire,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let team_str = match self {
            Team::Radiant => "Radiant",
            Team::Dire => "Dire",
        };
        write!(f, "{team_str}")
    }
}

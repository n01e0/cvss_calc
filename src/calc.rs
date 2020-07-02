extern crate display_initial;

use display_initial::DisplayInitial;
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::io::{self, Write, prelude::*};
use std::str;

pub struct ParseError {
    pub message: String,
}

trait ScoreWait {
    fn wait(&self) -> f32;
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum AttackVector {
    Network,
    Adjacent,
    Local,
    Physical,
}

impl ScoreWait for AttackVector {
    fn wait(&self) -> f32 {
        match self {
            Self::Network => 0.85,
            Self::Adjacent => 0.62,
            Self::Local => 0.55,
            Self::Physical => 0.20,
        }
    }
}

impl str::FromStr for AttackVector {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "Network" | "n" | "network" => Ok(Self::Network),
            "A" | "Adjacent" | "a" | "adjacent" => Ok(Self::Adjacent),
            "L" | "Local" | "l" | "local" => Ok(Self::Local),
            "P" | "Physical" | "p" | "physical" => Ok(Self::Physical),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum AttackComplexity {
    Low,
    High,
}

impl ScoreWait for AttackComplexity {
    fn wait(&self) -> f32 {
        match self {
            Self::Low => 0.77,
            Self::High => 0.44,
        }
    }
}

impl str::FromStr for AttackComplexity {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" | "Low" | "l" | "low" => Ok(Self::Low),
            "H" | "High" | "h" | "high" => Ok(Self::High),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum PrivilegesRequired {
    None,
    Low,
    High,
}

impl str::FromStr for PrivilegesRequired {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "None" | "n" | "none" => Ok(Self::None),
            "L" | "Low" | "l" | "low" => Ok(Self::Low),
            "H" | "High" | "h" | "high" => Ok(Self::High),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum UserInteraction {
    None,
    Required,
}

impl ScoreWait for UserInteraction {
    fn wait(&self) -> f32 {
        match self {
            Self::None => 0.85,
            Self::Required => 0.62,
        }
    }
}

impl str::FromStr for UserInteraction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "None" | "n" | "none" => Ok(Self::None),
            "R" | "Required" | "r" | "required" => Ok(Self::Required),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum Scope {
    Changed,
    Unchanged,
}

impl ScoreWait for Scope {
    fn wait(&self) -> f32 {
        match self {
            Self::Changed => 7.52,
            Self::Unchanged => 6.42,
        }
    }
}

impl str::FromStr for Scope {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" | "Changed" | "c" | "changed" => Ok(Self::Changed),
            "U" | "Unchanged" | "u" | "unchanged" => Ok(Self::Unchanged),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum Confidentiality {
    High,
    Low,
    None,
}

impl ScoreWait for Confidentiality {
    fn wait(&self) -> f32 {
        match self {
            Self::High => 0.56,
            Self::Low => 0.22,
            Self::None => 0.00,
        }
    }
}

impl str::FromStr for Confidentiality {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "None" | "n" | "none" => Ok(Self::None),
            "L" | "Low" | "l" | "low" => Ok(Self::Low),
            "H" | "High" | "h" | "high" => Ok(Self::High),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum Integrity {
    High,
    Low,
    None,
}

impl ScoreWait for Integrity {
    fn wait(&self) -> f32 {
        match self {
            Self::High => 0.56,
            Self::Low => 0.22,
            Self::None => 0.00,
        }
    }
}

impl str::FromStr for Integrity {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "None" | "n" | "none" => Ok(Self::None),
            "L" | "Low" | "l" | "low" => Ok(Self::Low),
            "H" | "High" | "h" | "high" => Ok(Self::High),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, DisplayInitial, Eq, PartialEq)]
pub enum Availability {
    High,
    Low,
    None,
}

impl ScoreWait for Availability {
    fn wait(&self) -> f32 {
        match self {
            Self::High => 0.56,
            Self::Low => 0.22,
            Self::None => 0.00,
        }
    }
}

impl str::FromStr for Availability {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" | "None" | "n" | "none" => Ok(Self::None),
            "L" | "Low" | "l" | "low" => Ok(Self::Low),
            "H" | "High" | "h" | "high" => Ok(Self::High),
            e => Err(ParseError {
                message: format!("can't parse {}", e),
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Grade {
    Critical,
    High,
    Medium,
    Low,
    None,
}

impl From<f32> for Grade {
    fn from(score: f32) -> Self {
        if score == 0.0 {
            Self::None
        } else if 0.1 <= score && score <= 3.9 {
            Self::Low
        } else if 4.0 <= score && score <= 6.9 {
            Self::Medium
        } else if 7.0 <= score && score <= 8.9 {
            Self::High
        } else {
            Self::Critical
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Critical => write!(f, "Critical"),
            Self::High => write!(f, "High"),
            Self::Medium => write!(f, "Medium"),
            Self::Low => write!(f, "Low"),
            Self::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub struct BaseScore {
    pub score: f32,
    pub grade: Grade,
}

#[derive(Debug)]
pub struct Calculator {
    pub base_score: BaseScore,
    pub av: AttackVector,
    pub ac: AttackComplexity,
    pub pr: PrivilegesRequired,
    pub ui: UserInteraction,
    pub s: Scope,
    pub c: Confidentiality,
    pub i: Integrity,
    pub a: Availability,
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CVSS:3.0/AV:{}/AC:{}/PR:{}/UI:{}/S:{}/C:{}/I:{}/A{}",
            self.av, self.ac, self.pr, self.ui, self.s, self.c, self.i, self.a
        )
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            base_score: BaseScore {
                score: 0.0,
                grade: Grade::None,
            },
            av: AttackVector::Local,
            ac: AttackComplexity::High,
            pr: PrivilegesRequired::High,
            ui: UserInteraction::Required,
            s: Scope::Unchanged,
            c: Confidentiality::None,
            i: Integrity::None,
            a: Availability::None,
        }
    }

    pub fn read_attack_vector<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "AV(N/A/L/P): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<AttackVector>() {
                Ok(t) => {
                    self.av = t;
                    return self;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
    }

    pub fn read_attack_complexity<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "AC(L/H): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<AttackComplexity>() {
                Ok(t) => {
                    self.ac = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_privileges_required<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "PR(N/L/H): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<PrivilegesRequired>() {
                Ok(t) => {
                    self.pr = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_user_interaction<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "UI(N/R): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<UserInteraction>() {
                Ok(t) => {
                    self.ui = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_scope<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "S(C/U): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<Scope>() {
                Ok(t) => {
                    self.s = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_confidentiality<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "C(H/L/N): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<Confidentiality>() {
                Ok(t) => {
                    self.c = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_integrity<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "I(H/L/N): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<Integrity>() {
                Ok(t) => {
                    self.i = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn read_availability<T: BufRead>(&mut self, mut reader: T) -> &mut Self {
        loop {
            print!("{}", "A(H/L/N): ");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            match buf.trim().parse::<Availability>() {
                Ok(t) => {
                    self.a = t;
                    break;
                }
                Err(e) => {
                    eprintln!("{}", e.message);
                }
            }
        }
        self
    }

    pub fn calc(&mut self) -> &mut Self {
        let mut impact =
            1.0 - (1.0 - self.c.wait()) * (1.0 - self.i.wait()) * (1.0 - self.a.wait());

        impact = if self.s == Scope::Unchanged {
            self.s.wait() * impact
        } else {
            self.s.wait() * (impact - 0.029) - 3.25 * (impact - 0.02).powi(15)
        };

        let pr_wait = if self.s == Scope::Unchanged {
            match self.pr {
                PrivilegesRequired::None => 0.85,
                PrivilegesRequired::Low => 0.62,
                PrivilegesRequired::High => 0.27,
            }
        } else {
            match self.pr {
                PrivilegesRequired::None => 0.85,
                PrivilegesRequired::Low => 0.68,
                PrivilegesRequired::High => 0.5,
            }
        };

        let ease = 8.22 * self.av.wait() * self.ac.wait() * pr_wait * self.ui.wait();

        let s = if (impact * 10.0).round() / 10.0 <= 0.0 {
            0.0
        } else if self.s == Scope::Unchanged {
            ((impact + ease).min(10.0) * 10.0).round() / 10.0
        } else {
            ((1.08 * (impact + ease)).min(10.0) * 10.0).round() / 10.0
        };

        let g = Grade::from(s);

        self.base_score = BaseScore { score: s, grade: g };
        self
    }
}

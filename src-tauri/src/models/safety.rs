use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SafetyLevel {
    Green,
    Yellow,
    Red,
}

impl SafetyLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SafetyLevel::Green => "green",
            SafetyLevel::Yellow => "yellow",
            SafetyLevel::Red => "red",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "green" => SafetyLevel::Green,
            "yellow" => SafetyLevel::Yellow,
            _ => SafetyLevel::Red,
        }
    }
}

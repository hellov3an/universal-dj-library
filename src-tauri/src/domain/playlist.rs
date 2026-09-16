use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A playlist in the universal model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub is_folder: bool,
    pub is_smart: bool,
    pub smart_rules: Option<SmartRules>,
    pub track_ids: Vec<Uuid>,
    pub sort_order: u32,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

/// Smart playlist rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRules {
    pub match_type: MatchType,
    pub conditions: Vec<SmartCondition>,
}

/// How to combine conditions: match all (AND) or any (OR).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchType {
    All,
    Any,
}

/// A single condition in a smart playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCondition {
    pub field: SmartField,
    pub operator: SmartOperator,
    pub value: String,
}

/// Fields available for smart playlist conditions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartField {
    Title,
    Artist,
    Album,
    Genre,
    Bpm,
    Key,
    Rating,
    Energy,
    Year,
    Comment,
    Tag,
    DateAdded,
    Duration,
    Label,
}

/// Comparison operators for smart playlist conditions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Between,
    IsEmpty,
    IsNotEmpty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_rules_serialization() {
        let rules = SmartRules {
            match_type: MatchType::All,
            conditions: vec![
                SmartCondition {
                    field: SmartField::Genre,
                    operator: SmartOperator::Equals,
                    value: "House".to_string(),
                },
                SmartCondition {
                    field: SmartField::Bpm,
                    operator: SmartOperator::Between,
                    value: "124-128".to_string(),
                },
                SmartCondition {
                    field: SmartField::Rating,
                    operator: SmartOperator::GreaterOrEqual,
                    value: "4".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&rules).unwrap();
        let deserialized: SmartRules = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.match_type, MatchType::All);
        assert_eq!(deserialized.conditions.len(), 3);
    }
}

use crate::{json_schema_is_string, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TraderIdOrEmail {
    Id(UserId),
    Email(String),
}

impl std::fmt::Display for TraderIdOrEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Email(email) => write!(f, "{}", email),
        }
    }
}

impl std::str::FromStr for TraderIdOrEmail {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('@') {
            Ok(Self::Email(s.to_string()))
        } else {
            Ok(Self::Id(s.parse()?))
        }
    }
}

json_schema_is_string!(TraderIdOrEmail);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trader_specifier_json() {
        let id: UserId = "aa0fc734-0da2-4168-8712-4c0b67f01c59".parse().unwrap();
        let email = "test@example.com".to_string();

        // Test UserId serialization
        let id_spec = TraderIdOrEmail::Id(id);
        insta::assert_json_snapshot!(id_spec, @r#""aa0fc734-0da2-4168-8712-4c0b67f01c59""#);

        // Test UserId deserialization
        let id_json = r#""aa0fc734-0da2-4168-8712-4c0b67f01c59""#;
        let id_deserialized: TraderIdOrEmail = serde_json::from_str(id_json).unwrap();
        assert_eq!(id_spec, id_deserialized);

        // Test email serialization
        let email_spec = TraderIdOrEmail::Email(email.clone());
        insta::assert_json_snapshot!(email_spec, @r#""test@example.com""#);

        // Test email deserialization
        let email_json = r#""test@example.com""#;
        let email_as_user_id: UserId = "test@example.com".parse().unwrap();
        let email_deserialized: TraderIdOrEmail =
            serde_json::from_str(email_json).unwrap();
        assert_eq!(TraderIdOrEmail::Id(email_as_user_id), email_deserialized);
    }
}

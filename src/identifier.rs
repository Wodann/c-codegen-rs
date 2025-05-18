use core::fmt;
use std::borrow::Cow;

/// Errors that can occur when creating an identifier
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("identifier cannot be empty")]
    Empty,
    #[error("identifier cannot start with a digit")]
    StartsWithDigit,
    #[error("identifier can only contain letters, digits, and underscore")]
    InvalidCharacters,
}

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn new<S: Into<String>>(value: S) -> Result<Self, Error> {
        let string = value.into();

        if string.is_empty() {
            return Err(Error::Empty);
        }

        let first_char = string.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            return Err(Error::StartsWithDigit);
        }

        if !string
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(Error::InvalidCharacters);
        }

        Ok(Self(string))
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Identifier> for Cow<'_, str> {
    fn from(identifier: Identifier) -> Self {
        Cow::Owned(identifier.0)
    }
}

impl<'a> From<&'a Identifier> for Cow<'a, str> {
    fn from(identifier: &'a Identifier) -> Self {
        Cow::Borrowed(&identifier.0)
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifier() {
        let id = Identifier::new("valid_identifier123").unwrap();
        assert_eq!(id.to_string(), "valid_identifier123");
    }

    #[test]
    fn test_empty_identifier() {
        let result = Identifier::new("");
        assert!(matches!(result, Err(Error::Empty)));
    }

    #[test]
    fn test_starts_with_digit() {
        let result = Identifier::new("123invalid");
        assert!(matches!(result, Err(Error::StartsWithDigit)));
    }

    #[test]
    fn test_invalid_characters() {
        let result = Identifier::new("invalid@identifier");
        assert!(matches!(result, Err(Error::InvalidCharacters)));
    }

    #[test]
    fn test_case_sensitivity() {
        let lower = Identifier::new("identifier").unwrap();
        let upper = Identifier::new("IDENTIFIER").unwrap();
        assert_ne!(lower.to_string(), upper.to_string());
    }

    #[test]
    fn test_underscore() {
        let id = Identifier::new("_valid_identifier").unwrap();
        assert_eq!(id.to_string(), "_valid_identifier");
    }
}

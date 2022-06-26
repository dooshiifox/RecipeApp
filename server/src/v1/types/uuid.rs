use std::fmt;

/// A type representing a unique identifier, specifically UUIDv4.
///
/// Internally is a [`u128`] as it allows for more efficient storage,
/// however has several functions that make it easier to use as a `String`.
///
/// [`u128`]: https://doc.rust-lang.org/std/primitive.u128.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Uuid(u128);

impl Uuid {
    /// Generates a new random `Uuid`.
    pub fn random() -> Self {
        Uuid(uuid::Uuid::new_v4().as_u128())
    }

    /// Constructs a new `Uuid` from a [`u128`].
    ///
    /// [`u128`]: https://doc.rust-lang.org/std/primitive.u128.html
    pub fn from_u128(u: u128) -> Self {
        Uuid(u)
    }

    /// Parses an input from UUIDv4 to a `Uuid`.
    ///
    /// `s` is expected to be a valid UUIDv4 string *
    /// (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`).
    /// If it is not, an error is returned.
    ///
    /// \* In actuality, anything parsable by [`uuid::Uuid::try_parse`] is accepted.
    ///
    /// [`uuid::Uuid::try_parse`]: https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.try_parse
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        uuid::Uuid::try_parse(s)
            .map(|u| Uuid(u.as_u128()))
            .map_err(|_| "Invalid UUID")
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", uuid::Uuid::from_u128(self.0).as_hyphenated())
    }
}

// Manually implement Serialize so we can serialize to and from a UUID
impl serde::Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::from_str(&s).map_err(serde::de::Error::custom)
    }
}

// &str -> Uuid conversion.
impl TryFrom<&str> for Uuid {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::from_str(s)?;
        Ok(uuid)
    }
}

// String -> Uuid conversion.
impl TryFrom<String> for Uuid {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::from_str(s.as_str())?;
        Ok(uuid)
    }
}

// u128 -> Uuid conversion.
impl From<u128> for Uuid {
    fn from(u: u128) -> Self {
        Uuid(u)
    }
}

// Uuid -> u128 conversion.
impl From<Uuid> for u128 {
    fn from(u: Uuid) -> Self {
        u.0
    }
}

// Uuid -> String conversion.
impl From<Uuid> for String {
    fn from(u: Uuid) -> Self {
        u.to_string()
    }
}

// uuid::Uuid -> Uuid conversion.
impl From<uuid::Uuid> for Uuid {
    fn from(u: uuid::Uuid) -> Self {
        Uuid(u.as_u128())
    }
}

// Uuid -> uuid::Uuid conversion.
impl From<Uuid> for uuid::Uuid {
    fn from(u: Uuid) -> Self {
        uuid::Uuid::from_u128(u.0)
    }
}

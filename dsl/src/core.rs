use core::fmt;
use std::{cmp::Ordering, hash::Hash, hash::Hasher};

// TODO it is very questionable to have this part of equality
#[derive(Debug, Clone)]
pub struct SourceLoc {
    pub start: usize,
    pub end: Option<usize>,
}

impl SourceLoc {
    pub fn new(start: usize) -> SourceLoc {
        SourceLoc { start, end: None }
    }

    pub fn range(start: usize, end: usize) -> SourceLoc {
        SourceLoc {
            start,
            end: Some(end),
        }
    }
}

impl PartialEq for SourceLoc {
    fn eq(&self, other: &Self) -> bool {
        // TODO this is dubious - two source locations are equal? But to some
        // degree this is true because we don't care about the location for
        // equality purposes.
        true
    }
}
impl Eq for SourceLoc {}

/// Implements Identifier declared by 2.1.2.
///
/// 61131-3 declares that identifiers are case insensitive.
/// This class ensures that we do case insensitive comparisons
/// and can use containers as appropriate.
pub struct Id {
    original: String,
    lower_case: String,
    location: Option<SourceLoc>,
}

impl Id {
    /// Converts a `&str` into an `Identifier`.
    pub fn from(str: &str) -> Id {
        Id {
            original: String::from(str),
            lower_case: String::from(str).to_lowercase(),
            location: None,
        }
    }

    pub fn with_location(mut self, loc: SourceLoc) -> Id {
        self.location = Some(loc);
        self
    }

    /// Converts an `Identifier` into a lower case `String`.
    pub fn lower_case(&self) -> &String {
        &self.lower_case
    }

    /// Get the location of the identifier
    pub fn location(&self) -> &Option<SourceLoc> {
        &self.location
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        let mut id = Id::from(self.original.as_str());
        if let Some(loc) = &self.location {
            id = id.with_location(loc.clone());
        }
        id
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.lower_case == other.lower_case
    }
}
impl Eq for Id {}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lower_case.hash(state);
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)
    }
}

//! Semantic rule that enumeration values in declarations must be unique.
//!
//! ## Passes
//!
//! TYPE
//! LOGLEVEL : (CRITICAL) := CRITICAL;
//! END_TYPE
//!
//! ## Fails
//!
//! //! TYPE
//! LOGLEVEL : (CRITICAL, CRITICAL) := CRITICAL;
//! END_TYPE
use ironplc_dsl::{dsl::*, visitor::Visitor};
use std::collections::HashSet;

use crate::error::SemanticDiagnostic;

pub fn apply(lib: &Library) -> Result<(), SemanticDiagnostic> {
    let mut visitor = RuleEnumerationValuesUnique {};
    visitor.walk(&lib)
}

struct RuleEnumerationValuesUnique {}

impl Visitor<SemanticDiagnostic> for RuleEnumerationValuesUnique {
    type Value = ();

    fn visit_enum_declaration(&mut self, node: &EnumerationDeclaration) -> Result<(), SemanticDiagnostic> {
        match &node.spec {
            EnumeratedSpecificationKind::TypeName(_) => return Ok(Self::Value::default()),
            EnumeratedSpecificationKind::Values(values) => {
                let mut seen_values = HashSet::new();
                for value in values {
                    if seen_values.contains(&value) {
                        return SemanticDiagnostic::error("S0004", format!(
                            "Enumeration declaration {} has duplicated value {}",
                            node.name, value
                        ));
                    }
                    seen_values.insert(value);
                }
                return Ok(Self::Value::default());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stages::parse;

    #[test]
    fn apply_when_values_unique_then_ok() {
        let program = "
TYPE
LOGLEVEL : (CRITICAL, ERROR);
END_TYPE";

        let library = parse(program).unwrap();
        let result = apply(&library);

        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn apply_when_value_duplicated_then_error() {
        let program = "
TYPE
LOGLEVEL : (CRITICAL, CRITICAL);
END_TYPE";

        let library = parse(program).unwrap();
        let result = apply(&library);

        assert_eq!(true, result.is_err());
    }
}

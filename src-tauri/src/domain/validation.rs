//! Pure input checks shared by the domain types. Each returns the cleaned value
//! or an `AppError::Validation` whose message can be shown to the user as-is.

use crate::error::{AppError, AppResult};

/// Trims `value` and checks it's non-empty and at most `max_chars` characters.
/// `field` names the input in the error message, e.g. "Account name".
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "first used by the M1 domain types")
)]
pub fn required_text(field: &str, value: &str, max_chars: usize) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{field} must not be empty")));
    }
    if trimmed.chars().count() > max_chars {
        return Err(AppError::Validation(format!(
            "{field} must be at most {max_chars} characters"
        )));
    }
    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validation_message(result: AppResult<String>) -> String {
        match result {
            Err(AppError::Validation(message)) => message,
            other => panic!("expected a validation error, got {other:?}"),
        }
    }

    #[test]
    fn required_text_trims_surrounding_whitespace() {
        assert_eq!(
            required_text("Name", "  Checking \n", 10).unwrap(),
            "Checking"
        );
    }

    #[test]
    fn required_text_rejects_empty_and_whitespace_only() {
        for value in ["", "   ", "\t\n"] {
            assert_eq!(
                validation_message(required_text("Name", value, 10)),
                "Name must not be empty"
            );
        }
    }

    #[test]
    fn required_text_rejects_values_over_the_limit() {
        assert_eq!(
            validation_message(required_text("Name", "abcdef", 5)),
            "Name must be at most 5 characters"
        );
    }

    #[test]
    fn required_text_counts_characters_not_bytes() {
        // "café" is 4 characters but 5 bytes.
        assert_eq!(required_text("Name", "café", 4).unwrap(), "café");
    }
}

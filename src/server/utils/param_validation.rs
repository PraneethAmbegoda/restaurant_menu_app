#![deny(warnings)]
#![deny(clippy::all)]

/// Validates that the given parameter string is a valid positive integer.
///
/// # Arguments
/// * `param` - The string representation of the parameter to be validated.
/// * `param_name` - The name of the parameter (used for error messages).
///
/// # Returns
/// * `Ok(u32)` - If the parameter is successfully parsed as a positive integer.
/// * `Err(String)` - If the parameter cannot be parsed as a valid positive integer, with an error message specifying the parameter name.
pub fn parse_path_param(param: &str, param_name: &str) -> Result<u32, String> {
    match param.parse::<u32>() {
        Ok(id) => Ok(id),
        Err(_) => Err(format!(
            "Invalid {}. Must be a valid positive integer.",
            param_name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path_param_valid() {
        let result = parse_path_param("123", "table_id");
        assert_eq!(result, Ok(123));
    }

    #[test]
    fn test_parse_path_param_invalid() {
        let result = parse_path_param("abc", "table_id");
        assert_eq!(
            result,
            Err("Invalid table_id. Must be a valid positive integer.".to_string())
        );
    }

    #[test]
    fn test_parse_path_param_negative_value() {
        let result = parse_path_param("-123", "table_id");
        assert_eq!(
            result,
            Err("Invalid table_id. Must be a valid positive integer.".to_string())
        );
    }

    #[test]
    fn test_parse_path_param_empty_string() {
        let result = parse_path_param("", "table_id");
        assert_eq!(
            result,
            Err("Invalid table_id. Must be a valid positive integer.".to_string())
        );
    }
}

#![deny(warnings)]
#![deny(clippy::all)]

/// Validates Parameters to be of Positive Integer
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

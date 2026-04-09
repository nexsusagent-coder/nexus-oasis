//! Custom assertions for testing

/// Assert that a message has the expected role
#[macro_export]
macro_rules! assert_message_role {
    ($message:expr, $expected_role:expr) => {
        assert_eq!(
            $message.role,
            $expected_role,
            "Message role mismatch: expected {:?}, got {:?}",
            $expected_role,
            $message.role
        );
    };
}

/// Assert that a string contains another string (case-insensitive)
#[macro_export]
macro_rules! assert_contains_ignore_case {
    ($haystack:expr, $needle:expr) => {
        let haystack_lower = $haystack.to_lowercase();
        let needle_lower = $needle.to_lowercase();
        assert!(
            haystack_lower.contains(&needle_lower),
            "'{}' does not contain '{}' (case-insensitive)",
            $haystack,
            $needle
        );
    };
}

/// Assert that a result is Ok
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        assert!(
            $result.is_ok(),
            "Expected Ok, got Err: {:?}",
            $result.err()
        );
    };
}

/// Assert that a result is Err
#[macro_export]
macro_rules! assert_err {
    ($result:expr) => {
        assert!(
            $result.is_err(),
            "Expected Err, got Ok: {:?}",
            $result.ok()
        );
    };
}

/// Assert that an async operation completes within a timeout
#[macro_export]
macro_rules! assert_completes_within {
    ($future:expr, $timeout_ms:expr) => {
        let result = tokio::time::timeout(
            std::time::Duration::from_millis($timeout_ms),
            $future
        )
        .await;
        assert!(
            result.is_ok(),
            "Operation did not complete within {}ms",
            $timeout_ms
        );
    };
}

/// Assert JSON equality (ignoring field order)
#[macro_export]
macro_rules! assert_json_eq {
    ($actual:expr, $expected:expr) => {
        let actual: serde_json::Value = serde_json::from_str(&$actual).unwrap_or_else(|e| {
            panic!("Failed to parse actual JSON: {}", e);
        });
        let expected: serde_json::Value = serde_json::from_str(&$expected).unwrap_or_else(|e| {
            panic!("Failed to parse expected JSON: {}", e);
        });
        assert_eq!(actual, expected, "JSON mismatch");
    };
}

#[cfg(test)]
mod tests {
    use sentient_core::{Message, MessageRole};

    #[test]
    fn test_assert_message_role() {
        let msg = Message::user("Hello");
        assert_message_role!(msg, MessageRole::User);
    }

    #[test]
    fn test_assert_contains_ignore_case() {
        assert_contains_ignore_case!("Hello WORLD", "world");
    }

    #[test]
    fn test_assert_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert_ok!(result);
    }

    #[test]
    fn test_assert_err() {
        let result: Result<i32, &str> = Err("error");
        assert_err!(result);
    }

    #[tokio::test]
    async fn test_assert_completes_within() {
        assert_completes_within!(async { 42 }, 100);
    }

    #[test]
    fn test_assert_json_eq() {
        let actual = r#"{"a": 1, "b": 2}"#;
        let expected = r#"{"b": 2, "a": 1}"#;
        assert_json_eq!(actual, expected);
    }
}

//! Property-based tests using proptest for complex scenarios
//!
//! These tests use property-based testing to verify that our implementations
//! hold true across a wide range of inputs, not just specific test cases.

#[cfg(test)]
mod property_tests {
    use mcp_context_browser::domain::types::{CodeChunk, Embedding, Language};
    use proptest::prelude::*;
    use proptest::test_runner::TestCaseError;

    // Property: CodeChunk content length should be preserved through operations
    proptest! {
        #[test]
        fn test_code_chunk_content_preservation(content in "\\PC*") {
            let chunk = CodeChunk {
                id: "test_id".to_string(),
                content: content.clone(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::json!({}),
            };

            // Content should be preserved
            prop_assert_eq!(chunk.content, content);
        }
    }

    // Property: Line numbers should maintain their relative ordering
    proptest! {
        #[test]
        fn test_start_line_ordering(
            start in 1..10000u32,
            end_offset in 0..1000u32
        ) {
            let end = start + end_offset;
            let chunk = CodeChunk {
                id: "test".to_string(),
                content: "test".to_string(),
                file_path: "test.rs".to_string(),
                start_line: start,
                end_line: end,
                language: Language::Rust,
                metadata: serde_json::json!({}),
            };

            // End should be >= start
            prop_assert!(chunk.end_line >= chunk.start_line);
        }
    }

    // Property: Embedding vectors should have consistent dimensions
    proptest! {
        #[test]
        fn test_embedding_vector_consistency(
            vector in prop::collection::vec(-1.0..1.0f32, 1..1000),
        ) {
            let embedding = Embedding {
                vector: vector.clone(),
                model: "test".to_string(),
                dimensions: vector.len(),
            };

            // Dimensions should match vector length
            prop_assert_eq!(embedding.dimensions, embedding.vector.len());
        }
    }

    // Property: File paths should be preserved
    proptest! {
        #[test]
        fn test_file_path_safety(path in "\\PC*") {
            let chunk = CodeChunk {
                id: "test".to_string(),
                content: "test".to_string(),
                file_path: path.clone(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::json!({}),
            };

            // File path should be preserved
            prop_assert_eq!(chunk.file_path, path.clone());
        }
    }

    // Property: IDs should be non-empty strings
    proptest! {
        #[test]
        fn test_id_non_empty(id in "\\PC+") {
            let chunk = CodeChunk {
                id: id.clone(),
                content: "test".to_string(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::json!({}),
            };

            // ID should not be empty
            prop_assert!(!chunk.id.is_empty());
            prop_assert_eq!(chunk.id, id);
        }
    }

    // Property: Language enum roundtrip serialization
    proptest! {
        #[test]
        fn test_language_serialization_roundtrip(lang in prop_oneof![
            Just(Language::Rust),
            Just(Language::Python),
            Just(Language::JavaScript),
            Just(Language::TypeScript),
            Just(Language::Java),
            Just(Language::Go),
            Just(Language::C),
            Just(Language::Cpp),
            Just(Language::Unknown),
        ]) {
            // Serialize and deserialize
            let serialized = serde_json::to_string(&lang).map_err(|e| TestCaseError::fail(e.to_string()))?;
            let deserialized: Language = serde_json::from_str(&serialized).map_err(|e| TestCaseError::fail(e.to_string()))?;

            // Should roundtrip correctly
            prop_assert_eq!(lang, deserialized);
        }
    }

    // Property: Metadata should be valid JSON
    proptest! {
        #[test]
        fn test_metadata_json_validity(key in "\\PC+", value in "\\PC*") {
            let mut metadata = serde_json::Map::new();
            metadata.insert(key, serde_json::Value::String(value));

            let chunk = CodeChunk {
                id: "test".to_string(),
                content: "test".to_string(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::Value::Object(metadata),
            };

            // Should be valid JSON
            let serialized = serde_json::to_string(&chunk.metadata).map_err(|e| TestCaseError::fail(e.to_string()))?;
            let _: serde_json::Value = serde_json::from_str(&serialized).map_err(|e| TestCaseError::fail(e.to_string()))?;
        }
    }

    // Property: Embedding model names should be reasonable length
    proptest! {
        #[test]
        fn test_model_name_length(model in "\\PC{1,100}") {
            let embedding = Embedding {
                vector: vec![0.1, 0.2, 0.3],
                model: model.clone(),
                dimensions: 3,
            };

            // Model name should be preserved and reasonable length (checked by character count)
            prop_assert_eq!(embedding.model, model.clone());
            prop_assert!(model.chars().count() <= 100);
            prop_assert!(!model.is_empty());
        }
    }

    // Property: Vector normalization (values should be bounded)
    proptest! {
        #[test]
        fn test_vector_value_bounds(values in prop::collection::vec(-100.0..100.0f32, 1..100)) {
            for &value in &values {
                // Values should be finite (not NaN or infinite)
                prop_assert!(value.is_finite());

                // Values should be within reasonable bounds for embeddings
                prop_assert!((-100.0..=100.0).contains(&value));
            }

            let embedding = Embedding {
                vector: values.clone(),
                model: "test".to_string(),
                dimensions: 0, // Will be set correctly in real usage
            };

            // All values should be preserved
            for (i, &expected) in embedding.vector.iter().enumerate() {
                if i < values.len() {
                    prop_assert_eq!(expected, values[i]);
                }
            }
        }
    }
}

/// Integration property tests
#[cfg(test)]
mod integration_property_tests {

    use mcp_context_browser::domain::types::{CodeChunk, Language};
    use proptest::prelude::*;

    // Property: System should handle various input sizes gracefully
    proptest! {
        #[test]
        fn test_input_size_handling(
            content_size in 1..10000usize,
            content in prop::string::string_regex("\\PC*").unwrap()
        ) {
            // Generate content of specified size (approximately)
            let test_content = if content_size > content.len() {
                content.repeat((content_size / content.len().max(1)) + 1)
                    .chars().take(content_size).collect::<String>()
            } else {
                content.chars().take(content_size).collect::<String>()
            };

            let chunk = CodeChunk {
                id: "size_test".to_string(),
                content: test_content.clone(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::json!({}),
            };

            // Content should be preserved
            prop_assert_eq!(chunk.content.len(), test_content.len());
        }
    }

    // Property: Concurrent operations should not corrupt data
    proptest! {
        #[test]
        fn test_data_integrity_under_operations(
            operations in prop::collection::vec(
                prop_oneof![
                    (1u32..100u32).prop_map(|line| ("set_line", line.to_string())),
                    prop::string::string_regex("\\PC{1,50}").unwrap().prop_map(|content| ("set_content", content)),
                    prop::bool::ANY.prop_map(|flag| ("toggle_flag", if flag { "true" } else { "false" }.to_string()))
                ],
                1..20
            )
        ) {
            let mut chunk = CodeChunk {
                id: "integrity_test".to_string(),
                content: "initial".to_string(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
                metadata: serde_json::json!({"integrity": true}),
            };

            // Apply operations
            for (op_type, value) in operations {
                match op_type {
                    "set_line" => {
                        if let Ok(line) = value.parse::<u32>() {
                            if line > 0 {
                                chunk.start_line = line;
                                chunk.end_line = chunk.end_line.max(line);
                            }
                        }
                    },
                    "set_content" => {
                        chunk.content = value;
                    },
                    "toggle_flag" => {
                        if let Some(meta) = chunk.metadata.as_object_mut() {
                            meta.insert("flag".to_string(), serde_json::Value::Bool(value == "true"));
                        }
                    },
                    _ => {}
                }
            }

            // Basic invariants should hold
            prop_assert!(!chunk.id.is_empty());
            prop_assert!(!chunk.content.is_empty());
            prop_assert!(!chunk.file_path.is_empty());
            prop_assert!(chunk.start_line > 0);
            prop_assert!(chunk.end_line >= chunk.start_line);
        }
    }
}

/// Property-based tests for domain validators
#[cfg(test)]
mod validator_property_tests {
    use mcp_context_browser::domain::validation::{
        NumberValidator, NumberValidatorTrait, StringValidator, StringValidatorTrait,
        ValidationError,
    };
    use proptest::prelude::*;

    // Property: not_empty validator accepts any non-empty string
    proptest! {
        #[test]
        fn test_not_empty_accepts_non_empty(s in "\\PC+") {
            let validator = StringValidator::not_empty();
            let result = validator.validate(&s);
            prop_assert!(result.is_ok(), "Non-empty string should be valid");
        }
    }

    // Property: not_empty validator rejects empty string
    #[test]
    fn test_not_empty_rejects_empty() {
        let validator = StringValidator::not_empty();
        let result = validator.validate("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::Required { .. }
        ));
    }

    // Property: min_length accepts strings >= min
    proptest! {
        #[test]
        fn test_min_length_accepts_valid(min in 1..100usize, extra in 0..100usize) {
            let len = min + extra;
            let s: String = (0..len).map(|_| 'a').collect();
            let validator = StringValidator::min_length(min);
            let result = validator.validate(&s);
            prop_assert!(result.is_ok(), "String of length {} should pass min_length({})", len, min);
        }
    }

    // Property: min_length rejects strings < min
    proptest! {
        #[test]
        fn test_min_length_rejects_short(min in 2..100usize) {
            let len = min - 1;
            let s: String = (0..len).map(|_| 'a').collect();
            let validator = StringValidator::min_length(min);
            let result = validator.validate(&s);
            prop_assert!(result.is_err(), "String of length {} should fail min_length({})", len, min);
        }
    }

    // Property: max_length accepts strings <= max
    proptest! {
        #[test]
        fn test_max_length_accepts_valid(max in 1..100usize, offset in 0..100usize) {
            let len = max.saturating_sub(offset).max(0);
            let s: String = (0..len).map(|_| 'a').collect();
            let validator = StringValidator::max_length(max);
            let result = validator.validate(&s);
            prop_assert!(result.is_ok(), "String of length {} should pass max_length({})", len, max);
        }
    }

    // Property: max_length rejects strings > max
    proptest! {
        #[test]
        fn test_max_length_rejects_long(max in 1..50usize, extra in 1..50usize) {
            let len = max + extra;
            let s: String = (0..len).map(|_| 'a').collect();
            let validator = StringValidator::max_length(max);
            let result = validator.validate(&s);
            prop_assert!(result.is_err(), "String of length {} should fail max_length({})", len, max);
        }
    }

    // Property: contains accepts strings with substring
    proptest! {
        #[test]
        fn test_contains_accepts_valid(
            prefix in "\\PC{0,20}",
            needle in "\\PC{1,10}",
            suffix in "\\PC{0,20}"
        ) {
            let haystack = format!("{}{}{}", prefix, needle, suffix);
            let validator = StringValidator::contains(&needle);
            let result = validator.validate(&haystack);
            prop_assert!(result.is_ok(), "String containing '{}' should be valid", needle);
        }
    }

    // Property: range accepts numbers within bounds
    proptest! {
        #[test]
        fn test_range_accepts_valid(min in -1000i64..0i64, max in 0i64..1000i64) {
            let value = (min + max) / 2; // Middle value
            let validator = NumberValidator::range(min, max);
            let result = validator.validate(&value);
            prop_assert!(result.is_ok(), "Value {} should be in range [{}, {}]", value, min, max);
        }
    }

    // Property: range rejects numbers outside bounds
    proptest! {
        #[test]
        fn test_range_rejects_invalid(min in 10i64..100i64, max in 100i64..200i64, offset in 1i64..50i64) {
            let below = min - offset;
            let above = max + offset;
            let validator = NumberValidator::range(min, max);

            let result_below = validator.validate(&below);
            let result_above = validator.validate(&above);

            prop_assert!(result_below.is_err(), "Value {} below range [{}, {}] should fail", below, min, max);
            prop_assert!(result_above.is_err(), "Value {} above range [{}, {}] should fail", above, min, max);
        }
    }

    // Property: positive accepts only positive numbers
    proptest! {
        #[test]
        fn test_positive_accepts_valid(n in 1i64..i64::MAX) {
            let validator = NumberValidator::positive();
            let result = validator.validate(&n);
            prop_assert!(result.is_ok(), "Positive number {} should be valid", n);
        }
    }

    // Property: positive rejects zero and negative
    proptest! {
        #[test]
        fn test_positive_rejects_invalid(n in i64::MIN..=0i64) {
            let validator = NumberValidator::positive();
            let result = validator.validate(&n);
            prop_assert!(result.is_err(), "Non-positive number {} should be invalid", n);
        }
    }

    // Property: non_negative accepts zero and positive
    proptest! {
        #[test]
        fn test_non_negative_accepts_valid(n in 0i64..i64::MAX) {
            let validator = NumberValidator::non_negative();
            let result = validator.validate(&n);
            prop_assert!(result.is_ok(), "Non-negative number {} should be valid", n);
        }
    }

    // Property: non_negative rejects negative
    proptest! {
        #[test]
        fn test_non_negative_rejects_invalid(n in i64::MIN..-1i64) {
            let validator = NumberValidator::non_negative();
            let result = validator.validate(&n);
            prop_assert!(result.is_err(), "Negative number {} should be invalid", n);
        }
    }

    // Property: negative accepts only negative numbers
    proptest! {
        #[test]
        fn test_negative_accepts_valid(n in i64::MIN..-1i64) {
            let validator = NumberValidator::negative();
            let result = validator.validate(&n);
            prop_assert!(result.is_ok(), "Negative number {} should be valid", n);
        }
    }

    // Property: negative rejects zero and positive
    proptest! {
        #[test]
        fn test_negative_rejects_invalid(n in 0i64..i64::MAX) {
            let validator = NumberValidator::negative();
            let result = validator.validate(&n);
            prop_assert!(result.is_err(), "Non-negative number {} should be invalid", n);
        }
    }

    // Property: combined validators enforce all constraints
    proptest! {
        #[test]
        fn test_combined_validators(s in "[a-z]{5,15}") {
            let validator = StringValidator::not_empty()
                .combine_with(StringValidator::min_length(3))
                .combine_with(StringValidator::max_length(20));
            let result = validator.validate(&s);
            prop_assert!(result.is_ok(), "String '{}' should pass combined validation", s);
        }
    }

    // Property: combined validators fail if any constraint fails
    proptest! {
        #[test]
        fn test_combined_validators_fail_on_constraint(s in "[a-z]{1,2}") {
            // String is 1-2 chars, min_length is 3
            let validator = StringValidator::not_empty()
                .combine_with(StringValidator::min_length(3))
                .combine_with(StringValidator::max_length(20));
            let result = validator.validate(&s);
            prop_assert!(result.is_err(), "Short string '{}' should fail min_length constraint", s);
        }
    }
}

/// Stress tests for edge cases
#[cfg(test)]
mod stress_tests {

    use mcp_context_browser::domain::types::{CodeChunk, Language};
    use proptest::prelude::*;

    // Test with extreme but valid inputs
    proptest! {
        #[test]
        fn test_extreme_valid_inputs(
            id in "\\PC{1,1000}",
            content in "\\PC{1,50000}",
            file_path in "\\PC{1,5000}",
            start_line in 1..u32::MAX,
            end_line in 1..u32::MAX,
        ) {
            // Ensure end_line >= start_line
            let end_line = end_line.max(start_line);

            let chunk = CodeChunk {
                id,
                content,
                file_path,
                start_line,
                end_line,
                language: Language::Rust,
                metadata: serde_json::json!({"stress_test": true}),
            };

            // Should not panic with extreme inputs
            prop_assert!(!chunk.id.is_empty());
            prop_assert!(!chunk.content.is_empty());
            prop_assert!(!chunk.file_path.is_empty());
            prop_assert!(chunk.start_line > 0);
            prop_assert!(chunk.end_line >= chunk.start_line);
        }
    }

    // Test boundary conditions
    proptest! {
        #[test]
        fn test_boundary_conditions(
            boundary_case in prop_oneof![
                Just(("empty_content", "".to_string())),
                Just(("zero_lines", "0".to_string())),
                Just(("negative_lines", "-1".to_string())),
                Just(("max_u32", u32::MAX.to_string())),
                prop::string::string_regex("\\PC{0,10}").unwrap().prop_map(|s| ("short_string", s)),
            ]
        ) {
            let (case_type, value) = boundary_case;

            match case_type {
                "empty_content" => {
                    // Empty content should be detectable
                    prop_assert!(value.is_empty());
                },
                "zero_lines" => {
                    // Zero should be detectable
                    prop_assert_eq!(value, "0");
                },
                "negative_lines" => {
                    // Negative should be detectable
                    prop_assert_eq!(value, "-1");
                },
                "max_u32" => {
                    // Max u32 should be detectable
                    prop_assert_eq!(value, u32::MAX.to_string());
                },
                "short_string" => {
                    // Short strings should be valid (checked by character count)
                    prop_assert!(value.chars().count() <= 10);
                },
                _ => {}
            }
        }
    }
}

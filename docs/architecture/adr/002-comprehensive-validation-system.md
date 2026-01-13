# ADR 002: Comprehensive Validation System

Date: 2026-01-07

## Status

Accepted

## Context

The MCP Context Browser handles complex business data structures (CodeChunk, Embedding, configuration) that require validation at multiple levels:

1.**Data integrity**: Ensuring required fields are present and valid
2.**Business rules**: Enforcing domain-specific constraints
3.**Security**: Preventing malicious input and injection attacks
4.**Performance**: Early validation to avoid expensive operations with invalid data

The current validation is scattered across the codebase with inconsistent patterns and incomplete coverage.

## Decision

Implement a comprehensive validation system using the `validator` crate with custom business logic validators. The system will provide:

1.**Declarative validation**using derive macros
2.**Custom validators**for business-specific rules
3.**Multi-layer validation**(input, business logic, security)
4.**Consistent error handling**with actionable messages
5.**Performance optimization**through early validation

## Consequences

### Positive

\1-  **Consistency**: Unified validation approach across all data structures
\1-  **Maintainability**: Centralized validation logic
\1-  **Security**: Comprehensive input sanitization and validation
\1-  **Performance**: Early rejection of invalid data
\1-  **Developer Experience**: Clear validation errors with actionable messages

### Negative

\1-  **Dependency**: Additional crate dependency
\1-  **Learning curve**: New validation DSL to learn
\1-  **Runtime overhead**: Validation execution time

### Risks

\1-  **Performance impact**: Validation on hot paths
\1-  **Error message quality**: Ensuring messages are actionable
\1-  **Coverage completeness**: Missing validation rules

## Implementation

### Data Structure Validation

```rust
#[derive(Debug, Validate)]
pub struct CodeChunk {
    #[validate(length(min = 1))]
    pub id: String,

    #[validate(length(min = 1, max = 10000))]
    pub content: String,

    #[validate(length(min = 1))]
    pub file_path: String,

    #[validate(range(min = 1))]
    pub start_line: u32,

    #[validate(range(min = 1))]
    #[validate(custom(function = "validate_line_range", arg = "&self.start_line"))]
    pub end_line: u32,
}
```

### Custom Validators

```rust
fn validate_file_path(path: &str) -> Result<(), ValidationError> {
    if path.is_empty() {
        return Err(ValidationError::new("Path cannot be empty"));
    }

    if path.contains("..") {
        return Err(ValidationError::new("Path cannot contain directory traversal"));
    }

    Ok(())
}

fn validate_line_range(end_line: u32, start_line: &u32) -> Result<(), ValidationError> {
    if*start_line > end_line {
        return Err(ValidationError::new("Start line cannot be greater than end line"));
    }
    Ok(())
}
```

### Business Logic Integration

```rust
impl CodeChunk {
    pub fn validate_business_rules(&self) -> Result<(), Error> {
        // Additional business logic validation beyond basic field validation
        if self.language == Language::Unknown && self.content.contains("fn ") {
            return Err(Error::validation("Unknown language but appears to be Rust code"));
        }

        Ok(())
    }
}
```

## Alternatives Considered

### Option 1: Manual Validation

```rust
if chunk.content.is_empty() {
    return Err("Content cannot be empty");
}
```

\1-  **Pros**: No dependencies, full control
\1-  **Cons**: Verbose, error-prone, inconsistent

### Option 2: Custom Derive Macros

\1-  **Pros**: Clean syntax, compile-time validation
\1-  **Cons**: Complex macro implementation, maintenance burden

### Option 3: JSON Schema Validation

\1-  **Pros**: Standard schemas, tooling support
\1-  **Cons**: Runtime-only, less type-safe

## Validation Layers

### 1. Input Validation

\1-   Required fields presence
\1-   Type constraints (String length, number ranges)
\1-   Format validation (paths, URLs)

### 2. Business Logic Validation

\1-   Domain-specific rules
\1-   Cross-field validation
\1-   Consistency checks

### 3. Security Validation

\1-   Path traversal prevention
\1-   XSS prevention
\1-   Injection attack prevention

### 4. Performance Validation

\1-   Size limits to prevent DoS
\1-   Complexity limits
\1-   Resource usage validation

## Error Handling

Validation errors provide:
\1-  **Field identification**: Which field failed validation
\1-  **Error type**: What validation rule was violated
\1-  **Actionable message**: How to fix the issue
\1-  **Context**: Additional debugging information

## References

\1-   [Validator Crate](https://docs.rs/validator/latest/validator/)
\1-   [Input Validation](https://owasp.org/www-community/Input_Validation_Cheat_Sheet)
\1-   [Domain Validation](https://martinfowler.com/bliki/EvansClassification.html)

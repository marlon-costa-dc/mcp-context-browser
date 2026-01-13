# ADR 011: HTTP Transport - Request-Response Pattern Over SSE Streaming

## Status

**Documented**(v0.1.0)

> Implemented with POST request-response pattern. SSE streaming deferred to v0.2.0.
> Current implementation in `src/server/transport/http.rs` provides:
>
> -   POST /MCP for client-to-server requests with immediate responses
> -   GET /MCP returns 501 Not Implemented with clear messaging
> -   Session management and message buffering infrastructure ready for future SSE support

## Context

The MCP (Model Context Protocol) specification defines a Streamable HTTP transport with:
\1-  **POST /MCP**: Client sends requests, receives responses (request-response pattern)
\1-  **GET /MCP**: Server streams updates to client via Server-Sent Events (SSE)

The current v0.1.0 implementation needed to decide whether to implement both patterns immediately or defer SSE streaming to a future release.

## Decision

**Implement request-response pattern only in v0.1.0**

1.**POST /MCP**: Fully functional request-response endpoint

\1-   Clients submit requests via POST and receive responses immediately
\1-   Supports session management, message buffering, and resumption
\1-   Works for all core functionality (search, index, etc.)

2.**GET /MCP**: Return 501 Not Implemented with clear messaging

\1-   Clients are explicitly informed SSE is not yet supported
\1-   Better than 200 OK with empty response (which would be misleading)
\1-   Infrastructure for SSE already in place (session handling, event IDs, message buffering)

## Rationale

### Why Request-Response is Sufficient for v0.1.0

1.**Functional Completeness**

\1-   All core MCP operations work via POST request-response pattern
\1-   Real-world usage patterns show POST is primary mechanism
\1-   Clients can implement polling for continuous updates if needed

2.**Reduced Complexity**

\1-   SSE streaming adds significant complexity:
\1-   Connection state management
\1-   Client reconnection handling
\1-   Event ordering and deduplication
\1-   Browser/HTTP proxy compatibility issues
\1-   Request-response is simpler, more reliable, and easier to debug

3.**Sufficient Infrastructure**

\1-   Session management already implemented
\1-   Message buffering for resumption ready
\1-   Event ID tracking (prepared for SSE)
\1-   No architectural changes needed for future SSE support

4.**Clear Communication**

\1-   Returning 501 with explanation is honest and helpful
\1-   Clients won't waste time trying to use non-existent feature
\1-   Sets clear expectations for v0.1.0 vs v0.2.0

### Why Defer to v0.2.0

1.**Future-Proof Architecture**

\1-   Current code structure supports adding SSE later
\1-   Session/buffering infrastructure is SSE-ready
\1-   No breaking changes needed when SSE is added

2.**Lower Risk**

\1-   Shipping v0.1.0 without SSE reduces complexity and bugs
\1-   Focus on core functionality quality
\1-   SSE can be added in incremental v0.2.0 release

3.**Alternative Patterns Available**

\1-   Clients can use polling with request-response
\1-   Pub/Sub via event bus for async notifications
\1-   WebSockets in future releases if needed

## Consequences

### Positive Consequences

1.**Reduced Scope & Risk**

\1-   v0.1.0 ships faster with fewer potential issues
\1-   Core functionality is stable and well-tested
\1-   Clear versioning and feature roadmap

2.**Better User Experience**

\1-   Explicit 501 response is better than misleading 200
\1-   Clients know exactly what's supported
\1-   No confusion about SSE vs polling

3.**Maintainability**

\1-   Simpler codebase easier to understand and modify
\1-   Request-response pattern is easier to test
\1-   Fewer edge cases to handle

4.**Architecture Flexibility**

\1-   Infrastructure in place for future SSE implementation
\1-   No need for breaking changes in v0.2.0
\1-   Can evaluate other patterns (WebSockets, gRPC) later

### Negative Consequences

1.**Limited Real-Time Streaming**

\1-   Clients must use polling or request-response for continuous updates
\1-   Not ideal for high-frequency update scenarios
\1-   SSE would be more efficient for server-pushed updates

2.**Incomplete Spec Compliance**

\1-   MCP specification defines streaming as optional feature
\1-   Current implementation doesn't fully support spec
\1-   May limit interoperability with certain MCP clients

3.**Feature Parity Gap**

\1-   Some MCP implementations may have SSE streaming
\1-   Users expecting full streaming support may be disappointed
\1-   v0.2.0 migration may require client code changes

## Alternatives Considered

### Alternative 1: Full SSE Implementation in v0.1.0

**Approach**: Implement Server-Sent Events streaming for GET /MCP endpoint

**Pros**:
\1-   Full MCP spec compliance
\1-   More efficient server-to-client updates
\1-   Real-time streaming support

**Cons**:
\1-   Significantly more complex implementation
\1-   Connection state management challenges
\1-   Browser/proxy compatibility issues
\1-   Higher risk of bugs in initial release
\1-   Delays v0.1.0 release

**Status**: Deferred to v0.2.0

### Alternative 2: Return 200 OK with Empty Stream

**Approach**: Respond to GET /MCP with 200 OK and empty SSE stream

**Pros**:
\1-   Spec-compliant response code

**Cons**:
\1-   Misleading to clients (implies working connection)
\1-   Clients won't know why they're getting no data
\1-   Encourages broken behavior
\1-   Bad user experience

**Status**: Rejected in favor of 501

### Alternative 3: WebSocket Transport

**Approach**: Use WebSocket instead of HTTP/SSE for bidirectional streaming

**Pros**:
\1-   Better for real-time bidirectional communication
\1-   Better performance for frequent updates
\1-   Simpler client libraries

**Cons**:
\1-   Not part of MCP spec (which uses HTTP)
\1-   Requires different client implementation
\1-   More infrastructure requirements
\1-   Out of scope for HTTP transport

**Status**: Deferred for potential v0.3.0 as alternative transport

## Implementation Notes

### Current State (v0.1.0)

```rust
// src/server/transport/http.rs:115-143
async fn handle_mcp_get(
    State(state): State<HttpTransportState>,
    headers: HeaderMap,
) -> Result<Response, McpError> {
    // Session validation happens here

    // TODO: Implement Server-Sent Events streaming
    // For now, return 501 to indicate not implemented
    Err(McpError::NotImplemented(
        "SSE streaming not yet implemented. Use POST for request-response communication."
            .to_string(),
    ))
}
```

### Ready for v0.2.0 Implementation

The following infrastructure is already in place:

1.**Session Management**(`src/server/transport/session.rs`)

\1-   Session creation and tracking
\1-   Activity timestamps
\1-   Message buffering

2.**Message Buffering**(in POST handler)

\1-   Event ID generation
\1-   Message history per session
\1-   Resumption support

3.**Error Handling**

\1-   McpError enum with NotImplemented variant
\1-   Response serialization infrastructure

### Migration Path to v0.2.0

```rust
// Pseudocode for v0.2.0 SSE implementation
async fn handle_mcp_get(
    State(state): State<HttpTransportState>,
    headers: HeaderMap,
) -> Result<Response, McpError> {
    // 1. Validate session (already done)
    let session_id = extract_session_id(&headers)?;
    let session = state.session_manager.get_session(&session_id)?;

    // 2. Create SSE stream
    let stream = create_sse_stream(
        state.session_manager.clone(),
        session_id.to_string(),
    );

    // 3. Return streaming response
    Ok(body_stream_response(stream))
}
```

## Recommendations

1.**Document clearly**in all client libraries and examples that GET /MCP is not yet implemented
2.**Monitor feedback**from users about SSE needs
3.**Plan v0.2.0**SSE implementation if users need real-time streaming
4.**Consider alternative patterns**if WebSocket demand grows (v0.3.0+)
5.**Update MCP compliance matrix**to note SSE as deferred feature

## References

\1-  **MCP Specification**: [Model Context Protocol](https://modelcontextprotocol.io/)
\1-  **Transport Layer**: `src/server/transport/http.rs`, `src/server/transport/session.rs`
\1-  **Related ADRs**:
\1-   ADR 001: Provider Pattern Architecture
\1-   ADR 002: Async-First Architecture
\1-  **Related Issues**: See GitHub issues tagged with "sse" or "streaming"

## Reviewers

\1-   Architecture Review: Pending
\1-   Security Review: Pending (for v0.2.0 SSE implementation)

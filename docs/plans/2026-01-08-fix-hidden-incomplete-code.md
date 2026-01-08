# Fix Hidden/Subtle Incomplete Implementations

> **IMPORTANT:** Start with fresh context. Run `/clear` before `/implement`.

Created: 2026-01-08
Status: PENDING

> **Status Lifecycle:** PENDING → COMPLETE → VERIFIED
>
> -   PENDING: Initial state, awaiting implementation
> -   COMPLETE: All tasks implemented (set by /implement)
> -   VERIFIED: Rules supervisor passed (set automatically)

## Summary

**Goal:** Fix subtle, hidden incomplete implementations that are not in the main fake implementations plan. These are stubs, workarounds, and simplified code buried within modules.

**Architecture:**

- Use proper cryptographic JWT via `jsonwebtoken` crate (already in dependencies)
- Replace hardcoded fallbacks with actual system queries
- Implement missing repository methods properly
- Remove unsafe assumptions (`.unwrap_or(true)`)

**Tech Stack:** Rust, Tokio, jsonwebtoken, sysinfo, existing infrastructure

## Scope

### In Scope

- Fixing cryptographically insecure JWT in core/auth.rs
- Implementing missing repository methods
- Replacing hardcoded fallbacks with real values
- Removing unsafe health assumptions
- Fixing Language parsing from metadata

### Out of Scope

- Tasks already in `2026-01-08-fix-fake-implementations.md`
- New feature development
- UI changes

## Prerequisites

- `jsonwebtoken` crate (already in Cargo.toml)
- `sysinfo` crate (already in Cargo.toml)
- Existing test infrastructure

## Feature Inventory

### Files Being Modified

| File | Issue | Priority | Task |
|------|-------|----------|------|
| `src/core/auth.rs` | Fake JWT with seahash | CRITICAL | Task 1 |
| `src/repository/chunk_repository.rs` | 4 stub methods | HIGH | Task 2 |
| `src/services/context.rs` | embed_text not implemented | HIGH | Task 3 |
| `src/providers/vector_store/edgevec.rs` | collection_exists stub | HIGH | Task 4 |
| `src/providers/routing/health.rs` | Assumes healthy if unknown | MEDIUM | Task 5 |
| `src/admin/service.rs` | Status assumed "active" | MEDIUM | Task 6 |
| `src/core/limits.rs` | Hardcoded fallback values | LOW | Task 7 |
| `src/admin/handlers.rs` | User context simplified | LOW | Task 8 |

### Feature Mapping Verification

- [x] All hidden implementations listed
- [x] Priority assigned based on security/functionality impact
- [x] Every feature has a task number
- [x] No overlap with main fake implementations plan

## Progress Tracking

**MANDATORY: Update this checklist as tasks complete.**

- [ ] Task 1: Fix Core Auth JWT Implementation (CRITICAL)
- [ ] Task 2: Fix Chunk Repository Stubs
- [ ] Task 3: Fix Context Service embed_text
- [ ] Task 4: Fix EdgeVec collection_exists
- [ ] Task 5: Fix Health Monitor Assumptions
- [ ] Task 6: Fix Provider Status Assumptions
- [ ] Task 7: Replace Hardcoded Limits Fallbacks
- [ ] Task 8: Fix Admin User Context

**Total Tasks:** 8 | **Completed:** 0 | **Remaining:** 8

## Implementation Tasks

### Task 1: Fix Core Auth JWT Implementation (CRITICAL)

**Objective:** Replace insecure seahash-based JWT with proper cryptographic implementation.

**Files:**

- Modify: `src/core/auth.rs`
- Test: `tests/core/auth_test.rs`

**Current Problem (lines 288-309):**

```rust
// Simplified signature (not cryptographically secure - use proper JWT library!)
let signature = format!("{:x}", seahash::hash(message.as_bytes()));
```

**Implementation Steps:**

1. **Use jsonwebtoken crate** (already in Cargo.toml):
   - Replace `encode_token` to use `jsonwebtoken::encode`
   - Replace `decode_token` to use `jsonwebtoken::decode`
   - Use `Algorithm::HS256` for HMAC-SHA256 signing
2. **Update Claims struct** if needed for jsonwebtoken compatibility
3. **Remove seahash dependency** from auth (it's a non-cryptographic hash)
4. **Update tests** to verify proper JWT validation

**Definition of Done:**

- [ ] JWT signed with HMAC-SHA256 via jsonwebtoken
- [ ] No seahash in auth module
- [ ] Token validation rejects tampered tokens
- [ ] Tests pass

### Task 2: Fix Chunk Repository Stubs

**Objective:** Implement the 4 stub methods in VectorStoreChunkRepository.

**Files:**

- Modify: `src/repository/chunk_repository.rs`

**Implementation Steps:**

1. **`find_by_id` (line 105-108):**
   - Query vector store with ID filter in metadata
   - Return actual chunk if found, None if not exists
2. **`find_by_collection` zero vector (line 115):**
   - Use a proper query strategy instead of zero vector
   - Option A: Store a representative vector per collection
   - Option B: Use metadata-only query if supported
3. **`language` parsing (line 138):**
   - Parse language from metadata `language` field
   - Use `Language::from_str()` or pattern matching
   - Fallback to `Language::Unknown` not `Language::Rust`
4. **`delete` (line 147-151):**
   - Implement delete by searching for chunk ID in metadata
   - Call vector store delete with found vector IDs

**Definition of Done:**

- [ ] `find_by_id` returns real data or None
- [ ] No zero vector workaround
- [ ] Language parsed from metadata
- [ ] Delete works for individual chunks

### Task 3: Fix Context Service embed_text

**Objective:** Implement repository-based embedding in ContextService.

**Files:**

- Modify: `src/services/context.rs`

**Current Problem (line 432):**

```rust
pub async fn embed_text(&self, _text: &str) -> Result<Embedding> {
    Err(Error::generic("Repository-based embedding not implemented"))
}
```

**Implementation Steps:**

1. **Inject embedding provider** into ContextService (if not already)
2. **Call embedding provider:**
   ```rust
   pub async fn embed_text(&self, text: &str) -> Result<Embedding> {
       self.embedding_provider.embed(text).await
   }
   ```
3. **Verify** the ContextService has access to embedding_provider

**Definition of Done:**

- [ ] `embed_text` calls real embedding provider
- [ ] No "not implemented" error
- [ ] Tests verify embeddings are generated

### Task 4: Fix EdgeVec collection_exists

**Objective:** Implement real collection existence check in EdgeVec provider.

**Files:**

- Modify: `src/providers/vector_store/edgevec.rs`

**Current Problem (lines 221-224):**

```rust
async fn collection_exists(&self, _name: &str) -> Result<bool> {
    // Simple check - for now just return true if it's the current collection
    Ok(true)
}
```

**Implementation Steps:**

1. **Track collections in actor:**
   - Add `collections: HashSet<String>` to EdgeVecActor
   - Update on create_collection/delete_collection
2. **Implement check via message:**
   - Add `CollectionExists { name, tx }` message variant
   - Actor checks if collection is in the set
3. **Return real result:**
   - `Ok(true)` if collection exists
   - `Ok(false)` if it doesn't

**Definition of Done:**

- [ ] Returns false for non-existent collections
- [ ] Returns true only for created collections
- [ ] Tests verify both cases

### Task 5: Fix Health Monitor Assumptions

**Objective:** Don't assume providers are healthy when status is unknown.

**Files:**

- Modify: `src/providers/routing/health.rs`

**Current Problem (line 254):**

```rust
.unwrap_or(true) // Assume healthy if unknown
```

**Implementation Steps:**

1. **Change default behavior:**
   - Unknown status should return `false` (not healthy)
   - Or better: return `Option<bool>` to distinguish unknown from unhealthy
2. **Add explicit unknown state:**
   - Modify `is_healthy` to return `Option<bool>` or add `is_known` method
3. **Update callers** to handle unknown state properly
4. **Trigger health check** when status is unknown

**Definition of Done:**

- [ ] Unknown providers not assumed healthy
- [ ] Callers handle unknown state
- [ ] Health check triggered for unknown providers

### Task 6: Fix Provider Status Assumptions

**Objective:** Get real provider status instead of assuming "active".

**Files:**

- Modify: `src/admin/service.rs`
- Modify: `src/server/server.rs`

**Current Problem (lines 423, 434, 440, 451):**

```rust
status: "active".to_string(), // Assume active for now
```

**Implementation Steps:**

1. **Query health monitor:**
   - Get actual health status from HealthMonitor
   - Map `ProviderHealthStatus` to status string
2. **Status values:**
   - "healthy" / "active" if healthy
   - "unhealthy" / "degraded" if not healthy
   - "unknown" if not yet checked
3. **Update both locations** in admin service and server

**Definition of Done:**

- [ ] Status reflects real health
- [ ] No hardcoded "active" assumptions
- [ ] Status updates when health changes

### Task 7: Replace Hardcoded Limits Fallbacks

**Objective:** Get real system info on non-Linux platforms.

**Files:**

- Modify: `src/core/limits.rs`

**Current Problem (lines 447-517):**

```rust
#[cfg(not(target_os = "linux"))]
{
    // Fallback for non-Linux systems
    Ok(MemoryStats {
        total: 8 * 1024 * 1024 * 1024, // 8GB assumed
        ...
    })
}
```

**Implementation Steps:**

1. **Use sysinfo crate** on all platforms:
   - `sysinfo` works on Windows, macOS, Linux
   - Remove `#[cfg(not(target_os = "linux"))]` fallbacks
2. **Unify implementation:**
   - Single code path using sysinfo
   - Works across all platforms
3. **Keep fallback only for edge cases:**
   - If sysinfo fails, use conservative defaults with warning log

**Definition of Done:**

- [ ] Real system stats on all platforms
- [ ] No hardcoded 8GB/256GB values
- [ ] Fallback only on sysinfo failure

### Task 8: Fix Admin User Context

**Objective:** Extract user from JWT claims, not simplified placeholder.

**Files:**

- Modify: `src/admin/handlers.rs`

**Current Problem (line 338):**

```rust
// Get user from request context (simplified - in real implementation, get from JWT)
```

**Implementation Steps:**

1. **Access claims from request extensions:**
   - Claims should be inserted by auth middleware
   - Extract: `req.extensions().get::<Claims>()`
2. **Use real user info:**
   - Username from `claims.sub`
   - Role from `claims.role`
3. **Handle missing claims:**
   - Return 401 if claims not present
   - Or use default "anonymous" with limited permissions

**Definition of Done:**

- [ ] User info from JWT claims
- [ ] No simplified placeholder
- [ ] Proper error handling

## Testing Strategy

- **Unit tests:** Each fixed method should have corresponding tests
- **Security tests:** Verify JWT properly validates/rejects tokens
- **Platform tests:** Verify sysinfo works on CI platforms
- **Gatekeeper:** `make test` and `make lint` must pass after each task

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking JWT compatibility | Medium | High | Version tokens, migration path |
| Performance regression in health checks | Low | Medium | Cache health status |
| Platform-specific sysinfo issues | Low | Low | Keep minimal fallback |

## Open Questions

None. All implementations follow established patterns.

---

**USER: Please review this plan. Edit any section directly, then confirm to proceed.**

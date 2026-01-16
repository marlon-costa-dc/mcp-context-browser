//! Hybrid Search Configuration Tests

use mcb_infrastructure::adapters::hybrid_search::HybridSearchConfig;

#[test]
fn test_default_config() {
    let config = HybridSearchConfig::default();
    assert!(config.enabled);
    assert!(config.bm25_weight > 0.0);
    assert!(config.semantic_weight > 0.0);
    assert!((config.bm25_weight + config.semantic_weight - 1.0).abs() < 0.01);
}

#[test]
fn test_with_weights() {
    let config = HybridSearchConfig::with_weights(0.3, 0.7);
    assert!((config.bm25_weight - 0.3).abs() < 0.01);
    assert!((config.semantic_weight - 0.7).abs() < 0.01);
}

#[test]
fn test_semantic_only() {
    let config = HybridSearchConfig::semantic_only();
    assert!((config.bm25_weight).abs() < 0.01);
    assert!((config.semantic_weight - 1.0).abs() < 0.01);
}

#[test]
fn test_bm25_only() {
    let config = HybridSearchConfig::bm25_only();
    assert!((config.bm25_weight - 1.0).abs() < 0.01);
    assert!((config.semantic_weight).abs() < 0.01);
}

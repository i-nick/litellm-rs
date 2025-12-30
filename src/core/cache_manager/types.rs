//! Cache manager type definitions
//!
//! This module contains all the type definitions for the cache manager,
//! including configuration, cache entries, keys, and statistics.

use crate::core::models::openai::ChatCompletionRequest;
use crate::utils::perf::strings::intern_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Type alias for semantic cache mapping
pub type SemanticCacheMap = HashMap<String, Vec<(CacheKey, f32)>>;

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of entries in the cache
    pub max_entries: usize,
    /// Default TTL for cache entries
    pub default_ttl: Duration,
    /// Enable semantic caching
    pub enable_semantic: bool,
    /// Semantic similarity threshold (0.0 to 1.0)
    pub similarity_threshold: f32,
    /// Minimum prompt length for caching
    pub min_prompt_length: usize,
    /// Enable compression for large responses
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            default_ttl: Duration::from_secs(3600), // 1 hour
            enable_semantic: true,
            similarity_threshold: 0.95,
            min_prompt_length: 10,
            enable_compression: true,
        }
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    /// The cached value
    pub value: T,
    /// When the entry was created
    pub created_at: Instant,
    /// When the entry expires
    pub expires_at: Instant,
    /// Access count for popularity tracking
    pub access_count: u64,
    /// Last access time
    pub last_accessed: Instant,
    /// Size in bytes (estimated)
    pub size_bytes: usize,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(value: T, ttl: Duration, size_bytes: usize) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            expires_at: now + ttl,
            access_count: 0,
            last_accessed: now,
            size_bytes,
        }
    }

    /// Check if the entry is expired
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }

    /// Mark the entry as accessed
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }

    /// Get the age of the entry
    pub fn age(&self) -> Duration {
        Instant::now().duration_since(self.created_at)
    }
}

/// Cache key for efficient lookups
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// Model name (interned for efficiency)
    pub model: Arc<str>,
    /// Request hash
    pub request_hash: u64,
    /// Optional user ID for user-specific caching
    pub user_id: Option<Arc<str>>,
}

impl CacheKey {
    /// Create a new cache key from a request
    pub fn from_request(request: &ChatCompletionRequest, user_id: Option<&str>) -> Self {
        let model = intern_string(&request.model);
        let request_hash = Self::hash_request(request);
        let user_id = user_id.map(intern_string);

        Self {
            model,
            request_hash,
            user_id,
        }
    }

    /// Hash a request for cache key generation
    fn hash_request(request: &ChatCompletionRequest) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();

        // Hash the messages
        for message in &request.messages {
            message.role.hash(&mut hasher);
            if let Some(content) = &message.content {
                content.hash(&mut hasher);
            }
        }

        // Hash other relevant parameters
        request.temperature.map(|t| t.to_bits()).hash(&mut hasher);
        request.max_tokens.hash(&mut hasher);
        request.top_p.map(|p| p.to_bits()).hash(&mut hasher);
        request
            .frequency_penalty
            .map(|p| p.to_bits())
            .hash(&mut hasher);
        request
            .presence_penalty
            .map(|p| p.to_bits())
            .hash(&mut hasher);
        request.stop.hash(&mut hasher);

        hasher.finish()
    }
}

/// Atomic cache statistics for lock-free hot path updates
#[derive(Debug, Default)]
pub struct AtomicCacheStats {
    /// L1 cache hits
    pub l1_hits: AtomicU64,
    /// L1 cache misses
    pub l1_misses: AtomicU64,
    /// L2 cache hits
    pub l2_hits: AtomicU64,
    /// L2 cache misses
    pub l2_misses: AtomicU64,
    /// Semantic cache hits
    pub semantic_hits: AtomicU64,
    /// Semantic cache misses
    pub semantic_misses: AtomicU64,
    /// Cache evictions
    pub evictions: AtomicU64,
    /// Total cache size in bytes
    pub total_size_bytes: AtomicUsize,
}

/// Cache statistics snapshot (returned to callers)
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// L1 cache hits
    pub l1_hits: u64,
    /// L1 cache misses
    pub l1_misses: u64,
    /// L2 cache hits
    pub l2_hits: u64,
    /// L2 cache misses
    pub l2_misses: u64,
    /// Semantic cache hits
    pub semantic_hits: u64,
    /// Semantic cache misses
    pub semantic_misses: u64,
    /// Cache evictions
    pub evictions: u64,
    /// Total cache size in bytes
    pub total_size_bytes: usize,
}

impl CacheStats {
    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let total_hits = self.l1_hits + self.l2_hits + self.semantic_hits;
        let total_requests = total_hits + self.l1_misses + self.l2_misses + self.semantic_misses;

        if total_requests == 0 {
            0.0
        } else {
            total_hits as f64 / total_requests as f64
        }
    }
}

impl AtomicCacheStats {
    /// Create a snapshot of current stats
    pub fn snapshot(&self) -> CacheStats {
        CacheStats {
            l1_hits: self.l1_hits.load(Ordering::Relaxed),
            l1_misses: self.l1_misses.load(Ordering::Relaxed),
            l2_hits: self.l2_hits.load(Ordering::Relaxed),
            l2_misses: self.l2_misses.load(Ordering::Relaxed),
            semantic_hits: self.semantic_hits.load(Ordering::Relaxed),
            semantic_misses: self.semantic_misses.load(Ordering::Relaxed),
            evictions: self.evictions.load(Ordering::Relaxed),
            total_size_bytes: self.total_size_bytes.load(Ordering::Relaxed),
        }
    }

    /// Reset all stats to zero
    pub fn reset(&self) {
        self.l1_hits.store(0, Ordering::Relaxed);
        self.l1_misses.store(0, Ordering::Relaxed);
        self.l2_hits.store(0, Ordering::Relaxed);
        self.l2_misses.store(0, Ordering::Relaxed);
        self.semantic_hits.store(0, Ordering::Relaxed);
        self.semantic_misses.store(0, Ordering::Relaxed);
        self.evictions.store(0, Ordering::Relaxed);
        self.total_size_bytes.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // ==================== CacheConfig Tests ====================

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert_eq!(config.max_entries, 10000);
        assert_eq!(config.default_ttl, Duration::from_secs(3600));
        assert!(config.enable_semantic);
        assert!((config.similarity_threshold - 0.95).abs() < 0.001);
        assert_eq!(config.min_prompt_length, 10);
        assert!(config.enable_compression);
    }

    #[test]
    fn test_cache_config_custom() {
        let config = CacheConfig {
            max_entries: 5000,
            default_ttl: Duration::from_secs(1800),
            enable_semantic: false,
            similarity_threshold: 0.8,
            min_prompt_length: 20,
            enable_compression: false,
        };
        assert_eq!(config.max_entries, 5000);
        assert!(!config.enable_semantic);
    }

    #[test]
    fn test_cache_config_clone() {
        let config1 = CacheConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.max_entries, config2.max_entries);
        assert_eq!(config1.default_ttl, config2.default_ttl);
    }

    #[test]
    fn test_cache_config_serialize() {
        let config = CacheConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("max_entries"));
        assert!(json.contains("10000"));
    }

    #[test]
    fn test_cache_config_deserialize() {
        let json = r#"{
            "max_entries": 5000,
            "default_ttl": {"secs": 600, "nanos": 0},
            "enable_semantic": false,
            "similarity_threshold": 0.9,
            "min_prompt_length": 5,
            "enable_compression": true
        }"#;
        let config: CacheConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.max_entries, 5000);
        assert!(!config.enable_semantic);
    }

    // ==================== CacheEntry Tests ====================

    #[test]
    fn test_cache_entry_new() {
        let entry = CacheEntry::new("test value", Duration::from_secs(60), 100);
        assert_eq!(entry.value, "test value");
        assert_eq!(entry.access_count, 0);
        assert_eq!(entry.size_bytes, 100);
    }

    #[test]
    fn test_cache_entry_not_expired() {
        let entry = CacheEntry::new("test", Duration::from_secs(3600), 10);
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_expired() {
        let entry = CacheEntry::new("test", Duration::from_millis(1), 10);
        thread::sleep(Duration::from_millis(10));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_entry_mark_accessed() {
        let mut entry = CacheEntry::new("test", Duration::from_secs(60), 10);
        assert_eq!(entry.access_count, 0);

        entry.mark_accessed();
        assert_eq!(entry.access_count, 1);

        entry.mark_accessed();
        entry.mark_accessed();
        assert_eq!(entry.access_count, 3);
    }

    #[test]
    fn test_cache_entry_age() {
        let entry = CacheEntry::new("test", Duration::from_secs(60), 10);
        thread::sleep(Duration::from_millis(10));
        let age = entry.age();
        assert!(age >= Duration::from_millis(10));
    }

    #[test]
    fn test_cache_entry_clone() {
        let entry1 = CacheEntry::new(42, Duration::from_secs(60), 8);
        let entry2 = entry1.clone();
        assert_eq!(entry1.value, entry2.value);
        assert_eq!(entry1.size_bytes, entry2.size_bytes);
    }

    #[test]
    fn test_cache_entry_with_struct_value() {
        #[derive(Clone, Debug, PartialEq)]
        struct Response {
            content: String,
            tokens: u32,
        }

        let response = Response {
            content: "Hello".to_string(),
            tokens: 10,
        };
        let entry = CacheEntry::new(response.clone(), Duration::from_secs(60), 50);
        assert_eq!(entry.value, response);
    }

    // ==================== CacheKey Tests ====================

    #[test]
    fn test_cache_key_creation() {
        let key = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        assert_eq!(&*key.model, "gpt-4");
        assert_eq!(key.request_hash, 12345);
        assert!(key.user_id.is_none());
    }

    #[test]
    fn test_cache_key_with_user_id() {
        let key = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: Some(Arc::from("user-123")),
        };
        assert_eq!(key.user_id, Some(Arc::from("user-123")));
    }

    #[test]
    fn test_cache_key_equality() {
        let key1 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        let key2 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cache_key_inequality_model() {
        let key1 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        let key2 = CacheKey {
            model: Arc::from("gpt-3.5"),
            request_hash: 12345,
            user_id: None,
        };
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_key_inequality_hash() {
        let key1 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        let key2 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 54321,
            user_id: None,
        };
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_cache_key_hash_trait() {
        use std::collections::HashSet;

        let key1 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };
        let key2 = key1.clone();

        let mut set = HashSet::new();
        set.insert(key1);
        assert!(set.contains(&key2));
    }

    #[test]
    fn test_cache_key_clone() {
        let key1 = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: Some(Arc::from("user-1")),
        };
        let key2 = key1.clone();
        assert_eq!(key1, key2);
    }

    // ==================== CacheStats Tests ====================

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert_eq!(stats.l1_hits, 0);
        assert_eq!(stats.l1_misses, 0);
        assert_eq!(stats.l2_hits, 0);
        assert_eq!(stats.l2_misses, 0);
        assert_eq!(stats.semantic_hits, 0);
        assert_eq!(stats.semantic_misses, 0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.total_size_bytes, 0);
    }

    #[test]
    fn test_cache_stats_hit_rate_zero() {
        let stats = CacheStats::default();
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_rate_100_percent() {
        let stats = CacheStats {
            l1_hits: 100,
            l1_misses: 0,
            l2_hits: 0,
            l2_misses: 0,
            semantic_hits: 0,
            semantic_misses: 0,
            evictions: 0,
            total_size_bytes: 0,
        };
        assert_eq!(stats.hit_rate(), 1.0);
    }

    #[test]
    fn test_cache_stats_hit_rate_50_percent() {
        let stats = CacheStats {
            l1_hits: 50,
            l1_misses: 50,
            l2_hits: 0,
            l2_misses: 0,
            semantic_hits: 0,
            semantic_misses: 0,
            evictions: 0,
            total_size_bytes: 0,
        };
        assert_eq!(stats.hit_rate(), 0.5);
    }

    #[test]
    fn test_cache_stats_hit_rate_combined() {
        let stats = CacheStats {
            l1_hits: 30,
            l1_misses: 20,
            l2_hits: 20,
            l2_misses: 10,
            semantic_hits: 10,
            semantic_misses: 10,
            evictions: 0,
            total_size_bytes: 0,
        };
        // Total hits = 30 + 20 + 10 = 60
        // Total requests = 60 + 20 + 10 + 10 = 100
        assert_eq!(stats.hit_rate(), 0.6);
    }

    #[test]
    fn test_cache_stats_clone() {
        let stats1 = CacheStats {
            l1_hits: 100,
            l1_misses: 50,
            l2_hits: 25,
            l2_misses: 10,
            semantic_hits: 5,
            semantic_misses: 2,
            evictions: 10,
            total_size_bytes: 1024,
        };
        let stats2 = stats1.clone();
        assert_eq!(stats1.l1_hits, stats2.l1_hits);
        assert_eq!(stats1.total_size_bytes, stats2.total_size_bytes);
    }

    // ==================== AtomicCacheStats Tests ====================

    #[test]
    fn test_atomic_cache_stats_default() {
        let stats = AtomicCacheStats::default();
        assert_eq!(stats.l1_hits.load(Ordering::Relaxed), 0);
        assert_eq!(stats.l1_misses.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_atomic_cache_stats_increment() {
        let stats = AtomicCacheStats::default();
        stats.l1_hits.fetch_add(1, Ordering::Relaxed);
        stats.l1_hits.fetch_add(1, Ordering::Relaxed);
        assert_eq!(stats.l1_hits.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_atomic_cache_stats_snapshot() {
        let atomic_stats = AtomicCacheStats::default();
        atomic_stats.l1_hits.store(100, Ordering::Relaxed);
        atomic_stats.l1_misses.store(50, Ordering::Relaxed);
        atomic_stats.l2_hits.store(25, Ordering::Relaxed);
        atomic_stats.evictions.store(10, Ordering::Relaxed);
        atomic_stats.total_size_bytes.store(1024, Ordering::Relaxed);

        let snapshot = atomic_stats.snapshot();
        assert_eq!(snapshot.l1_hits, 100);
        assert_eq!(snapshot.l1_misses, 50);
        assert_eq!(snapshot.l2_hits, 25);
        assert_eq!(snapshot.evictions, 10);
        assert_eq!(snapshot.total_size_bytes, 1024);
    }

    #[test]
    fn test_atomic_cache_stats_reset() {
        let stats = AtomicCacheStats::default();
        stats.l1_hits.store(100, Ordering::Relaxed);
        stats.l1_misses.store(50, Ordering::Relaxed);
        stats.evictions.store(10, Ordering::Relaxed);
        stats.total_size_bytes.store(1024, Ordering::Relaxed);

        stats.reset();

        assert_eq!(stats.l1_hits.load(Ordering::Relaxed), 0);
        assert_eq!(stats.l1_misses.load(Ordering::Relaxed), 0);
        assert_eq!(stats.evictions.load(Ordering::Relaxed), 0);
        assert_eq!(stats.total_size_bytes.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_atomic_cache_stats_concurrent_updates() {
        use std::sync::Arc;

        let stats = Arc::new(AtomicCacheStats::default());
        let mut handles = vec![];

        for _ in 0..10 {
            let stats_clone = Arc::clone(&stats);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    stats_clone.l1_hits.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(stats.l1_hits.load(Ordering::Relaxed), 1000);
    }

    #[test]
    fn test_atomic_cache_stats_snapshot_independence() {
        let atomic_stats = AtomicCacheStats::default();
        atomic_stats.l1_hits.store(100, Ordering::Relaxed);

        let snapshot = atomic_stats.snapshot();

        // Modify atomic stats after snapshot
        atomic_stats.l1_hits.store(200, Ordering::Relaxed);

        // Snapshot should retain original value
        assert_eq!(snapshot.l1_hits, 100);
        assert_eq!(atomic_stats.l1_hits.load(Ordering::Relaxed), 200);
    }

    // ==================== SemanticCacheMap Type Alias Test ====================

    #[test]
    fn test_semantic_cache_map_type() {
        let mut map: SemanticCacheMap = HashMap::new();

        let key = CacheKey {
            model: Arc::from("gpt-4"),
            request_hash: 12345,
            user_id: None,
        };

        map.insert("embedding_hash".to_string(), vec![(key.clone(), 0.95)]);

        assert!(map.contains_key("embedding_hash"));
        let entries = map.get("embedding_hash").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, key);
        assert!((entries[0].1 - 0.95).abs() < 0.001);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_cache_entry_workflow() {
        let config = CacheConfig::default();
        let mut entry = CacheEntry::new("cached response", config.default_ttl, 50);

        assert!(!entry.is_expired());
        assert_eq!(entry.access_count, 0);

        entry.mark_accessed();
        entry.mark_accessed();
        entry.mark_accessed();

        assert_eq!(entry.access_count, 3);
        assert!(entry.age() < config.default_ttl);
    }

    #[test]
    fn test_stats_hit_rate_precision() {
        let stats = CacheStats {
            l1_hits: 333,
            l1_misses: 667,
            l2_hits: 0,
            l2_misses: 0,
            semantic_hits: 0,
            semantic_misses: 0,
            evictions: 0,
            total_size_bytes: 0,
        };
        let rate = stats.hit_rate();
        assert!((rate - 0.333).abs() < 0.001);
    }
}

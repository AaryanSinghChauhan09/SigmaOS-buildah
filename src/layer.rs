//! Buildah layer.

/// Layer kind
#[derive(Debug, Clone, PartialEq)]
pub enum LayerKind {
    /// Rootfs layer
    Rootfs,
    /// Tmpfs layer
    Tmpfs,
    /// Overlay layer
    Overlay,
    /// Snapshot layer
    Snapshot,
}

/// Layer
#[derive(Debug, Clone)]
pub struct Layer {
    /// Layer ID
    pub id: String,
    /// Layer name
    pub name: String,
    /// Layer kind
    pub kind: LayerKind,
    /// Size in bytes
    pub size_bytes: u64,
    /// Parent layer ID
    pub parent_id: Option<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl Layer {
    /// Create new layer
    pub fn new(name: String, kind: LayerKind, size_bytes: u64) -> Self {
        Self {
            id: format!("layer-{}-{}", name, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            kind,
            size_bytes,
            parent_id: None,
            created_at: Self::now(),
        }
    }
    
    /// Set parent
    pub fn set_parent(&mut self, parent_id: String) {
        self.parent_id = Some(parent_id);
    }
    
    /// Get layer age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

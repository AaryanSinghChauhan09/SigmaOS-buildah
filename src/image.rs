//! Buildah image.

/// Image kind
#[derive(Debug, Clone, PartialEq)]
pub enum ImageKind {
    /// Base image
    Base,
    /// Intermediate image
    Intermediate,
    /// Final image
    Final,
}

/// Image state
#[derive(Debug, Clone, PartialEq)]
pub enum ImageState {
    /// Available image
    Available,
    /// Pulling image
    Pulling,
    /// Building image
    Building,
    /// Removed image
    Removed,
}

/// Image info
#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    /// Size in bytes
    pub size_bytes: u64,
    /// Layers count
    pub layers_count: usize,
    /// Created timestamp
    pub created_at: u64,
    /// Digest
    pub digest: String,
}

/// Buildah image
#[derive(Debug, Clone)]
pub struct Image {
    /// Image ID
    pub id: String,
    /// Image name
    pub name: String,
    /// Image tag
    pub tag: String,
    /// Image kind
    pub kind: ImageKind,
    /// Image state
    pub state: ImageState,
    /// Image info
    pub info: ImageInfo,
    /// Created timestamp
    pub created_at: u64,
}

impl Image {
    /// Create new image
    pub fn new(name: String, tag: String, kind: ImageKind) -> Self {
        Self {
            id: format!("image-{}-{}-{}", name, tag, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            tag,
            kind,
            state: ImageState::Available,
            info: ImageInfo::default(),
            created_at: Self::now(),
        }
    }
    
    /// Set size
    pub fn set_size(&mut self, size_bytes: u64) {
        self.info.size_bytes = size_bytes;
    }
    
    /// Set layers
    pub fn set_layers(&mut self, layers: usize) {
        self.info.layers_count = layers;
    }
    
    /// Set digest
    pub fn set_digest(&mut self, digest: String) {
        self.info.digest = digest;
    }
    
    /// Pull image
    pub fn pull(&mut self) {
        self.state = ImageState::Pulling;
    }
    
    /// Mark available
    pub fn available(&mut self) {
        self.state = ImageState::Available;
    }
    
    /// Mark building
    pub fn mark_building(&mut self) {
        self.state = ImageState::Building;
    }
    
    /// Remove image
    pub fn remove(&mut self) {
        self.state = ImageState::Removed;
    }
    
    /// Get full name
    pub fn get_full_name(&self) -> String {
        format!("{}:{}", self.name, self.tag)
    }
    
    /// Get image age in seconds
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

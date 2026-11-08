//! Buildah mount.

/// Mount kind
#[derive(Debug, Clone, PartialEq)]
pub enum MountKind {
    /// Bind mount
    Bind,
    /// Volume mount
    Volume,
    /// Tmpfs mount
    Tmpfs,
    /// Proc mount
    Proc,
    /// Sysfs mount
    Sysfs,
    /// Devpts mount
    Devpts,
}

/// Mount state
#[derive(Debug, Clone, PartialEq)]
pub enum MountState {
    /// Unmounted
    Unmounted,
    /// Mounted
    Mounted,
    /// Remounting
    Remounting,
    /// Failed
    Failed,
}

/// Mount
#[derive(Debug, Clone)]
pub struct Mount {
    /// Mount ID
    pub id: String,
    /// Source path
    pub source: String,
    /// Target path
    pub target: String,
    /// Mount kind
    pub kind: MountKind,
    /// Mount state
    pub state: MountState,
    /// Read-only flag
    pub read_only: bool,
    /// Created timestamp
    pub created_at: u64,
}

impl Mount {
    /// Create new mount
    pub fn new(source: String, target: String, kind: MountKind) -> Self {
        Self {
            id: format!("mount-{}-{}", kind_name(&kind), std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            source,
            target,
            kind,
            state: MountState::Unmounted,
            read_only: false,
            created_at: Self::now(),
        }
    }
    
    /// Set read-only
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
    
    /// Mount
    pub fn mount(&mut self) {
        self.state = MountState::Mounted;
    }
    
    /// Unmount
    pub fn unmount(&mut self) {
        self.state = MountState::Unmounted;
    }
    
    /// Remount
    pub fn remount(&mut self) {
        self.state = MountState::Remounting;
    }
    
    /// Fail mount
    pub fn fail(&mut self) {
        self.state = MountState::Failed;
    }
    
    /// Get mount age in seconds
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

fn kind_name(kind: &MountKind) -> &'static str {
    match kind {
        MountKind::Bind => "bind",
        MountKind::Volume => "volume",
        MountKind::Tmpfs => "tmpfs",
        MountKind::Proc => "proc",
        MountKind::Sysfs => "sysfs",
        MountKind::Devpts => "devpts",
    }
}

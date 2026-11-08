//! Buildah container.

/// Container state
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerState {
    /// Created container
    Created,
    /// Running container
    Running,
    /// Exited container
    Exited,
    /// Removed container
    Removed,
}

/// Container config
#[derive(Debug, Clone, Default)]
pub struct ContainerConfig {
    /// Environment variables
    pub env: Vec<(String, String)>,
    /// Working directory
    pub workdir: String,
    /// User
    pub user: String,
    /// Entrypoint
    pub entrypoint: Vec<String>,
    /// Command
    pub cmd: Vec<String>,
    /// Exposed ports
    pub ports: Vec<u16>,
}

/// Container
#[derive(Debug, Clone)]
pub struct Container {
    /// Container ID
    pub id: String,
    /// Container name
    pub name: String,
    /// Image name
    pub image: String,
    /// Container state
    pub state: ContainerState,
    /// Container config
    pub config: ContainerConfig,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Created timestamp
    pub created_at: u64,
}

impl Container {
    /// Create new container
    pub fn new(name: String, image: String) -> Self {
        Self {
            id: format!("container-{}-{}", name, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            image,
            state: ContainerState::Created,
            config: ContainerConfig::default(),
            exit_code: None,
            created_at: Self::now(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ContainerConfig) {
        self.config = config;
    }
    
    /// Run container
    pub fn run(&mut self) {
        self.state = ContainerState::Running;
    }
    
    /// Exit container
    pub fn exit(&mut self, code: i32) {
        self.state = ContainerState::Exited;
        self.exit_code = Some(code);
    }
    
    /// Remove container
    pub fn remove(&mut self) {
        self.state = ContainerState::Removed;
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == ContainerState::Running
    }
    
    /// Get container age in seconds
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

//! Buildah builder.

/// Builder state
#[derive(Debug, Clone, PartialEq)]
pub enum BuilderState {
    /// Idle builder
    Idle,
    /// Building
    Building,
    /// Committing
    Committing,
    /// Done
    Done,
    /// Failed
    Failed,
}

/// Build step
#[derive(Debug, Clone)]
pub enum BuildStep {
    /// FROM instruction
    From(String),
    /// RUN instruction
    Run(String),
    /// COPY instruction
    Copy { src: String, dst: String },
    /// ADD instruction
    Add { src: String, dst: String },
    /// ENV instruction
    Env { key: String, value: String },
    /// WORKDIR instruction
    Workdir(String),
    /// EXPOSE instruction
    Expose(u16),
    /// CMD instruction
    Cmd(String),
    /// ENTRYPOINT instruction
    Entrypoint(String),
    /// USER instruction
    User(String),
}

/// Build info
#[derive(Debug, Clone, Default)]
pub struct BuildInfo {
    /// Steps completed
    pub steps_completed: usize,
    /// Steps failed
    pub steps_failed: usize,
    /// Build log
    pub log: Vec<String>,
    /// Base image
    pub base_image: Option<String>,
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Success flag
    pub success: bool,
    /// Error message
    pub error: Option<String>,
    /// Image ID
    pub image_id: Option<String>,
    /// Build duration in seconds
    pub duration_secs: u64,
}

/// Buildah builder
#[derive(Debug, Clone)]
pub struct Builder {
    /// Builder ID
    pub id: String,
    /// Builder name
    pub name: String,
    /// Builder tag
    pub tag: String,
    /// Builder state
    pub state: BuilderState,
    /// Build steps
    pub steps: Vec<BuildStep>,
    /// Build info
    pub info: BuildInfo,
    /// Created timestamp
    pub created_at: u64,
}

impl Builder {
    /// Create new builder
    pub fn new(name: String, tag: String) -> Self {
        Self {
            id: format!("builder-{}-{}", name, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            tag,
            state: BuilderState::Idle,
            steps: Vec::new(),
            info: BuildInfo::default(),
            created_at: Self::now(),
        }
    }
    
    /// Add build step
    pub fn add_step(&mut self, step: BuildStep) {
        if let BuildStep::From(ref img) = step {
            self.info.base_image = Some(img.clone());
        }
        self.steps.push(step);
    }
    
    /// Build image
    pub fn build(&mut self) -> BuildResult {
        self.state = BuilderState::Building;
        
        // Simulate build
        let mut success = true;
        let mut error = None;
        let mut image_id = None;
        let start = Self::now();
        
        for (i, step) in self.steps.iter().enumerate() {
            match step {
                BuildStep::From(img) => {
                    if img == "nonexistent" {
                        success = false;
                        error = Some(format!("Base image '{}' not found", img));
                        self.info.steps_failed += 1;
                        break;
                    }
                    self.info.log.push(format!("Step {}: FROM {}", i + 1, img));
                }
                BuildStep::Run(cmd) => {
                    self.info.log.push(format!("Step {}: RUN {}", i + 1, cmd));
                }
                _ => {
                    self.info.log.push(format!("Step {}: {:?}", i + 1, step));
                }
            }
            self.info.steps_completed += 1;
        }
        
        let duration = Self::now() - start;
        
        if success {
            self.state = BuilderState::Done;
            image_id = Some(format!("sha256:{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()));
        } else {
            self.state = BuilderState::Failed;
        }
        
        BuildResult {
            success,
            error,
            image_id,
            duration_secs: duration,
        }
    }
    
    /// Commit image
    pub fn commit(&mut self) -> Option<String> {
        if self.state == BuilderState::Done {
            Some(format!("{}:{}", self.name, self.tag))
        } else {
            None
        }
    }
    
    /// Reset builder
    pub fn reset(&mut self) {
        self.state = BuilderState::Idle;
        self.steps.clear();
        self.info = BuildInfo::default();
    }
    
    /// Get builder age in seconds
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

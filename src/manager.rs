//! Buildah manager.

use std::collections::HashMap;
use crate::builder::{Builder, BuildStep, BuildResult};
use crate::image::{Image, ImageKind, ImageState};
use crate::container::{Container, ContainerState};
use crate::mount::{Mount, MountKind, MountState};
use crate::layer::Layer;

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total builders
    pub total_builders: usize,
    /// Building builders
    pub building_builders: usize,
    /// Total images
    pub total_images: usize,
    /// Available images
    pub available_images: usize,
    /// Total containers
    pub total_containers: usize,
    /// Running containers
    pub running_containers: usize,
    /// Total mounts
    pub total_mounts: usize,
    /// Mounted mounts
    pub mounted_mounts: usize,
    /// Total layers
    pub total_layers: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Buildah manager
#[derive(Debug, Clone)]
pub struct BuildahManager {
    /// Builders
    builders: HashMap<String, Builder>,
    /// Images
    images: HashMap<String, Image>,
    /// Containers
    containers: HashMap<String, Container>,
    /// Mounts
    mounts: HashMap<String, Mount>,
    /// Layers
    layers: HashMap<String, Layer>,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for BuildahManager {
    fn default() -> Self {
        Self {
            builders: HashMap::new(),
            images: HashMap::new(),
            containers: HashMap::new(),
            mounts: HashMap::new(),
            layers: HashMap::new(),
            statistics: ManagerStatistics {
                total_builders: 0,
                building_builders: 0,
                total_images: 0,
                available_images: 0,
                total_containers: 0,
                running_containers: 0,
                total_mounts: 0,
                mounted_mounts: 0,
                total_layers: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl BuildahManager {
    /// Create new buildah manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Builder methods
    
    /// Add builder
    pub fn add_builder(&mut self, name: String, tag: String) -> String {
        let builder = Builder::new(name.clone(), tag.clone());
        let id = builder.id.clone();
        
        self.builders.insert(id.clone(), builder);
        self.update_statistics();
        id
    }
    
    /// Get builder
    pub fn get_builder(&self, builder_id: &str) -> Option<&Builder> {
        self.builders.get(builder_id)
    }
    
    /// Get builder mutably
    pub fn get_builder_mut(&mut self, builder_id: &str) -> Option<&mut Builder> {
        self.builders.get_mut(builder_id)
    }
    
    /// Get all builders
    pub fn list_builders(&self) -> Vec<&Builder> {
        self.builders.values().collect()
    }
    
    /// Remove builder
    pub fn remove_builder(&mut self, builder_id: &str) -> bool {
        if self.builders.remove(builder_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add build step
    pub fn add_build_step(&mut self, builder_id: &str, step: BuildStep) -> bool {
        if let Some(builder) = self.builders.get_mut(builder_id) {
            builder.add_step(step);
            return true;
        }
        false
    }
    
    /// Build builder
    pub fn build(&mut self, builder_id: &str) -> Option<BuildResult> {
        if let Some(builder) = self.builders.get_mut(builder_id) {
            let result = builder.build();
            self.update_statistics();
            return Some(result);
        }
        None
    }
    
    /// Commit builder
    pub fn commit(&mut self, builder_id: &str) -> Option<String> {
        if let Some(builder) = self.builders.get_mut(builder_id) {
            let result = builder.commit();
            if result.is_some() {
                self.update_statistics();
            }
            return result;
        }
        None
    }
    
    // Image methods
    
    /// Add image
    pub fn add_image(&mut self, name: String, tag: String, kind: ImageKind) -> String {
        let image = Image::new(name.clone(), tag.clone(), kind);
        let id = image.id.clone();
        
        self.images.insert(id.clone(), image);
        self.update_statistics();
        id
    }
    
    /// Get image
    pub fn get_image(&self, image_id: &str) -> Option<&Image> {
        self.images.get(image_id)
    }
    
    /// Get image mutably
    pub fn get_image_mut(&mut self, image_id: &str) -> Option<&mut Image> {
        self.images.get_mut(image_id)
    }
    
    /// Get all images
    pub fn list_images(&self) -> Vec<&Image> {
        self.images.values().collect()
    }
    
    /// Remove image
    pub fn remove_image(&mut self, image_id: &str) -> bool {
        if self.images.remove(image_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Pull image
    pub fn pull_image(&mut self, image_id: &str) -> bool {
        if let Some(image) = self.images.get_mut(image_id) {
            image.pull();
            image.available();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get images by state
    pub fn get_images_by_state(&self, state: &ImageState) -> Vec<&Image> {
        self.images.values()
            .filter(|i| i.state == *state)
            .collect()
    }
    
    /// Get available images
    pub fn get_available_images(&self) -> Vec<&Image> {
        self.get_images_by_state(&ImageState::Available)
    }
    
    /// Search images
    pub fn search_images(&self, query: &str) -> Vec<&Image> {
        let query_lower = query.to_lowercase();
        self.images.values()
            .filter(|i| i.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Container methods
    
    /// Add container
    pub fn add_container(&mut self, name: String, image_id: &str) -> String {
        let image_name = self.images.get(image_id)
            .map(|i| i.get_full_name())
            .unwrap_or_else(|| image_id.to_string());
        
        let container = Container::new(name.clone(), image_name);
        let id = container.id.clone();
        
        self.containers.insert(id.clone(), container);
        self.update_statistics();
        id
    }
    
    /// Get container
    pub fn get_container(&self, container_id: &str) -> Option<&Container> {
        self.containers.get(container_id)
    }
    
    /// Get container mutably
    pub fn get_container_mut(&mut self, container_id: &str) -> Option<&mut Container> {
        self.containers.get_mut(container_id)
    }
    
    /// Get all containers
    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }
    
    /// Remove container
    pub fn remove_container(&mut self, container_id: &str) -> bool {
        if self.containers.remove(container_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Run container
    pub fn run_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.run();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop container
    pub fn stop_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.exit(0);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get containers by state
    pub fn get_containers_by_state(&self, state: &ContainerState) -> Vec<&Container> {
        self.containers.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get running containers
    pub fn get_running_containers(&self) -> Vec<&Container> {
        self.get_containers_by_state(&ContainerState::Running)
    }
    
    // Mount methods
    
    /// Add mount
    pub fn add_mount(&mut self, source: String, target: String, kind: MountKind) -> String {
        let mount = Mount::new(source.clone(), target.clone(), kind);
        let id = mount.id.clone();
        
        self.mounts.insert(id.clone(), mount);
        self.update_statistics();
        id
    }
    
    /// Get mount
    pub fn get_mount(&self, mount_id: &str) -> Option<&Mount> {
        self.mounts.get(mount_id)
    }
    
    /// Get mount mutably
    pub fn get_mount_mut(&mut self, mount_id: &str) -> Option<&mut Mount> {
        self.mounts.get_mut(mount_id)
    }
    
    /// Get all mounts
    pub fn list_mounts(&self) -> Vec<&Mount> {
        self.mounts.values().collect()
    }
    
    /// Remove mount
    pub fn remove_mount(&mut self, mount_id: &str) -> bool {
        if self.mounts.remove(mount_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Mount
    pub fn mount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.mount();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Unmount
    pub fn unmount(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.unmount();
            self.update_statistics();
            return true;
        }
        false
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_builders = self.builders.len();
        self.statistics.building_builders = self.builders.values()
            .filter(|b| b.state == crate::builder::BuilderState::Building)
            .count();
        self.statistics.total_images = self.images.len();
        self.statistics.available_images = self.images.values()
            .filter(|i| i.state == ImageState::Available)
            .count();
        self.statistics.total_containers = self.containers.len();
        self.statistics.running_containers = self.containers.values()
            .filter(|c| c.is_running())
            .count();
        self.statistics.total_mounts = self.mounts.len();
        self.statistics.mounted_mounts = self.mounts.values()
            .filter(|m| m.state == MountState::Mounted)
            .count();
        self.statistics.total_layers = self.layers.len();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            images: self.images.values().cloned().collect(),
            containers: self.containers.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.images.clear();
        for image in export.images {
            self.images.insert(image.id.clone(), image);
        }
        
        self.containers.clear();
        for container in export.containers {
            self.containers.insert(container.id.clone(), container);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Images
    pub images: Vec<Image>,
    /// Containers
    pub containers: Vec<Container>,
}

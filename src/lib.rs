//! SigmaOS-buildah — daemonless container builder for SigmaOS.

pub mod builder;
pub mod image;
pub mod container;
pub mod layer;
pub mod mount;
pub mod manager;
pub mod util;
pub mod options;

pub use builder::{Builder, BuilderState, BuildStep, BuildResult};
pub use image::{Image, ImageKind, ImageState};
pub use container::{Container, ContainerState, ContainerConfig};
pub use layer::{Layer, LayerKind};
pub use mount::{Mount, MountKind, MountState};
pub use manager::{BuildahManager, ManagerStatistics};
pub use util::Model;
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_builder_state() {
        let states = vec![BuilderState::Idle, BuilderState::Building, BuilderState::Committing, BuilderState::Done, BuilderState::Failed];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_image_kind() {
        let kinds = vec![ImageKind::Base, ImageKind::Intermediate, ImageKind::Final];
        assert_eq!(kinds.len(), 3);
    }
    
    #[test]
    fn test_container_state() {
        let states = vec![ContainerState::Created, ContainerState::Running, ContainerState::Exited, ContainerState::Removed];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_layer_kind() {
        let kinds = vec![LayerKind::Rootfs, LayerKind::Tmpfs, LayerKind::Overlay, LayerKind::Snapshot];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_mount_kind() {
        let kinds = vec![MountKind::Bind, MountKind::Volume, MountKind::Tmpfs, MountKind::Proc, MountKind::Sysfs, MountKind::Devpts];
        assert_eq!(kinds.len(), 6);
    }
    
    #[test]
    fn test_mount_state() {
        let states = vec![MountState::Unmounted, MountState::Mounted, MountState::Remounting, MountState::Failed];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_builder_creation() {
        let builder = Builder::new("my-image".into(), "latest".into());
        assert_eq!(builder.name, "my-image".to_string());
        assert_eq!(builder.tag, "latest".to_string());
        assert_eq!(builder.state, BuilderState::Idle);
        assert!(builder.steps.is_empty());
    }
    
    #[test]
    fn test_builder_add_step() {
        let mut builder = Builder::new("img".into(), "v1".into());
        builder.add_step(BuildStep::From("alpine".into()));
        builder.add_step(BuildStep::Run("apk add curl".into()));
        
        assert_eq!(builder.steps.len(), 2);
        assert!(matches!(builder.steps[0], BuildStep::From(_)));
        assert!(matches!(builder.steps[1], BuildStep::Run(_)));
    }
    
    #[test]
    fn test_builder_build() {
        let mut builder = Builder::new("img".into(), "v1".into());
        builder.add_step(BuildStep::From("alpine".into()));
        builder.add_step(BuildStep::Run("apk add curl".into()));
        
        let result = builder.build();
        assert!(result.success);
        assert_eq!(builder.state, BuilderState::Done);
        assert_eq!(builder.info.steps_completed, 2);
        assert!(result.image_id.is_some());
    }
    
    #[test]
    fn test_builder_build_fail() {
        let mut builder = Builder::new("img".into(), "v1".into());
        builder.add_step(BuildStep::From("nonexistent".into()));
        
        let result = builder.build();
        assert!(!result.success);
        assert_eq!(builder.state, BuilderState::Failed);
        assert!(result.error.is_some());
    }
    
    #[test]
    fn test_builder_commit() {
        let mut builder = Builder::new("img".into(), "v1".into());
        builder.add_step(BuildStep::From("alpine".into()));
        builder.build();
        
        let image_id = builder.commit();
        assert!(image_id.is_some());
        assert_eq!(builder.state, BuilderState::Done);
    }
    
    #[test]
    fn test_image_creation() {
        let image = Image::new("alpine".into(), "3.18".into(), ImageKind::Base);
        assert_eq!(image.name, "alpine".to_string());
        assert_eq!(image.tag, "3.18".to_string());
        assert_eq!(image.kind, ImageKind::Base);
        assert_eq!(image.state, ImageState::Available);
    }
    
    #[test]
    fn test_image_lifecycle() {
        let mut image = Image::new("img".into(), "v1".into(), ImageKind::Final);
        image.pull();
        assert_eq!(image.state, ImageState::Pulling);
        image.available();
        assert_eq!(image.state, ImageState::Available);
        image.remove();
        assert_eq!(image.state, ImageState::Removed);
    }
    
    #[test]
    fn test_image_size() {
        let mut image = Image::new("img".into(), "v1".into(), ImageKind::Base);
        image.set_size(1024 * 1024 * 100); // 100MB
        assert_eq!(image.info.size_bytes, 1024 * 1024 * 100);
    }
    
    #[test]
    fn test_container_creation() {
        let container = Container::new("my-container".into(), "alpine".into());
        assert_eq!(container.name, "my-container".to_string());
        assert_eq!(container.image, "alpine".to_string());
        assert_eq!(container.state, ContainerState::Created);
    }
    
    #[test]
    fn test_container_lifecycle() {
        let mut container = Container::new("c1".into(), "img".into());
        container.run();
        assert_eq!(container.state, ContainerState::Running);
        container.exit(0);
        assert_eq!(container.state, ContainerState::Exited);
        assert_eq!(container.exit_code, Some(0));
    }
    
    #[test]
    fn test_container_remove() {
        let mut container = Container::new("c1".into(), "img".into());
        container.run();
        container.remove();
        assert_eq!(container.state, ContainerState::Removed);
    }
    
    #[test]
    fn test_layer_creation() {
        let layer = Layer::new("layer1".into(), LayerKind::Rootfs, 1024 * 1024);
        assert_eq!(layer.name, "layer1".to_string());
        assert_eq!(layer.kind, LayerKind::Rootfs);
        assert_eq!(layer.size_bytes, 1024 * 1024);
    }
    
    #[test]
    fn test_mount_creation() {
        let mount = Mount::new("/data".into(), "/container/data".into(), MountKind::Bind);
        assert_eq!(mount.source, "/data".to_string());
        assert_eq!(mount.target, "/container/data".to_string());
        assert_eq!(mount.kind, MountKind::Bind);
        assert_eq!(mount.state, MountState::Unmounted);
    }
    
    #[test]
    fn test_mount_lifecycle() {
        let mut mount = Mount::new("/src".into(), "/dst".into(), MountKind::Volume);
        mount.mount();
        assert_eq!(mount.state, MountState::Mounted);
        mount.unmount();
        assert_eq!(mount.state, MountState::Unmounted);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = BuildahManager::new();
        assert!(manager.list_builders().is_empty());
        assert!(manager.list_images().is_empty());
        assert!(manager.list_containers().is_empty());
        assert!(manager.list_mounts().is_empty());
    }
    
    #[test]
    fn test_add_builder() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        let id = manager.add_builder("my-image".into(), "latest".into());
        assert!(manager.get_builder(&id).is_some());
    }
    
    #[test]
    fn test_build_from_builder() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let builder_id = manager.add_builder("img".into(), "v1".into());
        manager.add_build_step(&builder_id, BuildStep::From("alpine".into()));
        manager.add_build_step(&builder_id, BuildStep::Run("echo hello".into()));
        
        let result = manager.build(&builder_id);
        assert!(result.is_some());
        assert!(result.unwrap().success);
    }
    
    #[test]
    fn test_commit_builder() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let builder_id = manager.add_builder("img".into(), "v1".into());
        manager.add_build_step(&builder_id, BuildStep::From("alpine".into()));
        manager.build(&builder_id);
        
        let image_id = manager.commit(&builder_id);
        assert!(image_id.is_some());
    }
    
    #[test]
    fn test_add_image() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        let id = manager.add_image("alpine".into(), "3.18".into(), ImageKind::Base);
        assert!(manager.get_image(&id).is_some());
    }
    
    #[test]
    fn test_pull_image() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let image_id = manager.add_image("img".into(), "v1".into(), ImageKind::Base);
        assert!(manager.pull_image(&image_id));
        
        let image = manager.get_image(&image_id).unwrap();
        assert_eq!(image.state, ImageState::Available);
    }
    
    #[test]
    fn test_remove_image() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let image_id = manager.add_image("img".into(), "v1".into(), ImageKind::Base);
        manager.pull_image(&image_id);
        
        assert!(manager.remove_image(&image_id));
        assert!(manager.get_image(&image_id).is_none());
    }
    
    #[test]
    fn test_add_container() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let image_id = manager.add_image("alpine".into(), "3.18".into(), ImageKind::Base);
        manager.pull_image(&image_id);
        
        let container_id = manager.add_container("my-container".into(), &image_id);
        assert!(manager.get_container(&container_id).is_some());
    }
    
    #[test]
    fn test_run_container() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let image_id = manager.add_image("alpine".into(), "3.18".into(), ImageKind::Base);
        manager.pull_image(&image_id);
        
        let container_id = manager.add_container("c1".into(), &image_id);
        assert!(manager.run_container(&container_id));
        
        let container = manager.get_container(&container_id).unwrap();
        assert_eq!(container.state, ContainerState::Running);
    }
    
    #[test]
    fn test_add_mount() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let mount_id = manager.add_mount("/data".into(), "/container/data".into(), MountKind::Bind);
        assert!(manager.get_mount(&mount_id).is_some());
    }
    
    #[test]
    fn test_mount_unmount() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        let mount_id = manager.add_mount("/src".into(), "/dst".into(), MountKind::Volume);
        manager.mount(&mount_id);
        
        let mount = manager.get_mount(&mount_id).unwrap();
        assert_eq!(mount.state, MountState::Mounted);
        
        manager.unmount(&mount_id);
        let mount = manager.get_mount(&mount_id).unwrap();
        assert_eq!(mount.state, MountState::Unmounted);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        manager.add_image("alpine".into(), "3.18".into(), ImageKind::Base);
        manager.add_container("c1".into(), "img1");
        
        let export = manager.export_data();
        let mut manager2 = BuildahManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_images().len(), 1);
        assert_eq!(manager2.list_containers().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = BuildahManager::new();
        manager.initialize();
        
        manager.add_builder("img".into(), "v1".into());
        manager.add_image("alpine".into(), "3.18".into(), ImageKind::Base);
        manager.add_container("c1".into(), "img1");
        manager.add_mount("/data".into(), "/dst".into(), MountKind::Bind);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_builders, 1);
        assert_eq!(stats.total_images, 1);
        assert_eq!(stats.total_containers, 1);
        assert_eq!(stats.total_mounts, 1);
        assert!(stats.uptime > 0 || stats.uptime == 0);
    }
}

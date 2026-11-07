//! buildah model — daemonless image builder.

#[derive(Debug, Clone)]
pub struct Image {
    pub name: String,
    pub tag: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
    pub containers: Vec<Container>,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![], containers: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn from(&mut self, image: &str, name: &str) -> bool {
        self.containers.push(Container { name: name.into(), image: image.into(), running: false });
        true
    }

    pub fn commit(&mut self, container: &str, image: &str) -> bool {
        if self.containers.iter().any(|c| c.name == container) {
            self.images.push(Image { name: image.into(), tag: "latest".into(), id: format!("{:x}", self.images.len() + 1) });
            true
        } else { false }
    }

    pub fn list_images(&self) -> &[Image] { &self.images }
    pub fn list_containers(&self) -> &[Container] { &self.containers }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_commit() {
        let mut b = Model::new();
        assert!(b.from("alpine", "mycontainer"));
        assert!(b.commit("mycontainer", "myimage"));
        assert_eq!(b.list_images().len(), 1);
    }
}

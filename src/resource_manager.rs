use std::collections::HashMap;

use nwg::Icon;

pub struct Resources {
    resource_paths: HashMap<ResourceType, String>
}

#[derive(Eq, PartialEq, Hash)]
pub enum ResourceType {
    ArrowLeft,
    ArrowRight,
    Refresh
}

impl Resources {
    //TODO: add caching later on to only need whats necessary at first
    pub fn new() -> Resources {
        let mut data = HashMap::new();

        data.insert(ResourceType::ArrowLeft, "./assets/arrow-left.ico".into());
        data.insert(ResourceType::ArrowRight, "./assets/arrow-right.ico".into());
        data.insert(ResourceType::Refresh, "./assets/refresh.ico".into());

        Resources { resource_paths: data }
    }

    pub fn get_icon(&self, t: ResourceType) -> Result<Icon, nwg::NwgError> {
        let path = self.resource_paths.get(&t).unwrap();
        Icon::from_file(path, false)
    }

}

impl Default for Resources {
    fn default() -> Self {
        Resources::new()
    }
}
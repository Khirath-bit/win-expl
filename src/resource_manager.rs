use std::collections::HashMap;

use nwg::Bitmap;

pub struct Resources {
    resource_paths: HashMap<ResourceType, String>
}

#[derive(Eq, PartialEq, Hash)]
pub enum ResourceType {
    ArrowLeft,
    ArrowRight,
    Refresh,
    Copy,
}

impl Resources {
    //TODO: add caching later on to only need whats necessary at first
    pub fn new() -> Resources {
        let mut data = HashMap::new();

        data.insert(ResourceType::ArrowLeft, "./assets/arrow-left.png".into());
        data.insert(ResourceType::ArrowRight, "./assets/arrow-right.png".into());
        data.insert(ResourceType::Refresh, "./assets/refresh.png".into());
        data.insert(ResourceType::Copy, "./assets/copy.png".into());

        Resources {
            resource_paths: data
        }
    }

    pub fn get_bitmap(&self, t: ResourceType) -> Result<Bitmap, nwg::NwgError> {
        let path = self.resource_paths.get(&t).unwrap();
        Bitmap::from_file(path, false)
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources::new()
    }
}

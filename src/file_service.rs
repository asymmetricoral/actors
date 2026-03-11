use crate::world::World;
use std::path::PathBuf;

pub struct FileService {}

impl FileService {
    pub fn from_file(filepath: &str) -> World {
        let bytes = std::fs::read(filepath).unwrap();
        postcard::from_bytes(&bytes).unwrap()
    }

    pub fn to_file(world: &World, filename: &str) {
        let bytes = postcard::to_allocvec(world).unwrap();

        std::fs::create_dir_all("./resources").unwrap();
        
        let mut path = PathBuf::from("./resources");
        path.push(filename);
        path.set_extension("pulsar");

        std::fs::write(path, bytes).unwrap();
    }
}
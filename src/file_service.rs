use crate::world::World;
use std::{error::Error, path::PathBuf};

pub struct FileService {}

impl FileService {
    pub fn from_file(filepath: &str) -> Result<World, postcard::Error> {
        let bytes = std::fs::read(filepath).unwrap_or_else(|_| {
            println!("Save file not found, creating a new world...");
            postcard::to_allocvec(&World::new(10)).unwrap()
        });
        postcard::from_bytes(&bytes)
    }

    pub fn to_file(world: &World, filename: &str) -> Result<(), Box<dyn Error>> {
        let bytes = postcard::to_allocvec(world)?;

        std::fs::create_dir_all("./resources")?;

        let mut path = PathBuf::from("./resources");
        path.push(filename);
        path.set_extension("pulsar");

        std::fs::write(path, bytes)?;
        Ok(())
    }
}
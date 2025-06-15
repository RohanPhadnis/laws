use std::io::Write;
use crate::*;

struct File {
    name: String,
    path: String,
}

impl File {
    fn new(name: String, index: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let p = format!("data/file_{}.bytes", index);
        std::fs::File::create(&p)?;
        Ok(Self {
            name,
            path: p,
        })
    }
    fn read(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut f = std::fs::File::open(&self.path)?;
        let mut output = Vec::new();
        f.read_to_end(&mut output)?;
        Ok(output)
    }
    fn write(&self, buff: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(&self.path, buff)?;
        Ok(())
    }
    fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }
}

pub struct Storage {
    buckets: HashSet<Arc<RwLock<BTreeSet<Arc<RwLock<File>>>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            buckets: HashSet::new(),
        }
    }

    // bucket crud
    fn create_bucket(&mut self, bucket_name: &String) {}
    fn read_bucket(&self, bucket_name: &String) {}
    fn delete_bucket(&mut self, bucket_name: &String) {}


    // file crud
    fn read_file(&self, bucket_name: &String, file_name: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {todo!();}
    fn write_file(&self, bucket_name: &String, file_name: &String, buff: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {todo!();}
    fn delete_file(&self, bucket_name: &String, file_name: &String) -> Result<(), Box<dyn std::error::Error>> {todo!();}
}

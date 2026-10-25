pub trait VirtualFileSystem {
    pub fn limits(&self);
    pub fn read(&self, path: &str) -> Future<Result<Vec<u8>>>;
    pub fn write(&self, path: &str, data: &[u8]) -> Future<Result<()>>;
    pub fn list(&self, path: &str) -> Future<Result<Vec<str>>>;
    pub fn exists(&self, path: &str) -> Future<bool>;
    pub fn remove(&self, path: &str) -> Future<Result<()>>;
    pub fn create(&self, path: &str) -> Future<Result<()>>;
}

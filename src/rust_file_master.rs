pub struct FileOps{
    pub file_name: String,
    pub file_path: String,
}

impl FileOps {
    pub fn new(file_name: String, file_path: String) -> Self {
        FileOps { file_name, file_path }
    }

    pub fn create_file(&self) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        std::fs::File::create(path)?;
        Ok(())
    }

    pub fn write_file(&self, content: &str) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        std::fs::write(path, content)?;
        Ok(())
    }
    pub fn read_file(&self) -> std::io::Result<String> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }
}
#[derive(Debug)]
#[warn(dead_code)]
pub struct RustFileMasterError {
    pub message: String,
}

impl RustFileMasterError {
    pub fn new(message: String) -> Self {
        RustFileMasterError { message }
    }
}

impl std::fmt::Display for RustFileMasterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileOpsError: {}", self.message)
    }
}

impl std::error::Error for RustFileMasterError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub struct  Compression{
    pub file_name: String,
    pub file_path: String,
}
impl Compression{
    pub fn new(file_name: String, file_path: String) -> Self {
        Compression { file_name, file_path }
    }

    pub fn compress_file(&self) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let compressed_path = format!("{}.gz", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::default());
        std::io::copy(&mut input, &mut encoder)?;
        encoder.finish()?;
        Ok(())
    }
    pub fn decompress_file(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}/{}.gz", self.file_path, self.file_name);
        let decompressed_path = format!("{}/{}", self.file_path, self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut output = std::fs::File::create(decompressed_path)?;
        let mut decoder = flate2::read::GzDecoder::new(input);
        std::io::copy(&mut decoder, &mut output)?;
        Ok(())
    }
}

pub struct Encryption{
    pub file_name: String,
    pub file_path: String,
}

impl Encryption{
    pub fn new(file_name: String, file_path: String) -> Self {
        Encryption { file_name, file_path }
    }

    pub fn generate_key() -> Vec<u8> {
        let mut key = [0u8; 16];
        getrandom::getrandom(&mut key).unwrap();
        key.to_vec()
    }

    pub fn encrypt_file(&self, key: &[u8]) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let content = std::fs::read(path)?;
        let encrypted_content = aes_encrypt(&content, key)?;
        let encrypted_path = format!("{}.enc", path);
        std::fs::write(encrypted_path, encrypted_content)?;
        Ok(())
    }
    pub fn decrypt_file(&self, key: &[u8]) -> std::io::Result<()> {
        let encrypted_path = format!("{}/{}.enc", self.file_path, self.file_name);
        let encrypted_content = std::fs::read(encrypted_path)?;
        let decrypted_content = aes_decrypt(&encrypted_content, key)?;
        let decrypted_path = format!("{}/{}", self.file_path, self.file_name);
        std::fs::write(decrypted_path, decrypted_content)?;
        Ok(())
    }
}

pub struct Metadata{
    pub file_name: String,
    pub file_path: String,
}
impl Metadata{
    pub fn new(file_name: String, file_path: String) -> Self {
        Metadata { file_name, file_path }
    }

    pub fn read_metadata(&self) -> std::io::Result<std::fs::Metadata> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let metadata = std::fs::metadata(path)?;
        Ok(metadata)
    }
    pub fn write_metadata(&self, metadata: &std::fs::Metadata) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let mut file = std::fs::OpenOptions::new().write(true).open(path)?;
        file.set_len(metadata.len())?;
        Ok(())
    }
}

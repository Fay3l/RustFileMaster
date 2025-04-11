use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit}, Aes128};
use zip::write::SimpleFileOptions;

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

pub enum Format{
    Gzip,
    Zip,
    Tar
}

pub struct  Compression{
    pub file_name: String,
    pub file_path: String,
}
impl Compression{
    pub fn new(file_name: String, file_path: String) -> Self {
        Compression { file_name, file_path }
    }

    pub fn compress_file(&self,format:Format) -> std::io::Result<()> {
        match format {
            Format::Gzip => self.compress_gzip()?,
            Format::Zip => self.compress_zip()?,
            Format::Tar => self.compress_tar()?,
        }
        Ok(())
    }
    pub fn decompress_file(&self,format:Format) -> std::io::Result<()> {
        match format {
            Format::Gzip => self.decompress_gzip()?,
            Format::Zip => self.decompress_zip()?,
            Format::Tar => self.decompress_tar()?,
        }
        Ok(())
    }
    fn compress_gzip(&self) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let compressed_path = format!("{}.gzip", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::default());
        std::io::copy(&mut input, &mut encoder)?;
        encoder.finish()?;
        Ok(())
    }
    fn compress_zip(&self) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let compressed_path = format!("{}.zip", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut zip = zip::ZipWriter::new(output);
        let option = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file(self.file_name.as_str(), option)?;
        std::io::copy(&mut input, &mut zip)?;
        zip.finish()?;
        Ok(())
    }
    fn compress_tar(&self) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let compressed_path = format!("{}.tar", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut tar = tar::Builder::new(output);
        tar.append_file(self.file_name.clone(), &mut input)?;
        tar.finish()?;
        Ok(())
    }
    fn decompress_gzip(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}/{}.gzip", self.file_path, self.file_name);
        let decompressed_path = format!("{}/{}", self.file_path, self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut output = std::fs::File::create(decompressed_path)?;
        let mut decoder = flate2::read::GzDecoder::new(input);
        std::io::copy(&mut decoder, &mut output)?;
        Ok(())
    }
    fn decompress_zip(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}/{}.zip", self.file_path, self.file_name);
        let decompressed_path = format!("{}/{}", self.file_path, self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut archive = zip::ZipArchive::new(input)?;
        archive.extract(decompressed_path)?;
        Ok(())
    }
    fn decompress_tar(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}/{}.tar", self.file_path, self.file_name);
        let decompressed_path = format!("{}/{}", self.file_path, self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut archive = tar::Archive::new(input);
        archive.unpack(decompressed_path)?;
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

    pub fn generate_key_aes() -> Aes128 {
        let key = [0u8; 16];
        let cipher = Aes128::new(&key.into());
        cipher
    }
    
    pub fn encrypt_file(&self, key: Aes128) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let content = std::fs::read(path.clone())?;
        let mut buffer = GenericArray::clone_from_slice(&content[..16]);
        key.encrypt_block(&mut buffer);
        std::fs::write(path, buffer)?;
        Ok(())
    }

    pub fn decrypt_file(&self, key: Aes128) -> std::io::Result<()> {
        let path = format!("{}/{}", self.file_path, self.file_name);
        let content = std::fs::read(path.clone())?;
        let mut buffer = GenericArray::clone_from_slice(&content[..16]);
        key.decrypt_block(&mut buffer);
        std::fs::write(path, buffer)?;
        Ok(())
    }
    pub fn aes_encrypt(data: &[u8], key: Aes128) -> std::io::Result<()> {
        let mut buffer = GenericArray::clone_from_slice(data);
        key.encrypt_block(&mut buffer);
        Ok(())
    }
    pub fn aes_decrypt(data: &[u8], key: Aes128) -> std::io::Result<()> {
        let mut buffer = GenericArray::clone_from_slice(data);
        key.decrypt_block(&mut buffer);
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
        let file = std::fs::OpenOptions::new().write(true).open(path)?;
        file.set_len(metadata.len())?;
        Ok(())
    }
}

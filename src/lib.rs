use aes_gcm::Aes256Gcm;
use base64::Engine;
use chacha20poly1305::{aead::{generic_array::{typenum::{UInt, UTerm}, GenericArray}, Aead, OsRng}, consts::{B0, B1}, AeadCore, KeyInit, XChaCha20Poly1305};
use ring::rsa;
use zip::write::SimpleFileOptions;


pub struct FileOps{
    pub file_name: String,
}

impl FileOps {
    pub fn new(file_name: String) -> Self {
        FileOps { file_name }
    }

    pub fn create_file(&self) -> std::io::Result<()> {
        let path = format!("{}", self.file_name);
        std::fs::File::create(path)?;
        Ok(())
    }

    pub fn write_file(&self, content: &str) -> std::io::Result<()> {
        let path = format!("{}", self.file_name);
        std::fs::write(path, content)?;
        Ok(())
    }
    pub fn read_file(&self) -> std::io::Result<String> {
        let path = format!("{}", self.file_name);
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
}
impl Compression{
    pub fn new(file_name: String) -> Self {
        Compression { file_name }
    }

    pub fn compress_gzip(&self) -> std::io::Result<()> {
        let path = format!("{}", self.file_name);
        let compressed_path = format!("{}.gzip", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::default());
        std::io::copy(&mut input, &mut encoder)?;
        encoder.finish()?;
        Ok(())
    }
    pub fn compress_zip(&self) -> std::io::Result<()> {
        let path = format!("{}",  self.file_name);
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
    pub fn compress_tar(&self) -> std::io::Result<()> {
        let path = format!("{}", self.file_name);
        let compressed_path = format!("{}.tar", path);
        let mut input = std::fs::File::open(path)?;
        let output = std::fs::File::create(compressed_path)?;
        let mut tar = tar::Builder::new(output);
        tar.append_file(self.file_name.clone(), &mut input)?;
        tar.finish()?;
        Ok(())
    }
    pub fn decompress_gzip(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}.gzip",  self.file_name);
        let decompressed_path = format!("{}",  self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut output = std::fs::File::create(decompressed_path)?;
        let mut decoder = flate2::read::GzDecoder::new(input);
        std::io::copy(&mut decoder, &mut output)?;
        Ok(())
    }
    pub fn decompress_zip(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}.zip",  self.file_name);
        let decompressed_path = format!("{}",  self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut archive = zip::ZipArchive::new(input)?;
        archive.extract(decompressed_path)?;
        Ok(())
    }
    pub fn decompress_tar(&self) -> std::io::Result<()> {
        let compressed_path = format!("{}.tar",  self.file_name);
        let decompressed_path = format!("{}",  self.file_name);
        let input = std::fs::File::open(compressed_path)?;
        let mut archive = tar::Archive::new(input);
        archive.unpack(decompressed_path)?;
        Ok(())
    }
    
}

pub enum EncryptionAlgorithm{
    Aes256Gcm,
    ChaCha20Poly1305,
    Rsa,
}

pub struct Encryption{
    pub file_name: String,
    cipher_chacha: Option<XChaCha20Poly1305>,
    cipher_aes: Option<Aes256Gcm>,
    rsa_public_key: Option<rsa::PublicKey>,
    rsa_private_key: Option<ring::rsa::KeyPair>,
    pub key: Vec<u8>,
    nonce: Option<GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>>>
}

impl Encryption{
    pub fn new(file_name: String) -> Self {
        Encryption { 
            cipher_chacha: None,
            cipher_aes: None,
            rsa_public_key: None,
            rsa_private_key: None,
            file_name,
            key: vec![],
            nonce: None,
        }
    }

    pub fn generate_key(&mut self) -> Vec<u8> {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        self.cipher_chacha = Some(XChaCha20Poly1305::new(key.as_slice().into()));
        self.nonce = Some(XChaCha20Poly1305::generate_nonce(&mut OsRng));
        self.key = key.to_vec();
        key.to_vec()
    }
    
    pub fn encrypt_file(&self) -> std::io::Result<()> {
        let path = format!("{}" ,self.file_name);
        let content = std::fs::read(path.clone())?;
        let encryptext = self.cipher_chacha.encrypt(&self.nonce, content.as_ref()).expect("encryption failure!");
        std::fs::write(path, encryptext)?;
        Ok(())
    }

    pub fn decrypt_file(&self) -> std::io::Result<()> {
        let path = format!("{}" ,self.file_name);
        let content = std::fs::read(path.clone())?; 
        let plaintext = self.cipher_chacha.decrypt(&self.nonce, content.as_ref()).expect("decryption failure!");
        let plaintext_string = String::from_utf8(plaintext).expect("Failed to convert Vec<u8> to String");
        let decrypted_path = format!("{}_decrypted", self.file_name);
        std::fs::write(decrypted_path, plaintext_string)?;
        Ok(())
    }

    pub fn save_key_to_file(&self, key_path: &str) -> std::io::Result<()> {
        // Encrypt the key before saving (optional, for added security)
        let encrypted_key = base64::prelude::BASE64_STANDARD.encode(&self.key); // Example: Base64 encoding
        std::fs::write(key_path, encrypted_key)?;
        Ok(())
    }

    pub fn load_key_from_file(&mut self, key_path: &str) -> std::io::Result<()> {
        let encrypted_key = std::fs::read_to_string(key_path)?;
        let key = base64::prelude::BASE64_STANDARD.decode(encrypted_key).expect("Failed to decode key");
        self.cipher_chacha = Some(XChaCha20Poly1305::new(GenericArray::from_slice(&key)));
        self.key = key;
        Ok(())
    }

    pub fn generate_and_save_key(&mut self, key_path: &str) -> std::io::Result<()> {
        let key = self.generate_key();
        self.save_key_to_file(key_path)?;
        Ok(())
    }
    
    pub fn rotate_key(&mut self, key_path: &str) -> std::io::Result<()> {
        self.generate_and_save_key(key_path)?;
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

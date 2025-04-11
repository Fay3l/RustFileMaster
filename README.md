
# RustFileMaster 📄

**RustFileMaster** is a Rust library designed to simplify and optimize file and filesystem manipulation. It offers advanced features such as compression, encryption, and metadata management, while being performant and secure.

## Key Features

### 📄 File Manipulation

- **Creation, Reading, Writing, Deletion**: Basic functions to create, read, write, and delete files.
- **Copying and Moving**: Functions to copy and move files and directories.
- **Permissions**: Manage file permissions (read, write, execute).

### 📄 Compression

- **Supported Formats**: Support for common compression formats like ZIP, GZIP, and TAR.
- **Compression and Decompression**: Functions to compress and decompress files and directories.
- **Incremental Compression**: Support for incremental compression to minimize disk space usage.

### 📄 Encryption

- **Supported Algorithms**: Support for common encryption algorithms like AES, RSA, and ChaCha20.
- **Encryption and Decryption**: Functions to encrypt and decrypt files.
- **Key Management**: Tools to securely generate, store, and manage encryption keys.

### 📄 Metadata Management

- **Reading and Writing**: Functions to read and write file metadata (author, creation date, etc.).
- **Supported Formats**: Support for common metadata formats like EXIF, ID3, and XMP.
- **Indexing**: Tools to index metadata for easier file search and retrieval.

### 📄 Performance and Security

- **Optimization**: Use of optimization techniques to improve file operation performance.
- **Security**: Implementation of best security practices to protect files from unauthorized access.

## Architecture

### Modules

- **file_ops**: Module for basic file operations.
- **compression**: Module for compression and decompression functionalities.
- **encryption**: Module for encryption and decryption functionalities.
- **metadata**: Module for metadata management.
- **utils**: Module for utilities and helper functions.

## Dependencies

- **serde**: For serialization and deserialization of metadata.
- **flate2**: For GZIP compression and decompression.
- **zip**: For ZIP file management.
- **aes**: For AES encryption.
- **rsa**: For RSA encryption.

## Usage Examples

```rust
use rust_file_master::file_ops;
use rust_file_master::compression;
use rust_file_master::encryption;
use rust_file_master::metadata;

fn main() {
    // Create a file
    file_ops::create_file("example.txt").expect("Failed to create file");

    // Write to a file
    file_ops::write_file("example.txt", "Hello, world!").expect("Failed to write to file");

    // Read a file
    let content = file_ops::read_file("example.txt").expect("Failed to read file");
    println!("File content: {}", content);

    // Compress a file
    compression::compress_file("example.txt", "example.zip").expect("Failed to compress file");

    // Decompress a file
    compression::decompress_file("example.zip", "example_decompressed.txt").expect("Failed to decompress file");

    // Encrypt a file
    let key = encryption::generate_key();
    encryption::encrypt_file("example.txt", "example.enc", &key).expect("Failed to encrypt file");

    // Decrypt a file
    encryption::decrypt_file("example.enc", "example_decrypted.txt", &key).expect("Failed to decrypt file");

    // Read file metadata
    let metadata = metadata::read_metadata("example.txt").expect("Failed to read metadata");
    println!("File metadata: {:?}", metadata);

    // Write file metadata
    metadata::write_metadata("example.txt", metadata).expect("Failed to write metadata");
}
```

## Contributions

- **Documentation**: Comprehensive documentation with usage examples and detailed explanations of features.
- **Tests**: Extensive test suite to ensure the reliability and robustness of the library.
- **Contributions**: Contribution guide to encourage community contributions.

## Roadmap

- **Version 1.0**: Implementation of basic features and main modules.
- **Version 1.1**: Addition of advanced features like incremental compression and encryption key management.
- **Version 1.2**: Performance optimization and security improvements.
- **Version 2.0**: Addition of new features based on community feedback.

## Conclusion

RustFileMaster aims to become the go-to library for file manipulation in Rust, offering advanced features and great flexibility. It is designed to be performant, secure, and easy to use, while being extensible to meet future community needs.

**License**

Licensed under either of [MIT License](LICENSE) at your option.

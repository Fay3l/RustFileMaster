use rust_file_master::Compression;




fn main() -> anyhow::Result<()> {
    let file_name = "./test/example2.txt".to_string();
    let compression = Compression::new(file_name.clone());
    compression.decompress_zip()?;

    Ok(())
}

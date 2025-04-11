use rust_file_master::Compression;




fn main() -> anyhow::Result<()> {
    let file_name = "./test/example2.txt".to_string();7
    let file_ops = rust_file_master::FileOps::new(file_name.clone());
    file_ops.create_file()?;
    file_ops.write_file("Hello, world!")?;
    let compression = Compression::new(file_name.clone());
    compression.
    Ok(())
}

use rust_file_master::FileOps;


fn main() -> anyhow::Result<()> {
    let file_name = "./test/example.txt".to_string();
    let file_ops = FileOps::new(file_name.clone());
    file_ops.write_file("Hello, world!")?;
    let content = file_ops.read_file()?;
    println!("File content: {}", content);
    Ok(())
}

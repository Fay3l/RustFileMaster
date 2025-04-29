use rust_file_master::Encryption;

fn main() -> anyhow::Result<()> {
    let file_name = "./test/example3.txt".to_string();
    let mut encryption = Encryption::new(file_name.clone(),);
    encryption.generate_key();
    encryption.encrypt_file()?;
    println!("File encrypted successfully.");
    encryption.decrypt_file()?;
    println!("File decrypted successfully.");
    Ok(())
}

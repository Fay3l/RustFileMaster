use rust_file_master::Encryption;

fn main() -> anyhow::Result<()> {
    let file_name = "./test/example3.txt".to_string();
    let mut encryption = Encryption::new(file_name.clone());
    let key  = encryption.generate_key();
    
    encryption.encrypt_file()?;
    println!("File encrypted successfully.");

    let mut encryption2 = Encryption::new(file_name.clone());
    encryption2.key = key;
    encryption2.decrypt_file()?;
    println!("File decrypted successfully.");
    
    Ok(())
}

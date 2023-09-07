use std::fs::File;
use std::io::Read;
use std::io::Write;

fn save_vec_to_file(data: &Vec<u8>, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;

    Ok(())
}
fn xor_encrypt(data: &Vec<u8>, key: u8) -> Vec<u8> {
    data.iter().map(|byte| byte ^ key).collect()
}

fn xor_decrypt(encrypted_data: &Vec<u8>, key: u8) -> Vec<u8> {
    xor_encrypt(encrypted_data, key) // XORing twice with the same key decrypts the data
}

fn read_file_to_vec(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "file.txt";
    let key: u8 = 0x5A; // Replace with your secret key

    // Read the file content into a Vec<u8>
    let file_content = read_file_to_vec(file_path)?;

    // Encrypt the data
    let encrypted_data = xor_encrypt(&file_content, key);

    // Save the encrypted data to a file
    save_vec_to_file(&encrypted_data, "encrypted_file.txt")?;

    // Decrypt the data (optional)
    let decrypted_data = xor_decrypt(&encrypted_data, key);
    save_vec_to_file(&decrypted_data, "decrypted_file.txt")?;

    Ok(())
}

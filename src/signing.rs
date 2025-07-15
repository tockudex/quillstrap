use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use anyhow::{Result, Context};
use log::{info, warn, error};
use std::fs;

const GENERIC_DIGEST_EXT: &str = ".dgst";

pub fn sign_file(private_key_path: &str, file_path: &str) -> Result<()> {
    info!("Signing file '{}'", &file_path);
    let file_bytes = fs::read(&file_path)?;
    let private_key_bytes = fs::read(&private_key_path)?;

    let private_key = PKey::private_key_from_pem(&private_key_bytes)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
    signer.update(&file_bytes)?;
    
    let signature = signer.sign_to_vec()?;
    fs::write(&format!("{}{}", &file_path, &GENERIC_DIGEST_EXT), &signature)?;

    Ok(())
}

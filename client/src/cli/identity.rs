#![allow(dead_code)]

use clap::Error;

pub fn generate_identity_keypair() -> Result<(), Error> {
    let mut rand_generator = rand::rngs::OsRng {};
    let identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let encoded_private_key = base64::encode(identity_keypair.secret.to_bytes());
    println!("private Key: {} ", encoded_private_key);

    let encoded_public_key = base64::encode(identity_keypair.public.to_bytes());
    println!("public key: {}", encoded_public_key);

    Ok(())
}

/// Run 
pub fn run() -> Result<(), Error> {
    let mut rand_generator = rand::rngs::OsRng {};
    let identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let encoded_private_key = base64::encode(identity_keypair.secret.to_bytes());
    println!("private Key: {} ", encoded_private_key);

    let encoded_public_key = base64::encode(identity_keypair.public.to_bytes());
    println!("public key: {}", encoded_public_key);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Key pair should not be equal???
    fn test_generate_identity() {
        let keypair_1 = run().unwrap();
        let keypair_2 = generate_identity_keypair().unwrap();

        assert_eq!(keypair_1, keypair_2, "keypairs are not equal");
    }
}
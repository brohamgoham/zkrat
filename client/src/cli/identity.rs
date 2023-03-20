pub fn run() {
    let mut rand_generator = rand::rngs::OsRnd {};
    let identity_keypair = ed25519_dalek::KeyPair::generate(&mut rand_generator);

    let encoded_private_key = base64::encode(identity_keypair.secret.to_bytes())
    println!("private Key: {} ", encoded_private_key);

    let encoded_public_key = base64::encode(identity_keypair.public.to_bytes());
    println!("public key: {}", encoded_public_key);
}
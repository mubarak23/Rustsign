use rand::rngs::OsRng;
use rand::RngCore;
use anyhow::Result;
use ed25519_dalek::{Signer, Signature, Verifier, SigningKey, VerifyingKey};


pub fn generate_keypair() -> Result<(SigningKey, VerifyingKey)> {

  // generate random 32 bytes that can be used ad secret key
  let mut secret_key_bytes = [0u8; 32];
   OsRng.fill_bytes(&mut secret_key_bytes);

  // private key for signing key 
  let signing_key = SigningKey::from_bytes(&mut secret_key_bytes);
  
  // public key for verifying message;
  let verifying_key = signing_key.verifying_key();

  Ok((signing_key, verifying_key))
}


// sign a message with private key
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Result<Signature> {
  let signature = signing_key.sign(message);
  // Return a signature
  Ok(Signature::from_bytes(&signature.to_bytes()))
}

pub fn verify_signature(verifying_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
  verifying_key.verify(message, signature).is_ok()
}



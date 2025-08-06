use crate::crypto::*;
use crate::utils::*;
use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};

/// CLI for signing and verifying messages
#[derive(Parser)]
#[command(name = "signed-msg")]
#[command(about = "A simple message signing tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate,
    Sign { message: String },
    Verify { message: String, signature: String },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Generate => {
            let (sk, vk) = generate_keypair()?;
            write_file("keys/private.key", sk.to_bytes().as_ref())?;
            write_file("keys/public.key", vk.to_bytes().as_ref())?;
            println!("✅ Keypair generated and saved to 'keys/'");
        }

        Commands::Sign { message } => {
            let sk_bytes = read_file("keys/private.key")?;
            let sk_array: [u8; 32] = sk_bytes
                .try_into()
                .map_err(|_| anyhow!("❌ Public key must be exactly 32 bytes"))?;
            let sk = SigningKey::from_bytes(&sk_array);
            let sig = sign_message(&sk, message.as_bytes());
            println!(
                "✍️ Signature (base64): {}",
                encode_b64(sig?.to_bytes().as_ref())
            );
        }
        Commands::Verify { message, signature } => {
            let vk_bytes = read_file("keys/public.key")?;
            let vk_array: [u8; 32] = vk_bytes
                .try_into()
                .map_err(|_| anyhow!(" Public key must be exactly 32 bytes"))?;
            let vk = VerifyingKey::from_bytes(&vk_array)?;

            let sig_bytes = decode_b64(&signature)?;
            let sig_array: [u8; 64] = sig_bytes
                .try_into()
                .map_err(|_| anyhow!(" Signature must be exactly 64 bytes"))?;
            let sig = Signature::from_bytes(&sig_array);

            let valid = verify_signature(&vk, message.as_bytes(), &sig);
            if valid {
                println!("✅ Signature is valid.");
            } else {
                println!("❌ Signature is INVALID!");
            }
        }
    }

    Ok(())
}

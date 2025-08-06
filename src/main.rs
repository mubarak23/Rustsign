// use anyhow::Result;
// mod crypto;

// fn main() -> Result<()> {
//     let (signing_key, verifying_key)  = crypto::generate_keypair()?;
//     println!("Private key: {:?}", &signing_key);
//     println!("Public key: {:?}", &verifying_key);

//     // sign a message
//     let message = "Test message";
//     let signature = crypto::sign_message(&signing_key, &message.as_bytes())?;
//      println!("Signature of a message: {:?}", &signature);

//      // verify sign message
//      let is_verified = crypto::verify_signature(&verifying_key, &message.as_bytes(), &signature);
//      println!("Is this a verified message: {:?}", &is_verified);
//     Ok(())
// }

mod cmd;
mod crypto;
mod utils;

use clap::Parser;
use cmd::{Cli, run};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}

use libp2p::{identity, PeerId};
use std::error::Error;

pub fn create_p2p() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    Ok(())
}
use std::time::Duration;

use libp2p::{gossipsub, identify, ping, rendezvous};
use libp2p::swarm::NetworkBehaviour;
use libp2p_gossipsub::Topic;
use tracing::info;
use crate::peer::behaviour::base::create_gossip_sub_config;
use crate::peer::behaviour::PeerBehaviour;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct PeerAdminBehaviour {
    pub rendezvous: rendezvous::server::Behaviour,
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
    pub gossip_sub: gossipsub::Behaviour,
}

impl PeerBehaviour for PeerAdminBehaviour {
    fn new(local_public_key: libp2p::identity::Keypair) -> Self {
        let rendezvous_server = rendezvous::server::Behaviour::new(rendezvous::server::Config::default());
        let gossip_sub_config = create_gossip_sub_config();
        let gossip_sub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_public_key.clone()),
            gossip_sub_config,
        ).unwrap();

        Self {
            gossip_sub,
            rendezvous: rendezvous_server,
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(10))),
            identify: identify::Behaviour::new(identify::Config::new(
                "/CEYLON-AI-IDENTITY/0.0.1".to_string(),
                local_public_key.public(),
            )),
        }
    }
}

impl PeerAdminBehaviour {
    pub fn process_event(&mut self, event: PeerAdminBehaviourEvent) {
        match event {
            PeerAdminBehaviourEvent::Rendezvous(event) => {
                match event {
                    rendezvous::server::Event::PeerRegistered { peer, .. } => {
                        info!( "RendezvousServerConnected: {:?}", peer);

                        let topic = gossipsub::IdentTopic::new("test_topic");
                        self.gossip_sub.subscribe(&topic).unwrap();
                    }
                    _ => {
                        info!( "RendezvousServer: {:?}", event);
                    }
                }
            }
            PeerAdminBehaviourEvent::Ping(event) => {
                // info!( "Ping: {:?}", event);
            }
            PeerAdminBehaviourEvent::Identify(event) => {
                // info!( "Identify: {:?}", event);
            }

            PeerAdminBehaviourEvent::GossipSub(event) => {
                info!( "GossipSub: {:?}", event);
            }
        }
    }
} 
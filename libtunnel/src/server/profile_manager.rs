use super::profile::ClientProfile;
use crate::{str_hash, PAYLOAD_SIZE};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::collections::HashMap;

pub type Hash = [u8; PAYLOAD_SIZE];
pub struct ProfileManager {
    profiles: HashMap<Hash, ClientProfile>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn register(&mut self, profile: ClientProfile) -> Result<(), Error> {
        let hash: Hash = str_hash(&profile.id);

        if self.profiles.contains_key(&hash) {
            return Err(format!("Profile '{}' is already registered", &profile.id))
                .handle_err(location!());
        }

        let _ = self.profiles.insert(hash, profile);

        Ok(())
    }

    pub fn remove(&mut self, id: &Hash) -> Result<(), Error> {
        if !self.profiles.contains_key(id) {
            return Err(format!("Profile with id '{:?}' is not registered", id))
                .handle_err(location!());
        }

        let _ = self.profiles.remove(id);

        Ok(())
    }

    pub fn get(&self, hash: &Hash) -> Option<ClientProfile> {
        self.profiles.get(hash).cloned()
    }
}

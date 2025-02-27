use super::profile::ClientProfile;
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::collections::HashMap;

pub struct ProfileManager {
    profiles: HashMap<String, ClientProfile>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn register(&mut self, profile: ClientProfile) -> Result<(), Error> {
        if self.profiles.contains_key(&profile.id) {
            return Err(format!("Profile '{}' is already registered", &profile.id))
                .handle_err(location!());
        }

        let _ = self.profiles.insert(profile.id.clone(), profile);

        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<(), Error> {
        if !self.profiles.contains_key(id) {
            return Err(format!("Profile with id '{}' is not registered", id))
                .handle_err(location!());
        }

        let _ = self.profiles.remove(id);

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<ClientProfile> {
        self.profiles.get(id).cloned()
    }
}

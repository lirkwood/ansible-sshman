use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::Hash,
    iter::FromIterator,
};

use crate::{inventory::Inventory, model::SSHPlay};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Sudoer,
    SuperUser,
}

/// Models a user in the config file.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct SSHUser {
    pub name: String,
    pub pubkeys: Vec<String>,
    pub access: String,
    pub role: Role,
}

/// Models a config file.
pub struct SSHConfig {
    /// The users defined in the config file.
    pub users: HashMap<String, SSHUser>,
}

impl SSHConfig {
    /// Parses an SSHConfig from some yaml.
    pub fn from_str(content: &str) -> Result<SSHConfig, Box<dyn Error>> {
        let list: Vec<SSHUser> = serde_yaml::from_str(content)?;
        Ok(SSHConfig {
            users: list
                .into_iter()
                .map(|usr| (usr.name.clone(), usr))
                .collect(),
        })
    }

    /// Returns a playbook that will apply this config to a given inventory.
    pub fn apply(&self, inv: &Inventory) -> Result<String, Box<dyn Error>> {
        let mut host_users = HashMap::new();
        for user in self.users.values() {
            for host in inv.get_pattern_hosts(&user.access) {
                if !host_users.contains_key(host) {
                    host_users.insert(host, HashSet::new());
                }
                host_users.get_mut(host).unwrap().insert(user.name.clone());
            }
        }

        let mut user_hosts = HashMap::new();
        for (host, users) in host_users {
            let mut user_hash = Vec::from_iter(users);
            user_hash.sort();
            if !user_hosts.contains_key(&user_hash) {
                user_hosts.insert(user_hash.clone(), Vec::new());
            }
            user_hosts.get_mut(&user_hash).unwrap().push(host);
        }

        let mut plays = Vec::new();
        for (users, hosts) in user_hosts {
            let group = hosts.join(":");
            plays.push(SSHPlay::set_accounts(
                group.clone(),
                users.iter().filter_map(|u| self.users.get(u)).collect(),
            ));
            plays.push(SSHPlay::set_user_pubkeys(
                group.clone(),
                users
                    .into_iter()
                    .map(|name| self.users.get(&name).unwrap())
                    .collect(),
            ));
        }

        Ok(serde_yaml::to_string(&plays)?)
    }
}

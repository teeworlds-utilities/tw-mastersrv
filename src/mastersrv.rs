use serde::Deserialize;

use crate::req::req_json;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerMap {
    pub name: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Client {
    pub name: String,
    pub clan: String,
    pub country: i32,
    pub score: i32,
    pub is_player: bool
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerInfo {
    pub max_clients: i32,
    pub max_players: i32,
    pub passworded: bool,
    pub game_type: String,
    pub name: String,
    pub map: ServerMap,
    pub version: String,
    pub clients: Vec<Client>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub addresses: Vec<String>,
    pub location: Option<String>,
    pub info: ServerInfo,
}

#[derive(Debug, Deserialize)]
pub struct TwMasterServer {
   pub servers: Vec<Server>
}

impl TwMasterServer {
    pub fn new(url: &str) -> Self {
        let res = req_json::<TwMasterServer>(url);

        let servers = match res {
            Some(master) => master.servers,
            None => vec![]
        };

        Self {
            servers
        }
    }

    pub fn update(&mut self, url: &str) -> bool {
        let res = req_json::<TwMasterServer>(&url);

        match res {
            Some(master) => {
                self.servers = master.servers;
                true
            },
            None => false
        }
    }
    
    pub fn find<T>(
        &self,
        f: &dyn Fn(&Server) -> Option<T>
    ) -> Option<Vec<T>> 
    {
        let mut ret = Vec::new();
    
        for server in &self.servers {
            let res = f(server);

            match res {
                Some(value) => ret.push(value),
                None => continue
            };
        }

        if ret.len() == 0 {
            return None;
        }

        Some(ret)
    }

    pub fn find_player(&self, name: &str) -> Option<Vec<(Client, Server)>> {
        self.find(&|server| {
            for client in &server.info.clients {
                if client.name == name {
                    return Some(
                        (client.clone(), server.clone())
                    );
                }
            }

            None
        })
    }

    pub fn search_server_by_name(&self, name: &str) -> Option<Vec<Server>> {
        self.find(&|server| {
            if server.info.name.contains(name) {
                return Some(server.clone())
            }

            None
        })
    }
}

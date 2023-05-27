use serde::Deserialize;

lazy_static::lazy_static! {
    pub static ref CONFIG: ServerConfigRoot =
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
}

#[derive(Deserialize)]
pub struct ServerConfigRoot {
    pub server_info: String,
    pub general: ServerConfigGeneral,
    pub world: ServerConfigWorld,
}

#[derive(Deserialize)]
pub struct ServerConfigGeneral {
    pub view_distance: i32,
    pub gamemode: i32,
}

#[derive(Deserialize)]
pub struct ServerConfigWorld {
    pub seed: i64,
    pub dimension: i8,
    pub difficulty: i8,
}

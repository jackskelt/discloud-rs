#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub user_agent: String,
}

impl Config {
    pub fn new(token: &str, user_agent: &str) -> Config {
        Config {
            token: token.to_owned(),
            user_agent: user_agent.to_owned(),
        }
    }
}

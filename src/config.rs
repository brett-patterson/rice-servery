#[derive(Deserialize, Debug)]
pub struct Config {
    pub rules: Vec<Rule>,
    pub outgoing_email: Option<OutgoingEmail>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub keyword: String,
    pub alert: Option<Alert>,
}

#[derive(Deserialize, Debug)]
pub enum Alert {
    Email(String),
}

#[derive(Deserialize, Debug)]
pub struct OutgoingEmail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub from: (String, String),
}
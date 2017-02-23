/// The top-level configuration structure.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub rules: Vec<Rule>,
    pub outgoing_email: Option<OutgoingEmail>,
}

/// A rule that describes what menu items to match, who to alert when found,
/// and how to alert them.
#[derive(Deserialize, Debug)]
pub struct Rule {
    pub keyword: String,
    pub alert: Option<Alert>,
}

/// The type of alert to send to a user.
#[derive(Deserialize, Debug)]
pub enum Alert {
    Email(String),
}

/// The outgoing email information used to send email alerts.
#[derive(Deserialize, Debug)]
pub struct OutgoingEmail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub from: (String, String),
}
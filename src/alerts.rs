use std::io::{stderr, Write};

use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::SmtpTransportBuilder;

use super::config::{Config, Rule, Alert};

pub fn alert(item: &str, servery: &str, rule: &Rule, config: &Config) {
    let alert_text = format!("Found {} at {}", item, servery);
    
    if let Some(ref alert) = rule.alert {
        match alert {
            &Alert::Email(ref address) => {
                if let Some(ref outgoing) = config.outgoing_email {
                    let email = EmailBuilder::new()
                        .to(address.as_str())
                        .from((outgoing.from.0.as_str(), outgoing.from.1.as_str()))
                        .subject(&alert_text)
                        .build()
                        .unwrap();
                    
                    let result = SmtpTransportBuilder::new((outgoing.host.as_str(), outgoing.port))
                        .map(|m| {
                            m.credentials(&outgoing.username, &outgoing.password)
                            .build()
                        }).and_then(|mut mailer| mailer.send(email));
                    
                    if let Err(e) = result {
                        writeln!(stderr(), "Email alert failed to send: {}", e).unwrap();
                    }
                }
            }
        }
    }
    
    println!("{}", alert_text);
}
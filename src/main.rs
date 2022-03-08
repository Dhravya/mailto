use dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() {
    // Loads dotenv
    dotenv::dotenv().ok();

    // Collects the args as a Vector
    let args: Vec<String> = std::env::args().collect();

    let send_to_email = args[1].clone();

    // Checks the email using regex
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    if !email_regex.is_match(&send_to_email) {
        println!("{} is not a valid email address", send_to_email);
        return;
    }

    if args.len() < 3 {
        println!("Usage: {} <email> <message>", args[0]);
        return;
    }
    let message = args[2..].join(" ");

    // The first 4 words will be the subject
    // TODO: Make this better and more robust
    let subject: String = args[2..6].join(" ");

    // Builds the actual email itself, loads variables from env
    let email = Message::builder()
        .from(env::var("EMAIL").unwrap().parse().unwrap())
        .to(send_to_email.parse().unwrap())
        .subject(subject)
        .body(message)
        .unwrap();

    // Sets the credentials for mailserver
    let creds = Credentials::new(env::var("EMAIL").unwrap(), env::var("PASSWORD").unwrap());

    // The object that actually builds the email
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Sends the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate chrono;
extern crate azul;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransportBuilder;
use lettre::{ClientSecurity, ClientTlsParameters, EmailTransport};
use lettre_email::EmailBuilder;
use native_tls::Protocol;
use native_tls::TlsConnector;
use std::path::Path;
use std::env;
use dotenv::dotenv;
use chrono::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use azul::{prelude::*, widgets::{label::Label, button::Button}};

// ui implementation


struct CounterApplication {
    counter: usize,
}

impl Layout for CounterApplication {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("Emails sent: {}", self.counter)).dom();
        let button = Button::with_label("Click to send email").dom()
            .with_callback(On::MouseDown, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button)
            .with_class("Taskrunner")
    }


}

fn update_counter(app_state: &mut AppState<CounterApplication>, _info: &mut CallbackInfo<CounterApplication>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    send_mail();
    Redraw
}

fn send_mail(){
    dotenv().ok();
    
    let email_username = env::var("EMAIL_USERNAME").unwrap();
    let email_password = env::var("EMAIL_PASSWORD").unwrap();

    let weekday = Local::now().format("%a").to_string();
    let subject_line = ["Tasks for ", &weekday].concat();

    let task_file_path = ["./tasks/", &weekday, ".txt"].concat();
    let task_file_path_mail = task_file_path.clone();
    let mut task_file = File::open(task_file_path).expect("Unable to open the file");
    let mut task_file_contents = String::new();
        task_file.read_to_string(&mut task_file_contents).expect("Unable to read the file");


    let email = EmailBuilder::new()
        .to(("cs.mccaleb@gmail.com", "Colin McCaleb"))
        .from("cs.mccaleb@gmail.com")
        .subject(subject_line)
        .text(task_file_contents)
        .attachment(Path::new(&task_file_path_mail.to_string()), None, &mime::TEXT_PLAIN).unwrap()
        .build()
        .unwrap();

    pub const DEFAULT_TLS_PROT: &[Protocol] = &[Protocol::Tlsv10];

    let mut tls_builder = TlsConnector::builder().unwrap();
    tls_builder.supported_protocols(DEFAULT_TLS_PROT).unwrap();

    let tls_parameters =
        ClientTlsParameters::new("smtp.gmail.com".to_string(), tls_builder.build().unwrap());

    pub const SUBMISSION_PORT: u16 = 465;

    let mut mailer = SmtpTransportBuilder::new(
        ("smtp.gmail.com", SUBMISSION_PORT),
        ClientSecurity::Wrapper(tls_parameters),
    ).expect("Failed to create transport")
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            email_username.to_string(),
            email_password.to_string(),
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();

    let result = mailer.send(&email);

    if result.is_ok() {
        println!("Email sent to recipient");
        println!("{:#?}", weekday);
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}

fn main() {
    let mut app = App::new(CounterApplication { counter: 0 }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}



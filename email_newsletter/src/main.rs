
use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx:: PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
	init_subscriber(subscriber);
	
	// Set up tracing subscriber for structured logging	
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}

use secrecy::ExposeSecret;

use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //logging tha rustl
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    //configuring tha rustl
    let configuration = get_configuration().expect("failed to get config");
    let connection_pool = PgPoolOptions::new().acquire_timeout(std::time::Duration::from_secs(2)).
        connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("failed to connect to postgres");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

use crate::v1::utils::collection::{Collections, GetCollection};
use crate::v1::utils::WeeklyRecipeGetter;
use actix_cors::Cors;
use actix_web::{web, App as ActixApp, HttpServer};
use clap::{App as ClapApp, Arg};
use mongodb::bson::doc;
use std::sync::{Arc, Mutex};

mod envvar;
mod v1;

/// Determines the current environment of the project.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Environment {
    /// The project is running in development mode.
    Dev,
    /// The project is running locally simulating production mode.
    LocalProd,
    /// The project is running in production mode on a server.
    Prod,
}

/// Create the database client connection.
async fn create_db_client(env_file: &str) -> Result<mongodb::Client, String> {
    // Create a new MongoDB client
    let uri = envvar!(MONGODB_URI from env_file)?;

    // Initialise an options.
    let mut client_options = mongodb::options::ClientOptions::parse(&uri)
        .await
        .map_err(|_| format!("Could not create MongoDB client with URI `{}`", uri))?;

    client_options.app_name = dotenv::var("MONGODB_CONNECTION_APPNAME").ok();

    client_options.credential = Some(
        mongodb::options::Credential::builder()
            .username(envvar!(MONGO_INITDB_ROOT_USERNAME from env_file)?)
            .password(envvar!(MONGO_INITDB_ROOT_PASSWORD from env_file)?)
            .build(),
    );

    // Construct a client from those options.
    let client = mongodb::Client::with_options(client_options);
    let client = client.map_err(|_| "Could not create MongoDB client".to_string())?;

    // Create a text index on the recipes collection for the title.
    client
        .get_collection::<crate::v1::types::database::Recipe>(Collections::Recipes)
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "title": "text" })
                .build(),
            None,
        )
        .await
        .map_err(|_| "Could not create text index on recipes collection".to_string())?;

    Ok(client)
}

/// Set the log level of the application using `tracing`.
pub fn set_log_level(env_file: &str) -> Result<(), String> {
    let level = envvar!(LOG_LEVEL from env_file)?;
    let level = level.to_lowercase();
    let level = match level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => return Err(format!("Invalid envvar `LOG_LEVEL` in {}. Expected `TRACE`, `DEBUG`, `INFO`, `WARN`, or `ERROR`", env_file)),
    };

    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| format!("Could not set log level: `{}`", e))?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a command-line parser to get the current environment
    let matches = ClapApp::new("Rust Server")
        .about("The Rust API for the website/app.")
        .arg(Arg::with_name("environment")
            .short('e')
            .long("environment")
            .value_name("ENVIRONMENT")
            .help("The environment to run the server in. Valid modes are: development (dev, d), localproduction (localprod, lprod, lp), production (prod, p)")
            .takes_value(true)
        )
        .get_matches();

    // Default to dev in debug mode, or prod in release mode.
    let env = matches.value_of("environment").unwrap_or({
        #[cfg(debug_assertions)]
        {
            "dev"
        }
        #[cfg(not(debug_assertions))]
        {
            "prod"
        }
    });
    let env = env.to_lowercase();
    // Converts string to environment setting
    let env = match env.as_str() {
        "development" | "dev" | "d" => Environment::Dev,
        "localproduction" | "localprod" | "lprod" | "lp" => Environment::LocalProd,
        "production" | "prod" | "p" => Environment::Prod,
        _ => panic!("Invalid environment: `{}`", env),
    };

    // The environment file to load. Depends on environment setting
    // to load the correct one.
    let env_file = match env {
        Environment::Dev => ".env.dev",
        Environment::LocalProd => ".env.localprod",
        Environment::Prod => ".env.prod",
    };
    dotenv::from_filename(env_file)
        .unwrap_or_else(|e| panic!("Failed loading `{}` file: {}", env_file, e));

    // Load the secret key used for various operations from the secret.env file
    // TODO: Not use a secret-based key to interact with the database!
    dotenv::from_filename("secret.env")
        .unwrap_or_else(|e| panic!("Failed loading `secret.env` file: {}", e));

    // Get the server port, panicking if not set.
    let port = envvar!(SERVER_PORT from env_file).unwrap();
    let port = port.parse::<u16>().unwrap_or_else(|_| {
        panic!(
            "Could not parse `SERVER_PORT={}` in file `{}` as a u16",
            port, env_file
        )
    });

    // Create the tracing subscriber and set the log level for the application.
    set_log_level(env_file).unwrap();

    // Get a connection to the database.
    let client = create_db_client(env_file).await.unwrap();

    // Test the connection to the database
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await
        .expect("Could not ping MongoDB");
    println!("Connected to the database successfully.");

    let weekly_recipe_getter = WeeklyRecipeGetter::default();
    let weekly_recipe_getter = Arc::new(Mutex::new(weekly_recipe_getter));

    // Start the web server
    println!("Starting Actix-web server on http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .max_age(3600);
        let cors = Cors::permissive();

        ActixApp::new()
            .wrap(cors)
            .app_data(web::Data::new(env))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(weekly_recipe_getter.clone()))
            .service(web::scope("/api").service(v1::init(web::scope("/v1"))))
    })
    // Docker requires 0.0.0.0 and i wasted over an hour of my life
    // figuring this out.
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

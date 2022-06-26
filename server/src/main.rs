use actix_web::{web, App as ActixApp, HttpServer};
use clap::{App as ClapApp, Arg};

mod api;

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

macro_rules! envvar {
    ($name:ident from $filename:expr) => {
        dotenv::var(stringify!($name)).map_err(|_| {
            format!(
                "Could not find envvar `{}` in file `{}`",
                stringify!($name),
                $filename
            )
        })
    };
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

    // Get the server port, panicking if not set.
    let port = envvar!(SERVER_PORT from env_file).unwrap();
    let port = port.parse::<u16>().unwrap_or_else(|_| {
        panic!(
            "Could not parse `SERVER_PORT={}` in file `{}` as a u16",
            port, env_file
        )
    });

    let client = create_db_client(env_file).await.unwrap();

    // Test the connection to the database
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await
        .expect("Could not ping MongoDB");
    println!("Connected to the database successfully.");

    // Start the web server
    println!("Starting Actix-web server on http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        println!("Building app!");
        ActixApp::new()
            .app_data(web::Data::new(env))
            .app_data(web::Data::new(client.clone()))
            .service(web::scope("/api").service(api::world::get_world))
    })
    // Docker requires 0.0.0.0 and i wasted over an hour of my life
    // figuring this out.
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

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
    client.map_err(|_| "Could not create MongoDB client".to_string())
}

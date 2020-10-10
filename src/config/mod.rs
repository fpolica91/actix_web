use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};
// on the left hand side we specify the dependency and on the right the property of the dependency.
use tracing_subscriber::EnvFilter;
// struct defines a structure.

#[derive(Debug, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: i32,
}

// similar to implements in typescript, but we define the funciton that will use the struct
impl Config {
  // -> tells us what is being returned from function
  #[instrument]
  pub fn from_env() -> Result<Config> {
    dotenv().ok();
    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

    info!("Loading config");
    let mut c = config::Config::new();
    c.merge(config::Environment::default())?;
    c.try_into()
      .context("loading configuration from environment")
  }
}

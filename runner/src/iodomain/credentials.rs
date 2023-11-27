use anyhow::Ok;
use confy::{load, store};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
enum CredentialStoreError {
    #[error("There is no session stored. Please authenticate with Advent of Code before continuing.")]
    SessionNotSet,
    #[error("Credential data not loaded when expected. This is a bug.")]
    NoCredentialsFound,
}

#[derive(Serialize, Deserialize, Default)]
struct CredentialStore {
    session_cookie: String,
}

pub trait CookieStore {
    fn get_session_cookie(&self) -> anyhow::Result<&str>;
    fn set_session_cookie(&mut self, session: &str) -> anyhow::Result<()>;
}

pub struct ConfigFileCookieStore {
    config_data: Option<CredentialStore>,
}

impl ConfigFileCookieStore {
    pub fn new() -> anyhow::Result<ConfigFileCookieStore> {
        let mut instance = ConfigFileCookieStore { config_data: None };

        instance.read_file()?;

        Ok(instance)
    }

    fn read_file(&mut self) -> anyhow::Result<()> {
        self.config_data = Some(load(clap::crate_name!(), "creds")?);

        Ok(())
    }

    fn write_file(&mut self) -> anyhow::Result<()> {
        let Some(ref mut conf) = self.config_data else {
            return Err(CredentialStoreError::NoCredentialsFound.into());
        };

        Ok(store(clap::crate_name!(), "creds", conf)?)
    }
}

impl CookieStore for ConfigFileCookieStore {
    fn get_session_cookie(&self) -> anyhow::Result<&str> {
        match &self.config_data {
            None => Err(CredentialStoreError::SessionNotSet.into()),
            Some(conf) => Ok(&conf.session_cookie),
        }
    }

    fn set_session_cookie(&mut self, session: &str) -> anyhow::Result<()> {
        self.read_file()?;

        let credential_store = &mut self.config_data;
        let Some(ref mut conf) = credential_store else {
            unreachable!("No credential data found, but no error occurred while reading credential data.")
        };

        conf.session_cookie = session.to_owned();

        self.write_file()?;

        Ok(())
    }
}

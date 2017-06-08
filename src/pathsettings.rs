use std::path::PathBuf;
use app_dirs::{app_dir, AppDataType};
use super::app_info::{DELILA_INFO};
use super::errors::*;

#[derive(Clone)]
pub struct PathSettings
{
    pub logging_path: PathBuf,
    pub settings_database_path: PathBuf,
    pub database_path: PathBuf,
}
impl PathSettings {
    pub fn new() -> Result<PathSettings> {
        let logging_path = app_dir(AppDataType::UserData, &DELILA_INFO, "logs")
            .chain_err(|| "Unable to create/find a logging directory")?;
        let settings_path = app_dir(AppDataType::UserConfig, &DELILA_INFO, "settings")
            .chain_err(|| "Unable to create/find a settings directory")?;
        let database_path = app_dir(AppDataType::UserData, &DELILA_INFO, "dbs")
            .and_then(|mut database_path| {
                database_path.push("delila.db");
                Ok(database_path)
            })
            .chain_err(|| "Unable to create/find a database directory")?;
        Ok(PathSettings {
            logging_path: logging_path,
            settings_database_path: settings_path,
            database_path: database_path
        })
    }
}

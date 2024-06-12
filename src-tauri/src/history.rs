use crate::{app_error::AppError, util::get_app_directory};
use git2::Repository;

pub struct History {}

impl History {
    pub fn init() -> Result<(), AppError> {
        let path = get_app_directory()?;
        let repo = Repository::init(path).map_err(|_| AppError::HistoryInitFailed)?;

        repo.revwalk().unwrap();

        Ok(())
    }
}

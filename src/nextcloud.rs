use crate::Config;

#[derive(Clone)]
pub struct Nextcloud<'a> {
    credentials: Option<Credentials<'a>>,
    config: Config<'a>,
}

impl<'a> Nextcloud<'a> {
    pub fn new(credentials: Option<Credentials<'a>>, config: Config<'a>) -> Self {
        Nextcloud {
            credentials,
            config,
        }
    }

    pub fn work(
        &mut self,
        job_type: JobType,
        file: String,
        dest: Option<String>,
    ) -> Result<(), JobError> {
        if self.credentials.is_none() {
            return Err(JobError::NoCredentials);
        }

        match job_type {
            JobType::Upload => self.upload_file(file),
            JobType::Move => self.move_file(file, dest),
            JobType::Delete => self.delete_file(file),
        }
    }

    pub fn upload_file(&mut self, file: String) -> Result<(), JobError> {
        todo!();
    }

    pub fn move_file(&mut self, src: String, dest: Option<String>) -> Result<(), JobError> {
        if dest.is_none() {
            return Err(JobError::NoDestForMove);
        }
        todo!();
    }

    pub fn delete_file(&mut self, file: String) -> Result<(), JobError> {
        todo!();
    }
}

#[derive(Clone, Copy)]
pub struct Credentials<'a> {
    pub username: &'a str,
    pub secret: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobType {
    Upload,
    Move,
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobError {
    NoCredentials,
    NoDestForMove,
}

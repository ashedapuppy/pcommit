use std::fmt;

pub struct CommitMsg {
    pub commit_type: String,
    pub description: String,
    pub body: Option<String>,
}

impl CommitMsg {
    #[must_use]
    pub fn new(commit_type: String, description: String, body: Option<String>) -> Self {
        Self {
            commit_type,
            description,
            body,
        }
    }
}

impl fmt::Display for CommitMsg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.body {
            Some(body) => writeln!(
                fmt,
                "{}: {}\n\n{}",
                self.commit_type, self.description, body
            ),
            None => writeln!(fmt, "{}: {}\n", self.commit_type, self.description,),
        }
    }
}

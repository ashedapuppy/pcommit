use std::fmt;

pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Test,
    Refactor,
}

impl fmt::Display for CommitType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                CommitType::Feat => String::from("feat"),
                CommitType::Fix => String::from("fix"),
                CommitType::Docs => String::from("docs"),
                CommitType::Test => String::from("test"),
                CommitType::Refactor => String::from("refactor"),
            }
        )
    }
}

pub struct CommitMsg {
    pub commit_type: CommitType,
    pub description: String,
    pub body: Option<String>,
}

impl CommitMsg {
    #[must_use]
    pub fn new(commit_type: CommitType, description: String, body: Option<String>) -> Self {
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
            Some(body) => writeln!(fmt, "{}: {}\n{}", self.commit_type, self.description, body),
            None => writeln!(fmt, "{}: {}\n", self.commit_type, self.description,),
        }
    }
}

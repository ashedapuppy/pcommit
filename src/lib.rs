use std::fmt;

#[derive(Clone, Copy)]
pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Test,
    Refactor,
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

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Docs => "docs",
            CommitType::Test => "test",
            CommitType::Refactor => "refactor",
        }
        .to_owned();
        write!(f, "{}", string)
    }
}

impl fmt::Display for CommitMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.body {
            Some(body) => writeln!(f, "{}: {}\n\n{}", self.commit_type, self.description, body),
            None => writeln!(f, "{}: {}\n", self.commit_type, self.description,),
        }
    }
}

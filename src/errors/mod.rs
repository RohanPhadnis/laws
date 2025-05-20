use std::fmt::Formatter;

pub enum DbError {
    MissingFields(String),
    TableNotFound(String),
    BadInput(String),
}

impl DbError {
    fn message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::MissingFields(message) =>
                {
                    write!(f, "Missing Fields! {}", message)
                },
            DbError::TableNotFound(message) =>
                {
                    write!(f, "Table Not Found! {}", message)
                }
            DbError::BadInput(message) =>
                {
                    write!(f, "Bad Input! {}", message)
                }
        }
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message(f)
    }
}

impl std::fmt::Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.message(f)
    }
}

impl std::error::Error for DbError {}

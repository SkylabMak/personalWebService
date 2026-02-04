#[derive(Debug)]
#[allow(dead_code)]
pub enum RepositoryError {
    DatabaseError(String),
    InternalError(String),
    NotFound,
}

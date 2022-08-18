#[derive(PartialEq)]
pub enum AppError {
    ConnectionClosed(String),
    IncompleteInput(String),
}

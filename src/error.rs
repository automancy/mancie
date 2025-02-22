#[derive(Debug, Clone)]
pub struct Error {
    description: String,
    context: ErrorContext,
}
impl Error {
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn context(&self) -> &ErrorContext {
        &self.context
    }
    pub fn missing_executable(exec: String) -> Error {
        Error {
            description: format!("Couldn't find application: {}", exec),
            context: ErrorContext::MissingExecutable(exec),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ErrorContext {
    MissingExecutable(String),
}
impl ErrorContext {}

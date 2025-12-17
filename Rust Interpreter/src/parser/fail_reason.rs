#[derive(Debug, Clone, Copy)]
pub enum FailReason {
    Unknown,
    StackFrameLimit,
    NoLiteral
}

pub fn get_error_message() {}

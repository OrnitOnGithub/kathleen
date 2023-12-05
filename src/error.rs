pub enum ErrorCode {

    /// Error code for any keyword that is not recognised
    UnknownKeyword,
}

pub fn throw_error(error_code: ErrorCode, line: usize) {
    match error_code {
        ErrorCode::UnknownKeyword => {
            todo!();
        }
        _ => {
            todo!();
        }
    }
}
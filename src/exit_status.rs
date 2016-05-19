#[derive(Debug)]
pub struct ExitStatus {
    success: bool,
    code: Option<i32>,
}

impl ExitStatus {
    pub fn new(success: bool, code: i32) -> Self {
        ExitStatus {
            success: success,
            code: Some(code),
        }
    }

    getter!(success, bool);
    getter!(code, Option<i32>);
}

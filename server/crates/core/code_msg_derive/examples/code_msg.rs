use code_msg_derive::CodeMessage;

#[derive(CodeMessage)]
enum Error {
    #[status(code = 0, msg = "ok")]
    OK,
    /// 未知错误
    #[status(code = 10001, msg = "unknown error")]
    UnknownError,
}

fn main() {
    assert!(Error::UnknownError.code() == 10001);
    assert!(Error::OK.code() == 0);
}

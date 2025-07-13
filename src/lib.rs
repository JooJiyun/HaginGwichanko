mod routine;
pub mod system;
mod ui;

const APP_NAME: &str = "Hagin Gwichanko";

/// 작업의 결과를 나타내는 표준 반환 타입
///
/// * `Ok(T)`는 작업이 성공했고 그에 대한 결과를 나타냄
/// * `Err((context, error))`는 작업 실패
///   - `context`: 어느 작업에서 실패했는지 설명
///   - `error`: 실제 에러 메시지
pub type SResult<T> = Result<T, (String, String)>;

#[macro_export]
macro_rules! contexted_err {
    ($msg:expr, $err:expr) => {
        Err((
            format!("{}", $msg),
            format!("[{}:{}:{}] {}", file!(), line!(), column!(), $err),
        ))
    };
}

pub fn show_error_with_terminate(msg: &str, err: &str) {
    eprintln!("{}", err);
    uiautomation::dialogs::show_message(msg, &format!("{} - Error", APP_NAME));
    std::process::exit(1);
}

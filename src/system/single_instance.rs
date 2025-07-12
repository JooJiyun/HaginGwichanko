use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        System::Threading::{CreateMutexExW, CREATE_MUTEX_INITIAL_OWNER},
    },
};

use crate::{contexted_err, SResult};

const SINGLE_INSTANCE_MUTEX: &str = "Global\\RoutineItSingleInstance";

/// mutex 이름으로 이미 실행하고 있는지 체크
///
/// # Returns
///
/// * `Ok(true)` - single instance로 작업 중
/// * `Ok(false)` - single instance가 아님
/// * `Err((context, error))` - 실패한 경우,  
///   - `context`: 실제 에러 메시지
///   - `error`: 어느 위치/상황에서 어떤 에러가 발생했는지  
///
pub fn is_single_instance() -> SResult<bool> {
    unsafe {
        let name = U16CString::from_str(SINGLE_INSTANCE_MUTEX).unwrap();

        // 이미 존재하는 mutex -> valid한 핸들과 ERROR_ALREADY_EXISTS(183) 검출 가능
        let handle = CreateMutexExW(
            Some(std::ptr::null()),
            PCWSTR(name.as_ptr()),
            CREATE_MUTEX_INITIAL_OWNER,
            0,
        )
        .or_else(|e| contexted_err!("failed to create mutex handle", e))?;

        if handle.is_invalid() {
            contexted_err!("failed get valid mutex handle", "")?;
        }

        if GetLastError() == ERROR_ALREADY_EXISTS {
            return Ok(false);
        }
    }

    return Ok(true);
}

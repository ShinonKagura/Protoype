#[macro_export]
macro_rules! log_operation_start {
    ($msg:expr) => {
        log::info!("Starting operation: {}", $msg);
    };
}

#[macro_export]
macro_rules! log_operation_end {
    ($msg:expr, $result:expr) => {{
        match &$result {
            Ok(_) => log::info!("Operation completed: {}", $msg),
            Err(e) => log::error!("Operation failed: {} - Error: {:?}", $msg, e),
        }
        $result
    }};
}

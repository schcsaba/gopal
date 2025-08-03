use gopal::error_template::AppError;
use http::StatusCode;

#[test]
fn test_app_error_not_found_status_code() {
    let error = AppError::NotFound;
    assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn test_app_error_debug() {
    let error = AppError::NotFound;
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("NotFound"));
}

#[test]
fn test_app_error_clone() {
    let error = AppError::NotFound;
    let _cloned = error.clone();
    // Can't test equality since AppError doesn't implement PartialEq
    // but we can test that cloning works without panicking
}

#[test]
fn test_app_error_display() {
    let error = AppError::NotFound;
    let display_str = format!("{}", error);
    assert_eq!(display_str, "Not Found");
}
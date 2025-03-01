use prompt_color_tool::{Rgb, get_color_from_env, BGCOLOR_ENV_VAR};
use std::env;

#[test]
fn test_get_color_from_env() {
    // Test with environment variable set
    env::set_var(BGCOLOR_ENV_VAR, "18");
    let result = get_color_from_env(BGCOLOR_ENV_VAR);
    assert!(result.is_ok());
    let color = result.unwrap();
    assert_eq!(color.xterm, 18);
    
    // Test with invalid color value
    env::set_var(BGCOLOR_ENV_VAR, "invalid");
    let result = get_color_from_env(BGCOLOR_ENV_VAR);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unable to parse value"));
    
    // Test with missing environment variable
    env::remove_var(BGCOLOR_ENV_VAR);
    let result = get_color_from_env(BGCOLOR_ENV_VAR);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("is not set"));
}

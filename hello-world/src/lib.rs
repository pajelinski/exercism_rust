
pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(v) => format!("Hello, {}!", v),
        None => "Hello, World!".to_string()
    }
}

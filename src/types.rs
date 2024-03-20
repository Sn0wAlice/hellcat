
#[derive(Debug)]
pub struct MalwareAction {
    pub decrypt: bool,
    pub encrypt: bool,
    pub path: String
}
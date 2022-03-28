
pub struct Element {
    value: Vec<String>,
    version: u32,
}

impl Element {
    fn new() -> Element{
        let mut value: Vec<String> = Vec::new();
        let mut version = 0;
        Element {
            value,
            version,
        }
    }
}

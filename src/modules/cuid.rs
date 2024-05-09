use cuid2;

pub struct Cuid(pub String);

impl Cuid {
    pub fn new() -> Self {
        Cuid(cuid2::create_id())
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn create_cuid() -> String {
        cuid2::create_id()
    }
}

pub struct API {
    api_key: String
}

impl API {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }
}
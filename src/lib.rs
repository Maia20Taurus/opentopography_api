struct API {
    API_Key: String
}

impl API {
    pub fun new(API_Key) -> Self {
        Self {
            API_Key
        }
    }
}
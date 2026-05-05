pub struct JsonParser {
    pub m_source_code: String,
}

impl JsonParser {
    pub fn build(source_code: String) -> Self {
        JsonParser {
            m_source_code: source_code,
        }
    }
}

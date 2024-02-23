
#[derive(Clone)]
pub enum Serialization {
    Json,
    Protobuf,
    Other(String)
}


impl From<String> for Serialization {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "json" => Serialization::Json,
            "protobuf" => Serialization::Protobuf,
            _ => Serialization::Other(s)
        }
    }
}

#[derive(Clone)]
pub enum Deserialization {
    Json,
    Protobuf,
    Other(String)
}

impl From<String> for Deserialization {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "json" => Deserialization::Json,
            "protobuf" => Deserialization::Protobuf,
            _ => Deserialization::Other(s)
        }
    }
}
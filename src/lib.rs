use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "John Doe", "john@example.com");
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_user_serialization() {
        let user = User::new(1, "John Doe", "john@example.com");
        let json = user.to_json().unwrap();
        assert!(json.contains("\"name\":\"John Doe\""));
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{"id":1,"name":"John Doe","email":"john@example.com"}"#;
        let user = User::from_json(json).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }
}

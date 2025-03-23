use rust_base::User;

#[test]
fn test_user_json_roundtrip() {
    let original_user = User::new(1, "Test User", "test@example.com");
    let json_user = original_user.to_json().expect("Failed to serialize");
    let deserialized_user = User::from_json(&json_user).expect("Failed to deserialize");

    assert_eq!(original_user.id, deserialized_user.id);
    assert_eq!(original_user.name, deserialized_user.name);
    assert_eq!(original_user.email, deserialized_user.email);
}

#[test]
fn test_invalid_json_deserialization() {
    let invalid_json = r#"{"id":"not_a_number","name":123,"email":true}"#;
    assert!(User::from_json(invalid_json).is_err());
}

use rust_base::User;
use serde_json::json;

fn main() {
    // Example 1: Basic serialization and deserialization
    let user = User::new(1, "Alice", "alice@example.com");
    let json = user.to_json().expect("Serialization failed");
    println!("Serialized: {}", json);

    let deserialized_user = User::from_json(&json).expect("Deserialization failed");
    println!("Deserialized: {:?}", deserialized_user);

    // Example 2: Working with arrays of Users
    let users = vec![
        User::new(2, "Bob", "bob@example.com"),
        User::new(3, "Charlie", "charlie@example.com"),
    ];
    let json_array = serde_json::to_string(&users).expect("Array serialization failed");
    println!("Serialized array: {}", json_array);

    let deserialized_users: Vec<User> =
        serde_json::from_str(&json_array).expect("Array deserialization failed");
    println!("Deserialized array: {:?}", deserialized_users);

    // Example 3: Partial deserialization
    let partial_json = json!({
        "id": 4,
        "name": "David"
    });
    let partial_user: User =
        serde_json::from_value(partial_json).expect("Partial deserialization failed");
    println!("Partially deserialized user: {:?}", partial_user);
}

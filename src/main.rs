use rust_base::User;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        // serialization: ./program <id> <name> <email>
        [_, id, name, email] => {
            let user_id: u32 = match id.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: ID must be a valid integer");
                    std::process::exit(1);
                }
            };

            let user = User::new(user_id, name, email);
            match user.to_json() {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Serialization error: {}", e),
            }
        }

        // deserialization: ./program <json-string>
        [_, json] => match User::from_json(json) {
            Ok(user) => println!("{} \"{}\" \"{}\"", user.id, user.name, user.email),
            Err(e) => eprintln!("Deserialization error: {}", e),
        },

        // Invalid arguments
        _ => {
            eprintln!("Usage:");
            eprintln!("  Serialization: {} <id> <name> <email>", args[0]);
            eprintln!("  Deserialization: {} <json-string>", args[0]);
            std::process::exit(1);
        }
    }
}

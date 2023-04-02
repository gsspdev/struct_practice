// we define a struct by listing the fields and their types
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// we create an instance of the struct by specifying concrete values for each field


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// fn main() {
//     let mut user1 = User { //we make user1 mutable so we can change its fields
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     }; // Note: Entire instance is made mutable, not just one field

//     user1.email = String::from("anotheremail@example.com");// we change the value of a field by using dot notation

//     println!("The email of user is {}", user1.email);
// }

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
        let user2 = User {
            email: String::from("another@example.com"),
            ..user1 // we use the .. syntax and specify the instance we want to use as the starting point for the new instance
        };

    println!("The email of user1 is {}", user1.email);
    println!("The email of user2 is {}", user2.email);

    user1.email = String::from("replacedmail@example.com");// we change the value of a field by using dot notation

    println!("The email of user1 is {}", user1.email);

    // user1.email = String::from("secondedmail@example.com");// we change the value of a field by using dot notation
    // let new_email = user1.email.clone();
    // user2.email = new_email;
    // println!("The email of user1 is {}", user1.email);
    // println!("The email of user2 is {}", new_email);
}

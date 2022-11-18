struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn create_user(name: String, email: String) -> User {

    User { active: true, username: name, email: email, sign_in_count: 1 }
}

fn create_user_shorthand(username: String, email: String) -> User {
    User {username, email, active: true, sign_in_count: 1}
}

//
// Tuple Structs
struct Color(i8, i8, i8);
struct Point(i32, i32, i32);

fn main() {
    let mut user_1 = create_user(String::from("Someone Name"), String::from("someone@example.com"));
    let user_2: User = create_user_shorthand(String::from("HEllo World"), String::from("hello@world.com"));
    user_1.email = String::from("someones@example.com");

    let user_3: User = User {email: String::from("aTest@test.com"), ..user_2}; // user_2 is not invalid.

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}


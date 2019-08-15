struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user1 = User {
        username: "someone@example.com",
        email: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
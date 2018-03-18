struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// "tumple structs"
// no names on the fields, just types
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

// unit-like structs
struct MyUnit();

fn build_user(email: String, username: String) -> User {
//    User {
//        username: username,
//        email: email,
//        sign_in_count: 1,
//        active: true,
//    }
    // If vars match field names, don't need to specify field names
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn make_color_and_point() -> (Color, Point) {
    let black = Color(0, 0, 0);
    let point = Point(10, 20, 30);

    return (black, point);
}

fn main() {
    let user = build_user(String::from("joey@joeygibson.com"),
                          String::from("joeygibson"));

    // struct update syntax
    let user1 = User {
        username: String::from("foo"),
        email: String::from("foo@example.com"),
        ..user
    };

    let (color, point) = make_color_and_point();
}

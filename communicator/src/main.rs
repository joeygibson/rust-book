extern crate communicator;

// This pattern is quite common for executable projects: most functionality is
// in a library crate, and the binary crate uses that library crate. As a result,
// other programs can also use the library crate, and it’s a nice separation of concerns.
fn main() {
    communicator::client::connect();
}
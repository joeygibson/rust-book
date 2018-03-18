// If a module named foo has no submodules, you should put the declarations
// for foo in a file named foo.rs.
// If a module named foo does have submodules, you should put the declarations
// for foo in a file named foo/mod.rs.

pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::network;

    #[test]
    fn it_works() {
        // start at root
        ::client::connect();
        // or start up one level
        super::client::connect();

        network::connect();
    }
}
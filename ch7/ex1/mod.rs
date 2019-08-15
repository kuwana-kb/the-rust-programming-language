mod network {
    fn connect() {

    }
    // network::connect()

    mod client {
        fn connect() {

        }
        // network::client::connect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

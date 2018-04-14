#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod client {
    pub struct clt {
        name:String,
    }

    impl clt {
        fn new(srv_name:String) {

        }

        fn send(data:String) {

        }

        fn send_async(data:String) {

        }

        fn read() -> String {
            "".to_string()
        }

        fn poll() -> bool {
            false
        }
    }
}


mod server {
    pub struct srv {
        name: String,
    }

    impl srv {
        fn new(srv_name:String) -> srv {
            srv{name:srv_name}
        }

        fn start() {

        }

        fn stop() {

        }
    }
}

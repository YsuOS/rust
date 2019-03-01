pub mod client;

pub mod network;

mod outermost { // add pub, there is no meaning
    pub fn middle_function() {}

    fn middle_secret_function() {} //add pub

    mod inside { //add pub
        pub fn inner_function() {
            ::outermost::middle_secret_function(); 
            // can call module which is the parent module
        }

        fn secret_function() {}
    }
}

fn try_me() {
//    outermost::middle_function();
//    outermost::middle_secret_function();
//    outermost::inside::inner_function();
//    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
//        assert_eq!(2 + 2, 4);
        client::connect();
//        ::client::connect();
    }
}

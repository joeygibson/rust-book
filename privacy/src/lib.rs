mod outermost {
    pub fn middle_function() {}

    fn middle_secret_funtion() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
//    outermost::middle_secret_funtion();
//
//    outermost::inside::inner_function();
//    outermost::inside::secret_function();
}
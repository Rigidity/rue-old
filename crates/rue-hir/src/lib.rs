macro_rules! id {
    ($name:ident => $mutex:ident) => {
        static $mutex: std::sync::Mutex<u64> = std::sync::Mutex::new(0);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new() -> Self {
                let mut lock = $mutex.lock().unwrap();
                let id = *lock;
                *lock += 1;
                Self(id)
            }
        }
    };
}

id!(TypeId => NEXT_TYPE_ID);
id!(VarId => NEXT_VAR_ID);

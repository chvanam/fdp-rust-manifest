#[diplomat::bridge]
pub mod ffi {
    use fdp_definition::apps::app_1::broadcasted_events::RandomNumber;
    
    #[diplomat::opaque]
    pub struct RandomCounter(RandomNumber);

    impl RandomCounter {
        pub fn new() -> Box<Self> {
            Box::new(Self(RandomNumber { value: 0 }))
        }

        pub fn increment(&mut self) -> i32 {
            self.0.value += 1;
            self.0.value
        }

        pub fn get_value(&self) -> i32 {
            self.0.value
        }
    }
}
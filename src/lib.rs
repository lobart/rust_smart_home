mod devices {
    
    pub struct _Socket {
        pub state: bool,
        power_consumption: i32,
        description: String,
    }
    
    impl _Socket {
        pub fn _print(&self) {
            println!("{0}", self.description)
        }
        pub fn _print_power(&self) {
            println!("Power consumption is: {0}", self.power_consumption)
        }
    }
    
    
    pub struct _Termometer {
        pub temperature: i32,
    }
    
    impl _Termometer {
        pub fn _print(&self) {
            println!("Temperature is {0} Celsius", self.temperature);
        }
    }

}

/// This module implement a coffee order module with all the options for an order
pub mod coffee {
    use std::fmt;
    
    #[derive(Debug)]
    pub enum CoffeeSize {
        Big,
        Medium,
        Small,
    }

    #[derive(Debug)]
    pub enum EspressoType {
        Short,
        Long,
        Double,
    }

    #[derive(Debug)]
    pub enum IceCreamType {
        Vanilla,
        Chocolate,
        Mocha,
    }

    #[derive(Debug)]
    pub enum CoffeeType {
        Espresso { espresso_kind: EspressoType },
        Macchiato,
        Americano,
        Affogato { ice_cream: IceCreamType },
        ColdCoffee { num_ice_cubes: u32 },
    }

    impl fmt::Display for CoffeeType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CoffeeType::Espresso { espresso_kind } => write!(f, "{:?} espresso", espresso_kind),
                CoffeeType::Macchiato => write!(f, "Macchiato"),
                CoffeeType::Americano => write!(f, "Americano"),
                CoffeeType::Affogato { ice_cream } => {
                    write!(f, "Affogato with {:?} ice cream", ice_cream)
                }
                CoffeeType::ColdCoffee { num_ice_cubes } => {
                    write!(f, "Cold coffee with {num_ice_cubes} ice cubes")
                }
            }
        }
    }

    pub struct CoffeeOrder {
        pub coffee_type: CoffeeType,
        pub coffee_size: CoffeeSize,
        pub sugar_amount: u32,
    }

    impl fmt::Display for CoffeeOrder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?} {} with {} sugar spoons", self.coffee_size, self.coffee_type, self.sugar_amount)
        }
    }

    impl CoffeeOrder {
        /// Print the coffee order to the user 
        pub fn print(&self) {
            println!("The coffee order is {self}");
        }
    }
}
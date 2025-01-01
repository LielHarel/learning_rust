use std::fmt;

#[derive(Debug)]
enum CoffeeSize {
    Big,
    Medium,
    Small,
}

#[derive(Debug)]
enum EspressoType {
    Short,
    Long,
    Double,
}

#[derive(Debug)]
enum IceCreamType {
    Vanilla,
    Chocolate,
    Mocha,
}

#[derive(Debug)]
enum CoffeeType {
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

struct CoffeeOrder {
    coffee_type: CoffeeType,
    coffee_size: CoffeeSize,
}

impl fmt::Display for CoffeeOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {}",
            self.coffee_size, self.coffee_type
        )
    }
}

impl CoffeeOrder {
    fn print(&self) {
        println!("The coffee order is {self}");
    }
}

fn main() {
    let orders = [
        CoffeeOrder {
            coffee_type: CoffeeType::Affogato {
                ice_cream: IceCreamType::Chocolate,
            },
            coffee_size: CoffeeSize::Big,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::ColdCoffee { num_ice_cubes: 2 },
            coffee_size: CoffeeSize::Small,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Americano,
            coffee_size: CoffeeSize::Medium,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Espresso {
                espresso_kind: EspressoType::Double,
            },
            coffee_size: CoffeeSize::Medium,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Macchiato,
            coffee_size: CoffeeSize::Big,
        },
    ];

    println!("The orders:");
    for order in orders {
        order.print();
    }
}

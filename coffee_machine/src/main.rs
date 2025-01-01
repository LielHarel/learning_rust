use coffee_machine::coffee::*;
fn main() {
    let orders = [
        CoffeeOrder {
            coffee_type: CoffeeType::Affogato {
                ice_cream: IceCreamType::Chocolate,
            },
            coffee_size: CoffeeSize::Big,
            sugar_amount: 1,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::ColdCoffee { num_ice_cubes: 2 },
            coffee_size: CoffeeSize::Small,
            sugar_amount: 2,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Americano,
            coffee_size: CoffeeSize::Medium,
            sugar_amount: 2,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Espresso {
                espresso_kind: EspressoType::Double,
            },
            coffee_size: CoffeeSize::Medium,
            sugar_amount: 3,
        },
        CoffeeOrder {
            coffee_type: CoffeeType::Macchiato,
            coffee_size: CoffeeSize::Big,
            sugar_amount: 4,
        },
    ];

    println!("The orders:");
    for order in orders {
        order.print();
    }
}

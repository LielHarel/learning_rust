use coffee_machine::coffee;
fn main() {
    let orders = [
        coffee::CoffeeOrder {
            coffee_type: coffee::CoffeeType::Affogato {
                ice_cream: coffee::IceCreamType::Chocolate,
            },
            coffee_size: coffee::CoffeeSize::Big,
            sugar_amount: 1,
        },
        coffee::CoffeeOrder {
            coffee_type: coffee::CoffeeType::ColdCoffee { num_ice_cubes: 2 },
            coffee_size: coffee::CoffeeSize::Small,
            sugar_amount: 2,
        },
        coffee::CoffeeOrder {
            coffee_type: coffee::CoffeeType::Americano,
            coffee_size: coffee::CoffeeSize::Medium,
            sugar_amount: 2,
        },
        coffee::CoffeeOrder {
            coffee_type: coffee::CoffeeType::Espresso {
                espresso_kind: coffee::EspressoType::Double,
            },
            coffee_size: coffee::CoffeeSize::Medium,
            sugar_amount: 3,
        },
        coffee::CoffeeOrder {
            coffee_type: coffee::CoffeeType::Macchiato,
            coffee_size: coffee::CoffeeSize::Big,
            sugar_amount: 4,
        },
    ];

    println!("The orders:");
    for order in orders {
        order.print();
    }
}

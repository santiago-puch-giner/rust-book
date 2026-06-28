use packages_crates_modules::{Client, DietaryPreference, eat_at_restaurant};

fn main() {
    let client_1 = Client::new(
        String::from("Joaquin Phoenix"),
        DietaryPreference::Vegan { sattvic: false },
        None,
    );
    eat_at_restaurant(&client_1);

    let client_2 = Client::new(
        String::from("Siddharta Gothama"),
        DietaryPreference::Vegan { sattvic: true },
        Some(1),
    );
    eat_at_restaurant(&client_2);
}

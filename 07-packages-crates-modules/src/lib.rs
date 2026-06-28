mod front_of_house;
use self::front_of_house::hosting;
use self::front_of_house::serving;

pub fn eat_at_restaurant(client: &Client) {
    let client_name = &client.name;
    let client_diet = &client.dietary_preference;
    println!("Client {client_name} wants to eat (diet: {client_diet:?}");
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
    println!("Client {client_name} finished eating at our restaurant")
}

// Enums must be declared pub if they should be used from other modules
#[derive(Debug)]
pub enum DietaryPreference {
    // But their variants are automatically pub if the whole enum is pub
    Flexitarian,
    Pescetarian,
    Vegetarian,
    Vegan { sattvic: bool },
}

// Structs must be declared pub if they should be used from other modules
pub struct Client {
    // And so do their fields if we want to mutate them directly
    name: String,
    pub dietary_preference: DietaryPreference,
    pub fidelity_id: Option<u32>,
}

impl Client {
    pub fn new(
        name: String,
        dietary_preference: DietaryPreference,
        fidelity_id: Option<u32>,
    ) -> Client {
        Client {
            name,
            dietary_preference,
            fidelity_id,
        }
    }
}

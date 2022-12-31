enum HomeType {
    Apartment(String, u32),
    House(String, u32)
}

enum Message {
    Quit{x: u32, y: String}
}

fn main() {
    let house = HomeType::House(String::from("Pretty"), 7);
    let apartment = HomeType::Apartment(String::from("ugly"), 87);

    let message = Message::Quit({
            x: 32,
            y: String::from("hey")
        });

    decorate(house);
    decorate(apartment);
    println!("Hello, world!");
}

fn decorate(homeType: HomeType) {
  
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&mut self) -> ShirtColor {
        self.shirts.pop().unwrap()
    }

    fn get_shirt(&self, user_choice: Option<ShirtColor>) -> ShirtColor {
        user_choice.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;
        for shirt in self.shirts.iter() {
            if *shirt == ShirtColor::Red {
                red_count += 1;
            } else {
                blue_count += 1;
            }
        }
        if (red_count > blue_count) {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn fail_case() -> ShirtColor {
    ShirtColor::Red
}

fn main() {
    let shirt_1 = ShirtColor::Red;
    let shirt_2 = ShirtColor::Red;
    let shirt_3 = ShirtColor::Red;
    let shirt_4 = ShirtColor::Blue;

    let mut inventory = Inventory {
        shirts: vec![shirt_1, shirt_2, shirt_3, shirt_4]
    };
    // inventory.get_shirt();

    let opt_1: Option<ShirtColor> = Some(ShirtColor::Red);
    let opt_2: Option<ShirtColor> = None;
    opt_1.unwrap();
    opt_2.unwrap_or_else(|| fail_case());
}


// Functional programming is passing functions as arguments, returning functions from functions and assigning them from variables

//Functional concepts: Closures, iterators,
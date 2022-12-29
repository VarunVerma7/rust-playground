
struct NewsPaper {
    articles: String,
    author: String,
    pages: u32
}


struct Empty;

struct Point(i32, i32, i32);
fn main() {
    let mut newspaper = NewsPaper {
        articles: String::from("BOB"),
        author: String::from("Joe"),
        pages: 10
    };

    newspaper.pages = 100;
    let newspaper2 = build_newspaper(String::from("Hey"), String::from("byte"));


    let newspaper3 = NewsPaper {
        pages: 1000,
        ..newspaper2
    };

    let point = Point(1, 2, 3);
    let empty = Empty;


    let rectangle = (2, 5);
    println!("The area is {}", get_area(rectangle));

    let rectangle_2 = Rectangle {
        height: 30,
        width: 25
    };
    let area_2 = get_area_2(&rectangle_2);
    let rectangle_3 = Rectangle {
        height: 990,
        width: 20
    };
    println!("The rectangle is {:?}", rectangle_2.area());
    println!("Can it hold: {}", rectangle_2.can_hold(&rectangle_3));
}


fn build_newspaper(articles: String, author: String) -> NewsPaper {
    NewsPaper {
        articles,
        author,
        pages: 10
    }
}
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width 
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.height * self.width > other_rec.height * other_rec.width
    }
}

fn get_area_2(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn get_area(rectangle: (i32, i32)) -> i32 {
    return rectangle.0 * rectangle.1
}
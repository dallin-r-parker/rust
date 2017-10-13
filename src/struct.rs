fn main() {

    let circle1 = Circle {
        x: 10.0, y: 10.0, radius: 10.0
    };

    println!("X : {} Y : {} Radius : {}", circle1.x, circle1.y, circle1.radius);

    println!("Circle Radius : {}", get_radius(&circle1));
    println!("Circle X : {}", circle1.get_x());

    println!("Circle Area : {}", circle1.area());

    let rectangle1 = Rectangle {
        height: 10.0, width: 25.0
    };

    println!("Rectangle Area : {}", rectangle1.area());

    // ============ relating to ENUM
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned()};

    get_info(hulk);
    get_info(spiderman);
    get_info(quicksilver);

}
// ========================= STRUCT
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    height: f64,
    width: f64,
}
// Traits are similar to interfaces in other languages
// ========================= TRAITS
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.1415 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
// ========================= ENUM
enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} =>  {
            println!("{} is {}", name, secret);
        },

    }
}
struct Circle {
    radius: f64,
    area: Option<f64>,
}

impl Circle {
    // Constructor method
    fn new(radius: f64) -> Circle {
        Circle { radius, area: None }
    }

    // Method to calculate the area of the circle
    fn calculate_area(&mut self) {
        self.area = Some(std::f64::consts::PI * self.radius * self.radius)
    }

    fn get_area(&self) -> Option<f64> {
        self.area
    }
}

fn main() {
    let mut my_circle = Circle::new(3.0);

    my_circle.calculate_area();
    match my_circle.get_area() {
        Some(area) => {
            println!("The area of the circle is: {}", area);
        }
        None => {
            println!("The area of the circle is not calculated.");
        }
    }
}
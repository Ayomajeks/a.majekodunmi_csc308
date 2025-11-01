struct Circle {
    radius: f32,
}

fn main() {
    const PI: f32 = 3.214;
    let circle1 = Circle {
        radius: 12.0,

    };
    
    let exponent: f32 = 2.0;
    let circumference: f32 = 2.0 * PI * circle1.radius;
    let  area: f32 = PI * circle1.radius.powf(exponent);

    println!("The circumference of the circle is {}", circumference);
    println!("The area of the circle {}", area);

}

const PI: f64 = 3.14159;

fn main() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("");
    let num: f64 = a.trim().parse().unwrap();
    let volume: f64 = num.powf(3.0) * PI * (4.0 / 3.0);
    println!("VOLUME = {:.3}", volume);
}

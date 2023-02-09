const PI: f64 = 3.14159;

fn main() {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Error read line");

    let num: f64 = a.trim().to_string().parse::<f64>().unwrap();
    let area: f64 = f64::powf(num, 2.0) * PI;

    println!("A={:.4}", area);
}

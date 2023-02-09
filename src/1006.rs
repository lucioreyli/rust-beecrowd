const WEIGHT_A: u32 = 2;
const WEIGHT_B: u32 = 3;
const WEIGHT_C: u32 = 5;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    std::io::stdin().read_line(&mut a).expect("Error read line");
    std::io::stdin().read_line(&mut b).expect("Error read line");
    std::io::stdin().read_line(&mut c).expect("Error read line");

    let num_a: f64 = a.trim().to_string().parse::<f64>().unwrap();
    let num_b: f64 = b.trim().to_string().parse::<f64>().unwrap();
    let num_c: f64 = c.trim().to_string().parse::<f64>().unwrap();

    let avg: f64 =
        ((num_a * WEIGHT_A as f64) + (num_b * WEIGHT_B as f64) + (num_c * WEIGHT_C as f64))
            / (WEIGHT_A + WEIGHT_B + WEIGHT_C) as f64;

    println!("MEDIA = {:.1}", avg);
}

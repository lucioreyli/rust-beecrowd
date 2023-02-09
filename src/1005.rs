fn main() {
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin().read_line(&mut a).expect("Error read line");
    std::io::stdin().read_line(&mut b).expect("Error read line");

    let num_a: f64 = a.trim().to_string().parse::<f64>().unwrap();
    let num_b: f64 = b.trim().to_string().parse::<f64>().unwrap();

    let avg: f64 = ((num_a * 3.5)+ (num_b * 7.5)) / (3.5 + 7.5);

    println!("MEDIA = {:.5}", avg);
}

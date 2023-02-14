fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let total_distance: f64 = input.trim().parse::<f64>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).expect("");
    let total_gas_used: f64 = input.trim().parse::<f64>().unwrap();
    println!("{:.3} km/l", total_distance / total_gas_used);
}

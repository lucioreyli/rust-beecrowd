fn read_line(var_to_read: &mut String) -> usize {
    std::io::stdin().read_line(var_to_read).expect("")
}

fn main() {
    let mut employer_name: String = String::new();
    let mut input = String::new();
    read_line(&mut employer_name);
    
    read_line(&mut input);
    let salary: f64 = input.trim().parse::<f64>().unwrap();
    input.clear();

    read_line(&mut input);
    let total_sell: f64 = input.trim().parse::<f64>().unwrap();

    let result: f64 = salary as f64 + ((total_sell as f64) * (0.15));

    println!("TOTAL = R$ {:.2}", result);
}

fn main() {
    let mut total: f64 = 0.0;
    for _ in 1..3 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("");
        let product_line: Vec<&str> = input.split(' ').collect();
        let quantity: u32 = product_line[1].trim().parse::<u32>().unwrap();
        let product_price: f64 = product_line[2].trim().parse::<f64>().unwrap();
        total = total + (quantity as f64 * product_price);
    }
    println!("VALOR A PAGAR: R$ {:.2}", total);
}

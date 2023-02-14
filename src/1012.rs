fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let vars_line: Vec<&str> = input.split(' ').collect();

    let a: f64 = vars_line[0].trim().parse::<f64>().unwrap();
    let b: f64 = vars_line[1].trim().parse::<f64>().unwrap();
    let c: f64 = vars_line[2].trim().parse::<f64>().unwrap();

    println!("TRIANGULO: {:.3}", (a * c) / 2.0);
    println!("CIRCULO: {:.3}", 3.14159 * c.powf(2.0));
    println!("TRAPEZIO: {:.3}", ((a + b) * c) / 2.0);
    println!("QUADRADO: {:.3}", b.powf(2.0));
    println!("RETANGULO: {:.3}", a * b);
}

fn main() {
    let p = 210_000.0;
    let r = 5.0;
    let t = 3.0;

    let a = p * ((1.0 - (r / 100.0)) as f64).powf(t);

    println!("Amount: {}", a);

    let ci = a - p;
    println!("Compound Interest: {}", ci);

    let dep = p - ci;
    println!("Value after {} years: {}", t, dep);
}

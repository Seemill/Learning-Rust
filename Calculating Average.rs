fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let average: f64 = (a + b + c) / 3.0;

    assert_eq!(average, 45.1);
    
    println!("Test passed!"); 
}

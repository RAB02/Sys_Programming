fn main(){
    let fp :f64 = 32.0;
    println!("{}", fahrenheit_to_celsius(fp));

    println!("{}", celsius_to_fahrenheit(fp));
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    let cel: f64;

    cel = (f - 32.0) * 5.0 / 9.0;

    return cel;
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let fah: f64;

    fah = (1.8 * c) + 32.0;

    return fah;
}

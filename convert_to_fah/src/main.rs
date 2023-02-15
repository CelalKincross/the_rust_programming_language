fn main() {
    let conversion = cel_to_fah(23.0);
    println!("{conversion:.2}");
}

fn cel_to_fah(celsius:f32)-> f32 {
    celsius*(9.0/5.0) + 32.0
}
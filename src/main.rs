mod vector;

fn main() {
    let a = vector::vec3(10.0, 20.0, 30.0);
    println!("{:?}", 2.0 * (10.0 + a));
}

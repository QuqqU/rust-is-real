use is_real::is_real;

fn main() {
    let float = "1.5e3".to_string();
    let rlt = is_real(&float);
    assert_eq!(rlt, true);
    println!("{} {}", float, rlt);
}

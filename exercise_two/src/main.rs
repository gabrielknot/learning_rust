fn main() {
    let number: u32 = 30;
    assert_eq!("u32".to_string(), type_of(&number));
    println!("Hello, world!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())

}

fn main() {
    let mut number: i32 = 5;
    number+=5;
    //let y: i32 = 30;

    {
        let y: i32 = 30;
        println!("this is the number in an scope {}",y);
    }

    println!("this is the number {}",number);
}

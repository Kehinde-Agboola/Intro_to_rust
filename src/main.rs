// Variables
fn add(num_one: i32, num_two: i32) -> i32{
    return num_one + num_two; 
}
fn main() {
    let total = add(0, 0);

    if total > 50 {
        println!("you're qualify for free shipping");

    }
        else if total > 20 {
            println!("if you add more items, you can qualify for free shipping");
        }
    else{
        println!("No free shipping");
    }
    println!("{:?}", total);
}



// Variables
fn add(num_one: i32, num_two: i32) -> i32{
    return num_one + num_two; 
}
fn main() {
    let mut total = add(10, 15);
    let mut free_shipping = false;
    if total > 50 {
        println!("you're qualify for free shipping");
        free_shipping = true;
    }
        else if total > 20 {
            println!("if you add more items, you can qualify for free shipping");
        }
    else{
        println!("No free shipping");
    }
    match free_shipping {
        true => total + 0,
        false => total + 5
    };

    match total{
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found")
    }

    // match total{
    //     1 => println!('')
    // }
    println!(" Total:  {:?}", total);

    let items: [i32;5] = [1, 2, 3, 4, 5];
    println!("Arrys: {:?}", items);

    let vector_items = vec![1, 2, 3, 4, 5];
    let mut vector_items_2 = Vec::new();

    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(12);
    vector_items_2.push(3);
    vector_items_2.push(2);

println!("Vector_items: {:?}", vector_items);
println!("Vector_items_2 {:?}", vector_items_2);


}



fn main() {
    const MAX_HEALTH: u32 = 1_000_000;
    println!("Max health: {}", MAX_HEALTH);
    let user_data_tuple: (i8, f32, bool, char) = (42, 5.2, true, 'S');
    println!("User data: {}", user_data_tuple.0);
    println!("User data: {}", user_data_tuple.1);
    println!("User data: {}", user_data_tuple.2);
    println!("User data: {}", user_data_tuple.3);
    let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
    arr[0] = 10;
    println!("array[0] data: {}", arr[0]);
}

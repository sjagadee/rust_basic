
const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {

    println!("Welcome to this course on {}", OUR_COURSE);

    // stack
    let x: i32;
    x = 10;
    println!("{}", x);

    let y: i32 = 5;
    println!("{}", y);

    // control statements - for loop
    for i in 0..=y {
        if i != y {
            print!("{}, ", i);
        } else {
            println!("{} ", i)
        }
        
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {}", z);

    let freezing_temp: f32 = -2.4;
    println!("freezing temp is {}", freezing_temp);

    let is_zero_reminder: bool = 10  % 4 != 0;
    println!("is_zero_reminder is {}", is_zero_reminder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji: char = '😀';
    println!("emoji char is {}", emoji);

    // static array of floats
    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n: f32| n + 2.1);
    println!("my_floats_new is {:?}", my_floats_new);
}

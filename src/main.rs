mod m1_enums;
mod m2_structs;
mod m3_traits;
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

    // string litral
    let name: &str = "Srinivas";
    println!("name is {}", name);

    let dynamic_name: String = String::from("Srinivas Jagadeesh");
    println!("dynamic name is {} and its memory addr is {:p}", dynamic_name, &dynamic_name);

    let new_dyn_name: String = name.to_string();
    println!("name {}", new_dyn_name);

    let str_slice: &str = &dynamic_name[0..8];
    println!("name is {}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    println!("removed char is {}", removed_char);
    println!("chars is {:?}", chars);

    // chars.iter().for_each(|c| print!("{}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect();
    dbg!(collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            println!(", world!!");
        }
    }

    // closures
    let num:i32 = 4;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(5);
    dbg!(new_num);

    // Number Literals (from rust book)
    println!("Big number is {}", 98_222_000);
    println!("Hex number is {}", 0xff);
    println!("Octal number is {}", 0o77);
    println!("Binary number is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');

    // Raw String Literal
    let text: &str = r#"{"msg": "Rust is Awesome"}"#;
    dbg!(text);

    // Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a in binary {:08b}", a);
    println!("b in binary {:08b}", b);

    // Logic gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);

    // bit wise oprations
    println!("a << 1 {:08b}", a << 1);
    println!("a << 1 {}", a << 1);
    println!("a >> 1 {:08b}", a >> 1);
    println!("a << 1 {}", a >> 1);

}

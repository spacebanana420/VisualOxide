use std::io;

pub fn answer() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    return userinput;
}

pub fn answer_to_u8() -> u8 {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    let userinput:u8 = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}

pub fn answer_to_u32() -> u32 {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    let userinput:u32 = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}

pub fn remove_extension(originalstring:&str) -> String { //needs testing
    let mut original_extension = String::new();
    let mut addchars = false;
    for i in originalstring.chars() {
        if i == '.' {addchars = true;}
        if addchars == true {
            original_extension.push(i);
        }
    }
    let finalstring = String::from(originalstring).replace(&original_extension, "");
    return finalstring;
}

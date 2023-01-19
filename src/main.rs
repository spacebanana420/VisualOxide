use std::fs;
mod userinput;
mod generalops;
mod colorops;

fn main() {
    println!("0. Exit    1. Resize     2. Crop     3. Image to ASCII     4. Image to Ico (untested)");
    println!("");
    println!("5. Contrast adjustment");
    println!("");
    println!("Choose an operation");
    let operation = userinput::answer_to_u8();
    let currentdir = fs::read_dir(".").unwrap();
    let mut horizontal_count:u8 = 0;
    for dir in currentdir {
        println!("{}", dir.unwrap().path().display());
        horizontal_count+=1;
        if horizontal_count == 3 {
            horizontal_count = 0;
            println!("");
        }
    }
    println!("Input image file");
    let inputimg = userinput::answer();

    match operation {
        0=> return,
        1=> generalops::resizeimg(&inputimg),
        2=> generalops::cropimg(&inputimg),
        3=> generalops::image_to_ascii(&inputimg),
        4=> generalops::icogen(&inputimg),
        5=> colorops::contrast_adjust(&inputimg),
        _=> {
            println!("Please choose one of the available operation");
        }
    }
}

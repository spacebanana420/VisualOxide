use std::fs;
mod userinput;
mod generalops;
mod colorops;

fn main() { //print only if file contains image extension
    let image_formats = [".exr", ".tga", ".png", ".jpg", ".JPG", ".jpeg", ".tiff", ".tif", ".TIF", ".TIFF", ".bmp", ".webp", ".heic", ".heif", ".avif"];
    println!("0. Exit    1. Resize     2. Crop     3. Image to ASCII     4. Image to Ico (untested)");
    println!("");
    println!("5. Contrast adjustment");
    println!("");
    println!("Choose an operation");
    let operation = userinput::answer_to_u8();
    let currentdir = fs::read_dir(".").unwrap();
    let mut horizontal_count:u8 = 0;
    let charlength_max = get_max_charlength();

    for dir in currentdir {
        let dir_path = dir.unwrap().path();
        let dir_string = dir_path.to_str().unwrap();
        let mut isimage = false;
        for imgfmt in image_formats {
            if dir_string.contains(imgfmt) == true || dir_string.contains(".") == false {
                isimage = true;
                break;
            }
        }
        if isimage == true {
            let charcount:u8 = dir_string.chars().count().try_into().unwrap();
            let addspace:u8 = if charcount < charlength_max {
                charlength_max - charcount + 3
            }
            else if charcount == charlength_max {
                3
            }
            else {
                charcount - charlength_max + 3
            };
            let mut addspace_string = String::new();
            for i in 0..addspace {addspace_string.push_str(" ");}
            print!("{}{}", dir_path.display(), addspace_string);
            horizontal_count+=1;
            if horizontal_count == 6 {
                horizontal_count = 0;
                println!("");
            }
        }
    }

    println!(""); println!(""); println!("Input image file");
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

fn get_max_charlength () -> u8 {
    let currentdir = fs::read_dir(".").unwrap();
    let mut charlength_max:u8 = 0;
    for dir in currentdir {
        let charlength:u8 = dir.unwrap().path().to_str().unwrap().chars().count().try_into().unwrap();
        if charlength > charlength_max {charlength_max = charlength;}
    }
    return charlength_max;
}

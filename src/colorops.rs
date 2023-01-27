use image::{GenericImageView, GenericImage, DynamicImage};
use crate::userinput;

fn open_image(filename:&str) -> image::DynamicImage {
    let inputimage = image::open(filename.trim()).expect("Image file not found in specified directory");
    return inputimage;
}

fn brightest_channel (r:f32, g:f32, b:f32) -> u8 { //needs testing or improvement
    //for the returned value, red is 0, green is 1, blue is 2
    //let (pos_r, pos_g, pos_b):u8 = (1, 2, 3);
    if r < g {
        if r < b {
            if b < g {return 1;} else {return 2;}
        }
        else {return 1;}
    }
    else if r < b {return 2;}
    else {return 0;}
}

fn brightest_pixel (img:&DynamicImage, width:u32, height:u32) -> f32 {
    let mut brightest_value:f32 = 0.0;
    let mut brightness:f32;
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            let brightness:f32 = (r + g + b) / 3.0;
            if brightness > brightest_value {brightest_value = brightness;}
        }
    }
    return brightest_value;
}

pub fn contrast_adjust (imgname:&str) { //function is unfinished and will only work for 8 bit for now
    let mut img = open_image(&imgname);
    let (width, height) = img.dimensions();
    let mut contrastaddition:u32;
    //let mut newpixel: [u32; 4] = [0; 4];
    let mut newpixel = image::Rgba([0, 0, 0, 0]);
    println!("Choose the contrast middle point (0-255)");
    let middlepoint = userinput::answer_to_f32();
    //let top_brightness = 100.0; //brightest_pixel(&img, width, height);
    let top_brightness = middlepoint * 2.0;
    let increment:f32 = 40.0;
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            let brightness:f32 = (r + g + b) / 3.0;
            let whichis_brightest = brightest_channel(r, g, b);
            let mut top_channel:f32 = 0.0;

            match whichis_brightest {
                0=> top_channel = r,
                1=> top_channel = g,
                2=> top_channel = b,
                _=> top_channel = r, //never happens
            }
            if brightness > middlepoint {
                newpixel[0] = (r + increment * r / top_channel / brightness * top_brightness) as u8;
                newpixel[1] = (g + increment * g / top_channel / brightness * top_brightness) as u8;
                newpixel[2] = (b + increment * b / top_channel / brightness * top_brightness) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
            else if brightness < middlepoint / 2.0 {
                newpixel[0] = (r - increment * r / top_channel * brightness / top_brightness) as u8;
                newpixel[1] = (g - increment * g / top_channel * brightness / top_brightness) as u8;
                newpixel[2] = (b - increment * b / top_channel * brightness / top_brightness) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
        }
    }
    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_contrast.png");
    img.save(exportname).expect("Could not save the ico image");
}


/*
            if brightness > 127.0 {
                newpixel[0] = (r + increment * r / brightest / brightness * 255.0) as u8;
                newpixel[1] = (g + increment * g / brightest / brightness * 255.0) as u8;
                newpixel[2] = (b + increment * b / brightest / brightness * 255.0) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
            else if brightness < 127.0 { //needs inversion for brightness
                newpixel[0] = (r - increment * r / brightest * brightness / 255.0) as u8;
                newpixel[1] = (g - increment * g / brightest * brightness / 255.0) as u8;
                newpixel[2] = (b - increment * b / brightest * brightness / 255.0) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
            */

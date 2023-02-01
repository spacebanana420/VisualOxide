use image::{GenericImageView, DynamicImage, GenericImage};
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
/*
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
}*/

pub fn contrast_adjust (imgname:&str) { //function is unfinished and will only work for 8 bit for now
    let mut img = open_image(&imgname);
    let (width, height) = img.dimensions();
    let mut newpixel = image::Rgba([0, 0, 0, 0]);

    println!("Choose the contrast middle point (0-255) (above increases brightness, below decreases)");
    let middlepoint = userinput::answer_to_f32();
    println!("Choose the contrast increase level (1-200) (higher value, higher contrast)");
    let mut contrast_base = userinput::answer_to_u8();
    if contrast_base > 200 {contrast_base = 200;}
    else if contrast_base == 0 {contrast_base = 1;}
    //let top_brightness = 100.0; //brightest_pixel(&img, width, height);
    let top_brightness = middlepoint * 2.0;
    let contrast_base:f32 = contrast_base as f32;
    let mut top_channel:f32 = 0.0;

    for y in 0..height { //smooth it with cos
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            let brightness:f32 = (r + g + b) / 3.0;
            let whichis_brightest = brightest_channel(r, g, b);
            match whichis_brightest {
                0=> top_channel = r,
                1=> top_channel = g,
                2=> top_channel = b,
                _=> top_channel = r, //never happens
            }
            if brightness > middlepoint {
                let contrast_add:f32 = middlepoint / brightness * contrast_base;
                newpixel[0] = (r + contrast_add * r / top_channel) as u8;
                newpixel[1] = (g + contrast_add * g / top_channel) as u8;
                newpixel[2] = (b + contrast_add * b / top_channel) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
            else if brightness < middlepoint / 2.0 {
                let contrast_subtract:f32 = brightness / middlepoint * contrast_base;
                newpixel[0] = (r - contrast_subtract * r / top_channel) as u8;
                newpixel[1] = (g - contrast_subtract * g / top_channel) as u8;
                newpixel[2] = (b - contrast_subtract * b / top_channel) as u8;
                newpixel[3] = alpha as u8;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
        }
    }
    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_contrast.png");
    img.save(exportname).expect("Could not save the ico image");
}

pub fn saturation_adjust (imgname:&str) {
    let mut img = open_image(&imgname);
    let (width, height) = img.dimensions();
    let mut newpixel = image::Rgba([0, 0, 0, 0]);

    println!("Choose the saturation increase level (1-200) (higher value, higher saturation)");
    let mut saturation_base = userinput::answer_to_u8();
    println!("1. Hue preservation (default)     2. No preservation"); println!("Choose a saturation mode");
    let mut saturation_mode = userinput::answer_to_u8();
    if saturation_mode != 1 && saturation_mode != 2 {saturation_mode = 1;}
    if saturation_base > 200 {saturation_base = 200;}
    else if saturation_base == 0 {saturation_base = 1;}
    let saturation_base:f32 = saturation_base as f32;

    let mut top_channel:f32 = 0.0;
    let mut other_channel1:f32 = 0.0; let mut other_channel2:f32 = 0.0;

    let mut top_channel_index = 1;
    let mut other_channel_index1 = 1; let mut other_channel_index2 = 2;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            let brightness:f32 = (r + g + b) / 3.0;
            let whichis_brightest = brightest_channel(r, g, b);
            match whichis_brightest {
                0=> {
                    top_channel = r; top_channel_index = 0;
                    other_channel1 = g; other_channel_index1 = 1;
                    other_channel2 = b; other_channel_index2 = 2;
                },
                1=> {
                    top_channel = g; top_channel_index = 1;
                    other_channel1 = r; other_channel_index1 = 0;
                    other_channel2 = b; other_channel_index2 = 2;
                },
                2=> {
                    top_channel = b; top_channel_index = 2;
                    other_channel1 = g; other_channel_index1 = 1;
                    other_channel2 = r; other_channel_index2 = 0;
                },
                _=> { //never happens
                    top_channel = r; top_channel_index = 0;
                    other_channel1 = g; other_channel_index1 = 1;
                    other_channel2 = b; other_channel_index2 = 2;
                }
            }
            //make saturation_add, do not clip
            let saturation_base:f32 = saturation_base as f32;

            newpixel[top_channel_index] = (top_channel + saturation_base) as u8;
            newpixel[3] = alpha as u8;
            if saturation_mode == 1 {
                if other_channel1 > other_channel2 {
                    newpixel[other_channel_index1] = (other_channel1 + saturation_base * other_channel1 / top_channel) as u8;
                    newpixel[other_channel_index2] = (other_channel2 - saturation_base * other_channel2 / top_channel) as u8;
                }
                else {
                    newpixel[other_channel_index1] = (other_channel1 - saturation_base * other_channel1 / top_channel) as u8;
                    newpixel[other_channel_index2] = (other_channel2 + saturation_base * other_channel2 / top_channel) as u8;
                }
            }
            else {
                newpixel[other_channel_index1] = (other_channel1 - saturation_base * other_channel1 / top_channel) as u8;
                newpixel[other_channel_index2] = (other_channel2 - saturation_base * other_channel2 / top_channel) as u8;
            }
            /*newpixel[0] = (r + saturation_add * r / top_channel) as u8;
            newpixel[1] = (g + saturation_add * g / top_channel) as u8;
            newpixel[2] = (b + saturation_add * b / top_channel) as u8;
            newpixel[3] = alpha as u8;*/
            DynamicImage::put_pixel(&mut img, x, y, newpixel);
        }
    }
    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_saturation.png");
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

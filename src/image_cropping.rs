use image::{DynamicImage, GenericImageView, Rgba, Rgb};

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn get_top_left_corner(img: &DynamicImage) -> (u32, u32) {
    let mut corner_x = 0;
    let mut corner_y = 0;

    'outer: for x in 0..img.width() {
        for y in 0..img.height() {
            if img.get_pixel(x, y) != WHITE {
                corner_x = x;
                break 'outer
            }
        }
    }

    'outer: for y in 0..img.height() {
        for x in 0..img.width() {
            if img.get_pixel(x, y) != WHITE {
                corner_y = y;
                break 'outer
            }
        }
    }
    
    (corner_x, corner_y)
}

fn get_bottom_right_corner(img: &DynamicImage) -> (u32, u32) {
    let x_range = (0..(img.width() - 1)).rev();
    let y_range = (0..(img.height() - 1)).rev();

    let mut corner_x = img.width();
    let mut corner_y = img.height();
    
    'outer: for x in x_range {
        for y in 0..img.height() {
            if img.get_pixel(x, y) != WHITE {
                corner_x = x;
                break 'outer
            }
        }
    }

    'outer: for y in y_range {
        for x in 0..img.width() {
            if img.get_pixel(x, y) != WHITE {
                corner_y = y;
                break 'outer
            }
        }
    }

    (corner_x, corner_y)
}

pub fn crop_image(image_path: &str) {
    let img: DynamicImage = image::open(image_path).unwrap();

    let (top_x, top_y) = get_top_left_corner(&img);
    let (bottom_x, bottom_y) = get_bottom_right_corner(&img);

    let img = img.crop_imm(top_x, top_y, bottom_x - top_x, bottom_y - top_y);
    img.save(image_path).expect("Error saving cropped image");
}

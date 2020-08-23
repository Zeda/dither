extern crate image;

use std::u32;
use image::{ImageBuffer, Rgb, GenericImageView};

fn to_color(s : &str) -> [u8; 3] {
    let mut x:u32 = 0;
    for c in s.chars(){
        if c >= '0' && c <= '9' {
            x <<= 4;
            x += c as u32 - '0' as u32;
        } else if c >= 'A' && c <= 'F' {
            x <<= 4;
            x += c as u32 - 'A' as u32 + 10;
        } else if c >= 'a' && c <= 'f' {
            x <<= 4;
            x += c as u32 - 'a' as u32 + 10;
        }
    }


    if s.len() == 4 {
        return [
                (((x>>8)&15) * 17) as u8,
                (((x>>4)&15) * 17) as u8,
                ((x&15) * 17) as u8
               ]
    } else {
        return [
                ((x>>16)&255) as u8,
                ((x>>8)&255) as u8,
                (x&255) as u8
               ]
    }
}

fn dist8(n: u8, m: u8) -> u32{
    let dif:i32 = n as i32 - m as i32;
    return (dif * dif) as u32;
}

fn coloradjust(nxt : u8, cur : u8, new : u8, weight : i32, divisor : i32) -> u8 {
    // need (nxt +(cur-new)*weight/divisior)
    let out = nxt as i32 +(cur as i32 - new as i32)*weight/divisor;
    if out > 255 {
        return 255;
    } else if out < 0 {
        return 0;
    } else {
        return out as u8;
    }
}

fn main() {
    // let mut x:u32;
    // let mut y:u32;
    let mut r:u8;
    let mut g:u8;
    let mut b:u8;
    // let mut color:u8;
    let mut c0:u8;
    let mut c1:u8;
    let mut c2:u8;
    let mut palette_size:usize;
    let mut palette:[[u8; 3]; 32]=[[0u8; 3]; 32];

    let args: Vec<_> = std::env::args().collect();


    // a default (black) image containing Rgb values
    let src = image::open(&args[1]).unwrap();
    let (width, height) = src.dimensions();
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    palette_size = 0;
    for i in 2.. (args.len() - 1) {
        if args[i].chars().next().unwrap() == '#' {
            palette[palette_size] = to_color(&args[i]);
            palette_size += 1;
        }
    }


    for y in 1..height {
        for x in 1..width {
            let pixel = src.get_pixel(x, y);
            image.put_pixel(x, y, image::Rgb([pixel[0],pixel[1],pixel[2]]));
        }
    }

    for y in 1..height {
        for x in 1..width {
            let pixel = image.get_pixel(x, y);
            r = pixel[0];
            g = pixel[0];
            b = pixel[0];

// Now select the closest palette match
            c0 = palette[0][0];
            c1 = palette[0][1];
            c2 = palette[0][2];
            let mut delta:u32 = dist8(r,c0) + dist8(g,c1) + dist8(b,c1);
            for i in 1.. (palette_size - 1) {
                let dist:u32 = dist8(r, palette[i][0]) +
                               dist8(g, palette[i][1]) +
                               dist8(b, palette[i][2]);

                if dist < delta {
                    c0 = palette[i][0];
                    c1 = palette[i][1];
                    c2 = palette[i][2];
                    delta = dist;
                }
            }

            if x < width - 1{
                let pixel10 = image.get_pixel(x + 1, y);
                let r10 = pixel10[0];
                let g10 = pixel10[1];
                let b10 = pixel10[2];
                image.put_pixel(x + 1, y, image::Rgb([
                                            coloradjust(r10, r, c0, 7, 16),
                                            coloradjust(g10, g, c1, 7, 16),
                                            coloradjust(b10, b, c2, 7, 16)]));
            }

            if y < height - 1 {
                if x > 0 {
                    let pixeln11 = image.get_pixel(x - 1, y + 1);
                    let rn11 = pixeln11[0];
                    let gn11 = pixeln11[1];
                    let bn11 = pixeln11[2];
                    image.put_pixel(x - 1, y + 1, image::Rgb([
                                                coloradjust(rn11, r, c0, 3, 16),
                                                coloradjust(gn11, g, c1, 3, 16),
                                                coloradjust(bn11, b, c2, 3, 16)]));
                }

                if x < width - 1 {
                    let pixel11 = image.get_pixel(x + 1, y + 1);
                    let r11 = pixel11[0];
                    let g11 = pixel11[1];
                    let b11 = pixel11[2];
                    image.put_pixel(x + 1, y + 1, image::Rgb([
                                                coloradjust(r11, r, c0, 1, 16),
                                                coloradjust(g11, g, c1, 1, 16),
                                                coloradjust(b11, b, c2, 1, 16)]));
                }

                let pixel01 = image.get_pixel(x, y + 1);
                let r01 = pixel01[0];
                let g01 = pixel01[1];
                let b01 = pixel01[2];
                image.put_pixel(x, y + 1, image::Rgb([
                                            coloradjust(r01, r, c0, 5, 16),
                                            coloradjust(g01, g, c1, 5, 16),
                                            coloradjust(b01, b, c2, 5, 16)]));
            }

            // d0 = r - c0;
            // d1 = g - c1;
            // d2 = b - c2;

            image.put_pixel(x, y, image::Rgb([c0,c1,c2]));
        // image.put_pixel(x, y, pixel.clone());
        }
    }

    // write it out to a file
    image.save("output.png").unwrap();
}

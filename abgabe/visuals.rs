extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate conv;
use decoder::*;
use decoder_class::*;
use decoder_usecase::*;
use image::{GenericImage, ImageBuffer, Pixel};
use imageproc::definitions::{Clamp, Image};
use conv::ValueInto;
use std::f32;
use std::i32;
use std;
use imageproc::pixelops::weighted_sum;
use imageproc::drawing::*;
use imageproc::rect::Rect;
use imageproc::rect::Region;
use rusttype::{Font, Scale, point, PositionedGlyph};
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::*;
use std::collections::HashMap;
use std::ops::Mul;
use image::{Rgb,RgbImage};
use rusttype::{FontCollection};
use image::GenericImageView;

pub fn draw_text_mut<'a, I>(
    image: &'a mut I,
    color: I::Pixel,
    x: u32,
    y: u32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
) where
    I: GenericImage,
    <I::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;
                let image_x = gx + x as i32;
                let image_y = gy + y as i32;
                let image_width = image.width() as i32;
                let image_height = image.height() as i32;
                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = image.get_pixel(image_x as u32, image_y as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    image.put_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            })
        }
    }
}
pub fn draw_text<'a, I>(
    image: &'a mut I,
    color: I::Pixel,
    x: u32,
    y: u32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
) -> Image<I::Pixel>
    where
        I: GenericImage,
        <I::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
        I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0);
    draw_text_mut(&mut out, color, x, y, scale, font, text);
    out
}

pub fn erstelle_image()->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(1000, 1000);
    for a in 0..1000 {
        for b in 0..1000 {
            image.get_pixel_mut(a,b).data=[255,255,255];
        }
    }
    return(image)
}

pub fn main() {
}


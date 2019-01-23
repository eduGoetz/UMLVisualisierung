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
pub fn create_system_and_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,systemname:&str,vec_akteure: &Vec<Actor>)  -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut image=image;
    let mut systemname=systemname;
    let mut done_create=false;
    let mut vec_stelle=0;
    let mut anzahl=0;
    image=draw_systemborder(image,systemname);

    while !done_create {
        let mut position = vec_akteure[vec_stelle].id;
        image = draw_akteur(image, 0, position,"l");//0 muss da bleiben
        image = name_akteur(image, position, &vec_akteure[vec_stelle].name,"l");
        let mut relation =  vec_akteure[vec_stelle].extends_from;
        match relation {
            Some(relation) => image = draw_relationship_akteur(image, position,relation, "l"),
            None => (),
        }
        vec_stelle=vec_stelle+1;
        let mut vektor_inhalt=&vec_akteure[vec_stelle-1].has_use_case.len();
        anzahl=*vektor_inhalt;
        for x in 0..anzahl {
            let mut rel = &vec_akteure[1].has_use_case;
            image=draw_case_with_assoziation(image,rel[x],position,"","","l");
        }
        if vec_stelle==vec_akteure.iter().len(){
            done_create=true;
        }
    }

    let _ = image.save(Path::new("res/UML_visual_result.png")).unwrap();
    return(image);
}
pub fn create_cases(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,vec_cases: &Vec<UseCase>)-> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut image=image;
    let mut done_create=false;
    let mut vec_stelle=0;
    let mut name="";

    while !done_create {
        let mut place = vec_cases[vec_stelle].id;
        let mut extend = vec_cases[vec_stelle].is_extension_point;
        name= &vec_cases[vec_stelle].name;

        if extend==true{
            image=draw_case_extend(image,place)
        }
        image = draw_case(image, place);
        image=name_case(image,place,name);

        vec_stelle=vec_stelle+1;
        if vec_stelle==vec_cases.iter().len(){
            done_create=true;
        }
    }
    let _ = image.save(Path::new("res/UML_visual_result.png")).unwrap();
    return(image);
}
pub fn create_relations(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,vec: &Vec<UseCaseRelation>)->(image::ImageBuffer<Rgb<u8>, Vec<u8>>){
    let mut image=image;
    let mut done_create=false;
    let mut vec_stelle=0;
    for rel in vec{
        if let UseCaseRelationType::Include = rel.relation_type {
            image=draw_arrow(image,rel.from,rel.to,"<<include>>");
        }
            else{
                image=draw_arrow(image,rel.from,rel.to,"<<extend>>");
            }
    }
    let _ = image.save(Path::new("res/UML_visual_result.png")).unwrap();
    return(image);
}
    fn draw_systemborder(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, name: &str) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let font = Vec::from(include_bytes!("../res/fonts/DejaVuSans-Bold.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let schrift = Scale { x: 20.0, y: 20.0 };
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let rect = Rect::at(200, 10).of_size(600, 990);
        let mut image = image;
        let mut name = name;
        draw_hollow_rect_mut(&mut image, rect, draw_color);
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 400, 20, schrift, &font, name);
        return (image);
    }
    fn draw_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, ist_anzahl_guys: i32, soll_anzahl_guys: i32,side: &str) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut image = image;
        let mut ist_anzahl_guys = ist_anzahl_guys;
        let mut soll_anzahl_guys = soll_anzahl_guys;
        let mut side=side;
        let mut fertig = false;
        let mut x_anfang = 80;
        let mut head_anfang = 50;
        let mut body_anfang = 60;
        let mut arm_anfang = 70;
        let mut bein_anfang = 90;
        let mut bein_ende = 110;
        if side=="l" {
            while !fertig {
                if soll_anzahl_guys-1==ist_anzahl_guys {
                    draw_hollow_circle_mut(&mut image, (x_anfang as i32, head_anfang as i32), 10 as i32, draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, body_anfang as f32), (x_anfang as f32, bein_anfang as f32), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, arm_anfang as f32), ((90 as f32), (body_anfang as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, arm_anfang as f32), ((70 as f32), (body_anfang as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, bein_anfang as f32), ((90 as f32), (bein_ende as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, bein_anfang as f32), ((70 as f32), (bein_ende as f32)), draw_color);
                }
                head_anfang = head_anfang + 130;
                body_anfang = body_anfang + 130;
                arm_anfang = arm_anfang + 130;
                bein_anfang = bein_anfang + 130;
                bein_ende = bein_ende + 130;
                ist_anzahl_guys = ist_anzahl_guys + 1;
                if ist_anzahl_guys == 10 {
                    fertig = true;
                }
            }
        }else if side=="r"{
            x_anfang = 920;
            while !fertig {
                if soll_anzahl_guys-1==ist_anzahl_guys {
                    draw_hollow_circle_mut(&mut image, (x_anfang as i32, head_anfang as i32), 10 as i32, draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, body_anfang as f32), (x_anfang as f32, bein_anfang as f32), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, arm_anfang as f32), ((910 as f32), (body_anfang as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, arm_anfang as f32), ((930 as f32), (body_anfang as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, bein_anfang as f32), ((910 as f32), (bein_ende as f32)), draw_color);
                    draw_line_segment_mut(&mut image, (x_anfang as f32, bein_anfang as f32), ((930 as f32), (bein_ende as f32)), draw_color);
                }
                head_anfang = head_anfang + 130;
                body_anfang = body_anfang + 130;
                arm_anfang = arm_anfang + 130;
                bein_anfang = bein_anfang + 130;
                bein_ende = bein_ende + 130;
                ist_anzahl_guys = ist_anzahl_guys + 1;
                if ist_anzahl_guys == 10 {
                    fertig = true;
                }
            }
        }
        return (image);
    }
    fn name_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, person: i32, name: &str,side: &str) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let schrift = Scale { x: 10.0, y: 10.0 };
        let mut image = image;
        let mut bein_ende = 110;
        let mut person = person - 1;
        let mut name = name;
        let mut side=side;
        bein_ende = bein_ende + (130 * person);
        if side=="l" {
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 70, (bein_ende + 10) as u32, schrift, &font, name);
        }if side=="r"{
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 910, (bein_ende + 10) as u32, schrift, &font, name);
        }
        return (image);
    }
    fn draw_case(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,stelle: i32) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut image = image;
        let mut stelle=stelle;
        let mut tuple = get_case_koordinaten(stelle);
        let mut y_ellipse = tuple.1;
        let mut x_ellipse = tuple.0;
        draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32, y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
        return (image);
    }
    fn draw_case_with_assoziation(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,stelle: i32, person: i32, von: &str, nach: &str,side: &str) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut image = image;
        let font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let schrift = Scale { x: 10.0, y: 10.0 };
        let mut stelle = stelle;
        let mut tuple = get_case_koordinaten(stelle);
        let mut x_anfang = 80;
        let mut y_ellipse = tuple.1;
        let mut x_ellipse = tuple.0;
        let mut person = person - 1;
        let mut anfang = 75;
        anfang = anfang + (130 * person);
        if side == "l" {
            draw_line_segment_mut(&mut image, (x_anfang as f32, anfang as f32), ((x_ellipse - 50) as f32, y_ellipse as f32), draw_color);
            draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32, y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_anfang + 10) as u32, (anfang - 5) as u32, schrift, &font, von);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_ellipse - 60) as u32, (y_ellipse) as u32, schrift, &font, nach);
        } else if side == "r" {
            x_anfang = 920;
            draw_line_segment_mut(&mut image, (x_anfang as f32, anfang as f32), ((x_ellipse + 50) as f32, y_ellipse as f32), draw_color);
            draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32, y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_anfang - 10) as u32, (anfang - 5) as u32, schrift, &font, von);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_ellipse + 60) as u32, (y_ellipse) as u32, schrift, &font, nach);
        }
        return (image);
    }
    fn draw_relationship_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, person_von: i32, person_nach: i32,side:&str) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut image = image;
        let mut person_von = person_von - 1;
        let mut person_nach = person_nach - 1;
        let mut kopf_oben_x = 80;
        let mut kopf_oben_y = 50 - 10;
        let mut side=side;
        if side=="r" {kopf_oben_x = 920;}
        kopf_oben_y = kopf_oben_y + (130 * person_von);
        draw_line_segment_mut(&mut image, ((kopf_oben_x) as f32, (kopf_oben_y) as f32), ((kopf_oben_x) as f32, (kopf_oben_y - 30) as f32), draw_color);
        draw_line_segment_mut(&mut image, (kopf_oben_x as f32, (kopf_oben_y - 50) as f32), ((kopf_oben_x - 10) as f32, (kopf_oben_y - 30) as f32), draw_color);
        draw_line_segment_mut(&mut image, (kopf_oben_x as f32, (kopf_oben_y - 50) as f32), ((kopf_oben_x + 10) as f32, (kopf_oben_y - 30) as f32), draw_color);
        draw_line_segment_mut(&mut image, ((kopf_oben_x - 10) as f32, (kopf_oben_y - 30) as f32), ((kopf_oben_x + 10) as f32, (kopf_oben_y - 30) as f32), draw_color);
        return (image);
    }
    fn draw_case_extend(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, stelle: i32) -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut image = image;
        let mut stelle=stelle;
        let mut tuple = get_case_koordinaten(stelle);
        let mut y_ellipse = tuple.1;
        let mut x_ellipse = tuple.0;
        draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32, y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
        draw_line_segment_mut(&mut image, ((x_ellipse-30) as f32, (y_ellipse-20) as f32), ((x_ellipse+30) as f32, (y_ellipse-20) as f32), draw_color);
        return (image);
    }
    fn get_case_koordinaten(ende: i32) -> (i32,i32,i32,i32) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let mut fertig=false;
        let mut y_ellipse = 70;
        let mut x_ellipse = 250;
        let mut reihe=1;
        let mut anzahl=1;
        let mut ende=ende;
        let mut spalte=1;
        if ende==1{
            y_ellipse = 70;
            x_ellipse = 250;
        }
        else {
            while !fertig {
                x_ellipse = x_ellipse + 200;
                if spalte == 3 {
                    y_ellipse = y_ellipse + 70;
                    x_ellipse = 250;
                    spalte = 0;
                    reihe=reihe+1;
                }
                anzahl = anzahl + 1;
                spalte = spalte + 1;
                if anzahl == ende {
                    fertig = true;
                }
            }
        }
        return (x_ellipse,y_ellipse,spalte,reihe);
    }
    fn draw_arrow(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>, von: i32,nach: i32,beschriftung: &str)->(image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let draw_color = Rgb([0u8, 0u8, 0u8]);
        let draw_white = Rgb([255u8, 255u8, 255u8]);
        let font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let schrift = Scale { x: 13.0, y: 13.0 };
        let mut beschriftung=beschriftung;
        let mut image=image;
        let mut von=get_case_koordinaten(von);
        let mut nach=get_case_koordinaten(nach);
        let mut start_x=von.0;
        let mut ende_x=nach.0;
        let mut start_y=von.1;
        let mut ende_y=nach.1;
        let mut spalte_von=von.2;
        let mut spalte_nach=nach.2;
        let mut reihe_von=von.3;
        let mut reihe_nach=nach.3;
        let mut dazu=10;
        let mut dazu_y=10;
        let mut anderes=10;
        let mut richtung_h="";
        let mut richtung_w="";
        let mut richtung_pfeil="";
       if reihe_von==reihe_nach && spalte_von<=spalte_nach{
           start_x=start_x+50;
           ende_x=ende_x-50;
           dazu=10;
           richtung_pfeil="rechts";
       } else if reihe_von==reihe_nach && spalte_von>=spalte_nach{
            start_x=start_x-50;
            ende_x=ende_x+50;
            dazu=-10;
            richtung_pfeil="links";
        } else if spalte_von==spalte_nach && reihe_von<=reihe_nach{
            start_y=start_y+25;
            ende_y=ende_y-25;
            dazu=10;
            richtung_pfeil="unten";
        } else if spalte_von==spalte_nach && reihe_von>=reihe_nach{
                start_y=start_y-25;
                ende_y=ende_y+25;
                dazu=-10;
            richtung_pfeil="oben";
        } else if spalte_von < spalte_nach && reihe_von <reihe_nach{
            start_x=start_x+50;
            ende_x=ende_x-50;
            dazu=10;
            dazu_y=10;
            anderes=-10;
            richtung_h="unten";
            richtung_w="rechts";
            richtung_pfeil="rechts";
        } else if spalte_von < spalte_nach && reihe_von >reihe_nach{
            start_x=start_x+50;
            ende_x=ende_x-50;
            dazu=10;
            dazu_y=-10;
            anderes=10;
            richtung_h="oben";
            richtung_w="rechts";
            richtung_pfeil="rechts";
        } else if spalte_von > spalte_nach && reihe_von < reihe_nach{
            start_x=start_x-50;
            ende_x=ende_x+50;
            dazu=10;
            dazu_y=10;
            anderes=-10;
            richtung_h="unten";
            richtung_w="links";
            richtung_pfeil="links";
        } else if spalte_von > spalte_nach && reihe_von > reihe_nach{
                start_x=start_x-50;
                ende_x=ende_x+50;
                dazu=10;
                dazu_y=-10;
                anderes=10;
                richtung_h="oben";
                richtung_w="links";
                richtung_pfeil="links";
            }
        let mut zwischen_x=start_x;
        let mut zwischen_y=start_y;
        let mut fertig=false;
        while fertig==false {
            draw_line_segment_mut(&mut image, ((start_x) as f32, (start_y) as f32), ((zwischen_x) as f32, (zwischen_y) as f32), draw_color);
            if start_y==ende_y{
                start_x=zwischen_x;
                zwischen_x=zwischen_x+dazu;
                if spalte_von>spalte_nach{
                    if zwischen_x <= ende_x {
                        fertig = true;
                    }
                }else {
                    if zwischen_x >= ende_x {
                        fertig = true;
                    }
                }
                start_x=zwischen_x;
                zwischen_x=zwischen_x+dazu;
            }
            else if start_x==ende_x{
                start_y=zwischen_y;
                zwischen_y=zwischen_y+dazu;
                if reihe_von>reihe_nach{
                    if zwischen_y <= ende_y {
                        fertig = true;
                    }
                }else {
                    if zwischen_y >= ende_y {
                        fertig = true;
                    }
                }
                start_y=zwischen_y;
                zwischen_y=zwischen_y+dazu;
            }
            else{
                let mut tuple=zeiche_pfeil_richtung_eins(start_x,start_y,ende_x,ende_y,zwischen_x,zwischen_y,dazu,dazu_y,richtung_h,richtung_w);
                start_x=tuple.0;
                start_y=tuple.1;
                ende_x=tuple.2;
                ende_y=tuple.3;
                zwischen_x=tuple.4;
                zwischen_y=tuple.5;
                if zwischen_y >= ende_y && richtung_pfeil=="rechts"{
                    if zwischen_x>=ende_x{
                        fertig = true;
                    }
                } else if zwischen_y >= ende_y && richtung_pfeil=="links"{
                    if zwischen_x<=ende_x{
                        fertig = true;
                    }
                }
                let mut tuple=zeiche_pfeil_richtung_zwei(start_x,start_y,ende_x,ende_y,zwischen_x,zwischen_y,dazu,dazu_y,anderes,richtung_h,richtung_w);
                start_x=tuple.0;
                start_y=tuple.1;
                ende_x=tuple.2;
                ende_y=tuple.3;
                zwischen_x=tuple.4;
                zwischen_y=tuple.5;
            }
        }
        if richtung_pfeil=="links" {
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (zwischen_y) as f32), ((ende_x + 20) as f32, (zwischen_y-10) as f32), draw_color);
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (zwischen_y) as f32), ((ende_x + 20) as f32, (zwischen_y+10 ) as f32), draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]),(ende_x+30)as u32 , (ende_y+5) as u32, schrift, &font, beschriftung);
        }else if richtung_pfeil=="rechts"{
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (zwischen_y) as f32), ((ende_x - 20) as f32, (zwischen_y-10) as f32), draw_color);
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (zwischen_y) as f32), ((ende_x - 20) as f32, (zwischen_y+10 ) as f32), draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]),(ende_x-80)as u32 , (ende_y+5) as u32, schrift, &font, beschriftung);
        }else if richtung_pfeil=="oben"{
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (ende_y) as f32), ((ende_x + 10) as f32, (ende_y+10) as f32), draw_color);
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (ende_y) as f32), ((ende_x - 10) as f32, (ende_y+10 ) as f32), draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]),(ende_x+8)as u32 , (ende_y+10) as u32, schrift, &font, beschriftung);
        }else if richtung_pfeil=="unten"{
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (ende_y) as f32), ((ende_x + 10) as f32, (ende_y-10) as f32), draw_color);
            draw_line_segment_mut(&mut image, ((ende_x) as f32, (ende_y) as f32), ((ende_x - 10) as f32, (ende_y-10 ) as f32), draw_color);
            draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]),(ende_x+8)as u32 , (ende_y-10) as u32, schrift, &font, beschriftung);

        }
        return(image);
    }
    fn name_case(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,stelle: i32,text: &str)->(image::ImageBuffer<Rgb<u8>, Vec<u8>>){
        let mut image=image;
        let font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let schrift = Scale { x: 13.0, y: 13.0 };
        let mut stelle=stelle;
        let mut text=text;
        let mut tuple = get_case_koordinaten(stelle);
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]),(tuple.0-45)as u32 , (tuple.1-5) as u32, schrift, &font, text);
        return(image);
    }

fn zeiche_pfeil_richtung_eins(start_x: i32,start_y: i32,ende_x: i32,ende_y: i32,zwischen_x: i32,zwischen_y: i32,dazu: i32,dazu_y: i32,richtung_h: &str,richtung_w: &str)
    ->(i32,i32,i32,i32,i32,i32){
    let mut start_x=start_x;
    let mut start_y=start_y;
    let mut ende_x=ende_x;
    let mut ende_y=ende_y;
    let mut zwischen_x=zwischen_x;
    let mut zwischen_y=zwischen_y;
    let mut dazu=dazu;
    let mut dazu_y=dazu_y;
    let mut richtung_h=richtung_h;
    let mut richtung_w=richtung_w;

    //rechts oben
    if richtung_h=="oben" && richtung_w=="rechts"{
        if zwischen_y <= ende_y {
            zwischen_y = zwischen_y + dazu;
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
        } else if zwischen_x >= ende_x {
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        } else {
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }
    }
        //rechts unten
    else if richtung_h=="unten" && richtung_w=="rechts"{
        if zwischen_y >= ende_y {
            zwischen_y = zwischen_y + dazu;
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
        } else if zwischen_x >= ende_x {
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        } else {
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }
    }
        //links unten
    else if richtung_h=="unten" && richtung_w=="links"{
        if zwischen_y >= ende_y {
            zwischen_y = zwischen_y + dazu;
            start_x = zwischen_x;
            zwischen_x = zwischen_x - dazu;

        } else if zwischen_x <= ende_x {
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        } else {
            start_x = zwischen_x;
            zwischen_x = zwischen_x - dazu;
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }
    }
        //links oben
    else if richtung_h=="oben" && richtung_w=="links"{
        if zwischen_y <= ende_y {
            zwischen_y = zwischen_y + dazu;
            start_x = zwischen_x;
            zwischen_x = zwischen_x - dazu;
        } else if zwischen_x <= ende_x {
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        } else {
            start_x = zwischen_x;
            zwischen_x = zwischen_x - dazu;
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }

    }
    return(start_x,start_y,ende_x,ende_y,zwischen_x,zwischen_y)
}
fn zeiche_pfeil_richtung_zwei(start_x: i32,start_y: i32,ende_x: i32,ende_y: i32,zwischen_x: i32,zwischen_y: i32,dazu: i32,dazu_y: i32,anderes: i32,richtung_h: &str,richtung_w: &str)
                         ->(i32,i32,i32,i32,i32,i32){
    let mut start_x=start_x;
    let mut start_y=start_y;
    let mut ende_x=ende_x;
    let mut ende_y=ende_y;
    let mut zwischen_x=zwischen_x;
    let mut zwischen_y=zwischen_y;
    let mut dazu=dazu;
    let mut dazu_y=dazu_y;
    let mut anderes=anderes;
    let mut richtung_h=richtung_h;
    let mut richtung_w=richtung_w;
    //rechts oben
    if richtung_h=="oben" && richtung_w=="rechts"{
        if zwischen_y <= ende_y {
            zwischen_y = zwischen_y + anderes;
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
        }
            else if zwischen_x >= ende_x {
                start_y = zwischen_y;
                zwischen_y = zwischen_y + dazu_y;
            }
                else {
                    start_x=zwischen_x;
                    zwischen_x=zwischen_x+dazu;
                    start_y = zwischen_y;
                    zwischen_y = zwischen_y + dazu_y;
                }
    }
        //rechts unten
    else if richtung_h=="unten" && richtung_w=="rechts"{
        if zwischen_y >= ende_y {
            zwischen_y = zwischen_y + anderes;
            start_x = zwischen_x;
            zwischen_x = zwischen_x + dazu;
        }
        else if zwischen_x >= ende_x {
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }
        else {
            start_x=zwischen_x;
            zwischen_x=zwischen_x+dazu;
            start_y = zwischen_y;
            zwischen_y = zwischen_y + dazu_y;
        }
    }
        //links unten
        else if richtung_h=="unten" && richtung_w=="links"{
            if zwischen_y >= ende_y {
                zwischen_y = zwischen_y + anderes;
                start_x = zwischen_x;
                zwischen_x = zwischen_x - dazu;
            }
                else if zwischen_x <= ende_x {
                    start_y = zwischen_y;
                    zwischen_y = zwischen_y + dazu_y;
                }
                    else {
                        start_x=zwischen_x;
                        zwischen_x=zwischen_x-dazu;
                        start_y = zwischen_y;
                        zwischen_y = zwischen_y + dazu_y;
                    }
        }
            //oben links
    else if richtung_h=="oben" && richtung_w=="links"{
        if zwischen_y <= ende_y {
            zwischen_y = zwischen_y + anderes;
            start_x = zwischen_x;
            zwischen_x = zwischen_x - dazu;
        }
            else if zwischen_x <= ende_x {
                start_y = zwischen_y;
                zwischen_y = zwischen_y + dazu_y;
            }
                else {
                    start_x=zwischen_x;
                    zwischen_x=zwischen_x-dazu;
                    start_y = zwischen_y;
                    zwischen_y = zwischen_y + dazu_y;
                }
    }
    return(start_x,start_y,ende_x,ende_y,zwischen_x,zwischen_y)
}




pub fn klasse(ueberschrift: &str,klassentyp: &str,image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,file: &std::path::Path,anzahl: i32,vec_attribute: &Vec<Attribute>,vec_methode: &Vec<Method>)
              ->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){//,i32,i32){//,HashMap<u32, i32>) {

    let mut eingabe_ueberschift=ueberschrift;
    let mut klassentyp=klassentyp;
    //let mut eingabe_pfeil=pfeil;
    let mut file=file;
    let mut image=image;
    let mut anzahl=anzahl;
    //erster wert sagt die höhe aus in y
    //erster wert bei get pixel sagt die höhe in x
    let mut erster_wert=30;
    let mut zweiter_wert=180;
    let mut erster_wert_x=30;
    let mut zweiter_wert_x=180;
    let mut ab = erster_wert;
    //let mut anzahl_alt=anzahl_alt;

    let mut fertig=false;
    let mut done = false;
    let mut zeile=1;
    let mut pfeil_hoehe=zweiter_wert-erster_wert;
    //let mut eingabe = eingabe_pfeil;
    let mut pfeil_schr=pfeil_hoehe;
    //let mut pfeil_richtung=richtung;

    let mut vec_attribute=vec_attribute;
    let mut vec_methode=vec_methode;

    let mut tuple= zeichne_klasse(anzahl,"",image,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
    //let mut alte_werte=(tuple.1,tuple.2,tuple.3,tuple.4,anzahl);
    image=zeichne_schrift(tuple.0,eingabe_ueberschift,klassentyp,vec_attribute,vec_methode,tuple.1,tuple.2,tuple.3,anzahl);
    //image=zeichne_pfeil(image,"asso",von,nach);
    let mut anzahl_alt=koordinaten(anzahl);
    let  _ = image.save(file).unwrap();
    anzahl=anzahl+1;
    //let  _ = image.save("res/UML_visual_result.png").unwrap();
    return(image);//,anzahl,anzahl_alt.4);

}
fn zeichne_klasse(nummer: i32,eingabe: &str,image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,eins:u32,zwei:u32,drei:u32,vier:u32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >,u32,u32,u32,u32){
    let mut erster_wert=eins;
    let mut zweiter_wert=zwei;
    let mut erster_wert_x=drei;
    let mut zweiter_wert_x=vier;
    let mut done = false;
    let mut ab = erster_wert;
    let mut zeile=1;
    let mut fertig=false;
    let mut eingabe=eingabe;
    let mut anzahl=0;
    anzahl=nummer;
    let mut image=image;
    while !fertig {
        let mut tuple=koordinaten(anzahl);
        erster_wert=tuple.0;
        zweiter_wert=tuple.1;
        erster_wert_x=tuple.2;
        zweiter_wert_x=tuple.3;
        for d in erster_wert..zweiter_wert {
            image.get_pixel_mut(erster_wert_x,d).data=[0,0,0];
            image.get_pixel_mut(zweiter_wert_x,d).data=[0,0,0];
        }
        ab=erster_wert;
        while !done {
            for c in erster_wert_x..zweiter_wert_x{
                image.get_pixel_mut(c,ab).data=[0,0,0];
            }
            if zeile==2 || zeile == 3{
                ab=ab+65;
            } else {
                ab=ab+20;
            }
            zeile=zeile+1;
            if ab > zweiter_wert {
                zeile=1;
                done = true;
                fertig=true;
            }
        }
    }
    return (image,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
}
fn zeichne_schrift(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,name: &str,klassentyp: &str,vec_attribute: &Vec<Attribute>,vec_methode: &Vec<Method>,erster_wert: u32,zweiter_wert: u32,erster_wert_x: u32,anzahl: i32)->
(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
    let  font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
    let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let mut anzahl=anzahl;
    let  ueberschrift = Scale { x: 13.0 , y: 13.0 };
    let mut erster_wert=erster_wert;
    let mut ab = erster_wert;
    let mut erster_wert_x = erster_wert_x;
    let mut zweiter_wert=zweiter_wert;
    let mut eingabe_ueberschift=name;
    let mut done=false;
    let mut done_schrift = false;
    let mut zahl = 1;
    let mut image=image;
    let mut vektor_inhalt="".to_string();
    let mut vec_attribute=vec_attribute;
    let mut vec_methode=vec_methode;
    let mut vec_stelle=0;
    let mut sichtbarkeit_ueberschrift=klassentyp;
    let mut schreiben=100;
    if anzahl >= 5{
        schreiben=370
    }
    if anzahl >= 9 {
        schreiben=640;
    }
    if anzahl >= 13 {
        schreiben=910;
    }
    if sichtbarkeit_ueberschrift == "Paket"{
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, "Paket::");
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+42, ab+5, ueberschrift, &font, eingabe_ueberschift);
    }else if sichtbarkeit_ueberschrift=="Interface"{
        let mut ueberschrift = Scale { x: 12.0 , y: 12.0 };
        let  font = Vec::from(include_bytes!("../res/fonts/DejaVuSans-Oblique.ttf") as &[u8]);
        let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, ueberschrift, &font, "<<<Interface>>>");
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+10, ueberschrift, &font, eingabe_ueberschift);
    } else if sichtbarkeit_ueberschrift=="Abstrakt"{
        let mut ueberschrift = Scale { x: 12.0 , y: 12.0 };
        let  font = Vec::from(include_bytes!("../res/fonts/DejaVuSans-Oblique.ttf") as &[u8]);
        let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, ueberschrift, &font, "<<<Abstract>>>");
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+10, ueberschrift, &font, eingabe_ueberschift);
    } else if sichtbarkeit_ueberschrift=="Class"{
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, eingabe_ueberschift);
    }
    let mut attribute = Scale { x: 8.0, y: 8.0 };
    ab=ab+20;
    while !done_schrift {
        if ab<=schreiben{
            if vec_stelle < vec_attribute.iter().len(){
                vektor_inhalt=vec_attribute[vec_stelle].to_string();
                if vektor_inhalt.contains("static") {
                    let v: Vec<&str> = vektor_inhalt.split('/').collect();
                    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  v[0]);

                    for d in erster_wert_x+10..erster_wert_x+130{
                        image.get_pixel_mut(d,ab+8).data=[0,0,0];
                    }
                } else{
                    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  vec_attribute[vec_stelle].to_string().as_ref());
                }
                if vec_stelle <= vec_attribute.iter().len()-1{
                    vec_stelle=vec_stelle+1;
                }
            }

        } else if ab>schreiben{
            if vec_stelle < vec_methode.iter().len(){
                vektor_inhalt=vec_methode[vec_stelle].to_string();
                if vektor_inhalt.contains("static") {
                    let v: Vec<&str> = vektor_inhalt.split('/').collect();
                    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  v[0]);

                    for d in erster_wert_x+10..erster_wert_x+130{
                        image.get_pixel_mut(d,ab+8).data=[0,0,0];
                    }
                } else{
                    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  vec_methode[vec_stelle].to_string().as_ref());
                }
                if vec_stelle <= vec_attribute.iter().len(){
                    vec_stelle=vec_stelle+1;
                }
            }
        }
        if ab==schreiben{
            ab=schreiben+10;
            vec_stelle=0;
        }
        ab=ab+10;
        zahl=zahl+1;
        if ab == zweiter_wert {
            ab=erster_wert;
            zahl=1;
            done_schrift = true;
            schreiben=100;
        }
    }
    return(image);
}
pub fn zeichne_pfeil(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,file: &std::path::Path,pfeilart: &str,von: i32,nach: i32, multi_von: &str,multi_nach:&str)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
    let draw_color = Rgb([0u8, 0u8, 0u8]);
    let mut image=image;
    let mut von=von;
    let mut nach=nach;
    let mut file=file;
    let mut multi_von=multi_von;
    let mut multi_nach=multi_nach;
    let mut anzahl_alt=5;
    if von == 0 {anzahl_alt=0;}
    if nach ==0 {anzahl_alt=0;}
    let mut von=koordinaten(von);
    let mut zweiter_wert=von.1;
    let mut erster_wert=von.0;
    let mut zweiter_wert_x=von.3;
    let mut erster_wert_x=von.2;
    let mut pfeil_hoehe=erster_wert+70;
    let mut eingabe = pfeilart;
    let mut pfeil_schr=pfeil_hoehe;
    let mut c=pfeil_hoehe;
    let mut richtung="";
    let mut mitte_oberseite=0;
    let mut tuple=koordinaten(nach);
    mitte_oberseite=erster_wert_x+50;
    let mut mitte_unterseite=tuple.2+von.5;
    if eingabe == "asso" {
        if anzahl_alt>0{
            draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),((mitte_unterseite) as f32,tuple.1 as f32), draw_color);
        }
    }
    if eingabe == "ge_asso" {
        if anzahl_alt>0{
            draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite) as f32, (tuple.1+20) as f32),((mitte_unterseite) as f32,(tuple.1) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
        }
    }
    if eingabe == "ver" {
        if anzahl_alt>0{
            draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),((mitte_unterseite) as f32,(tuple.1+35) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite) as f32, (tuple.1+25) as f32),((mitte_unterseite) as f32,(tuple.1+35) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+25) as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
        }
    }
    if eingabe == "agg" {
        if anzahl_alt>0{
            draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite-10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite) as f32,(tuple.1) as f32),((mitte_unterseite-10) as f32,(tuple.1+10) as f32), draw_color);
        }
    }
    if eingabe == "kompo" {
        if anzahl_alt>0{
            draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite-10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
            draw_line_segment_mut(&mut image,((mitte_unterseite) as f32,(tuple.1) as f32),((mitte_unterseite-10) as f32,(tuple.1+10) as f32), draw_color);
            mitte_unterseite=tuple.2+von.5;
            let mut gemalt=false;
            let mut anfang=mitte_unterseite-10;
            let mut ende=mitte_unterseite+10;
            let mut c=tuple.1+10;
            let mut d=tuple.1+10;
            while !gemalt {
                for x in anfang..ende {
                    image.get_pixel_mut(x,c).data=[0,0,0];
                }
                for x in anfang..ende {
                    image.get_pixel_mut(x,d).data=[0,0,0];
                }
                anfang=anfang+1;
                ende=ende-1;
                c=c+1;
                d=d-1;
                if c==tuple.1+105{
                    gemalt = true;
                }
            }
        }
    }
    if eingabe == "abh" {
        let draw_color_white = Rgb([255u8, 255u8, 255u8]);
        draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
        draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
        if anzahl_alt>0{
            let mut s=0;
            let mut w=0;
            let mut anfang=tuple.1+20;
            let mut ende=mitte_unterseite;
            let mut ka=mitte_oberseite;//mitte_unterseite;
            let mut ak=von.0;
            for d in 1..2000{
                if s<=8 {
                    if ak != anfang{
                        ak=ak-1;
                    }
                    if ka > mitte_unterseite {
                        if ka != ende{
                            ka=ka-1;
                        }
                    } else if ka < mitte_unterseite {
                        if ka != ende{
                            ka=ka+1;
                        }
                    }
                } else if s>8{
                    image.get_pixel_mut(ka,ak).data=[0,0,0];
                    w=w+1;
                    if w==10 {
                        s=0;
                        w=0;
                    }
                }
                s=s+1;
            }

        }
    }
    if eingabe == "imple" {
        let draw_color_white = Rgb([255u8, 255u8, 255u8]);
        draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
        draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
        draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+25) as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
        if anzahl_alt>0{
            let mut s=0;
            let mut w=0;
            let mut anfang=tuple.1+20;
            let mut ende=mitte_unterseite;
            let mut ka=mitte_oberseite;//mitte_unterseite;
            let mut ak=von.0;
            for d in 1..2000{
                if s<=8 {
                    if ak != anfang{
                        ak=ak-1;
                    }
                    if ka > mitte_unterseite {
                        if ka != ende{
                            ka=ka-1;
                        }
                    } else if ka < mitte_unterseite {
                        if ka != ende{
                            ka=ka+1;
                        }
                    }
                } else if s>8{
                    image.get_pixel_mut(ka,ak).data=[0,0,0];
                    w=w+1;
                    if w==10 {
                        s=0;
                        w=0;
                    }
                }
                s=s+1;
            }
        }
    }
    eingabe="";
    let  font = Vec::from(include_bytes!("../res/fonts/DejaVuSans.ttf") as &[u8]);
    let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let mut multi = Scale { x: 10.0, y: 10.0 };
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), mitte_oberseite+5, (zweiter_wert-160), multi, &font, multi_von);
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), mitte_unterseite+5, (tuple.1), multi, &font, multi_nach);
    let  _ = image.save(file).unwrap();
    return(image);
}
fn koordinaten(anzahl:i32)->(u32,u32,u32,u32,i32,u32) {

    let mut erster_wert=0;
    let mut zweiter_wert=0;
    let mut erster_wert_x=0;
    let mut zweiter_wert_x=0;
    let mut mitte_unterseite=0;

    if anzahl == 1{
        erster_wert=30;
        zweiter_wert=180;
        erster_wert_x=30;
        zweiter_wert_x=180;
        mitte_unterseite=0;
        //eingabe="";
    }

    if anzahl == 2{
        erster_wert=30;
        zweiter_wert=180;
        erster_wert_x=280;
        zweiter_wert_x=430;
        mitte_unterseite=10;
        //eingabe="";
    }
    if anzahl == 3{
        erster_wert=30;
        zweiter_wert=180;
        erster_wert_x=530;
        zweiter_wert_x=680;
        mitte_unterseite=20;
        //eingabe="";
    }
    if anzahl == 4{
        erster_wert=30;
        zweiter_wert=180;
        erster_wert_x=780;
        zweiter_wert_x=930;
        mitte_unterseite=30;
        //eingabe="";
    }
    if anzahl == 5{
        erster_wert=300;
        zweiter_wert=450;
        erster_wert_x=780;
        zweiter_wert_x=930;
        mitte_unterseite=40;
        //eingabe="";
    }
    if anzahl == 6{
        erster_wert=300;
        zweiter_wert=450;
        erster_wert_x=530;
        zweiter_wert_x=680;
        mitte_unterseite=50;
        //eingabe="";
    }

    if anzahl == 7{
        erster_wert=300;
        zweiter_wert=450;
        erster_wert_x=280;
        zweiter_wert_x=430;
        mitte_unterseite=60;
        //eingabe="";
    }
    if anzahl == 8{
        erster_wert=300;
        zweiter_wert=450;
        erster_wert_x=30;
        zweiter_wert_x=180;
        mitte_unterseite=70;
        //eingabe="";
    }
    if anzahl == 9{
        erster_wert=570;
        zweiter_wert=720;
        erster_wert_x=30;
        zweiter_wert_x=180;
        mitte_unterseite=80;
        //eingabe="";
    }
    if anzahl == 10{
        erster_wert=570;
        zweiter_wert=720;
        erster_wert_x=280;
        zweiter_wert_x=430;
        mitte_unterseite=90;
        //eingabe="";
    }
    if anzahl == 11{
        erster_wert=570;
        zweiter_wert=720;
        erster_wert_x=530;
        zweiter_wert_x=680;
        mitte_unterseite=100;
        //eingabe="";
    }
    if anzahl == 12{
        erster_wert=570;
        zweiter_wert=720;
        erster_wert_x=780;
        zweiter_wert_x=930;
        mitte_unterseite=110;
        //eingabe="";
    }
    if anzahl == 13{
        erster_wert=840;
        zweiter_wert=990;
        erster_wert_x=630;
        zweiter_wert_x=930;
        mitte_unterseite=120;
        //eingabe="";
    }
    if anzahl == 14{
        erster_wert=840;
        zweiter_wert=990;
        erster_wert_x=230;
        zweiter_wert_x=530;
        mitte_unterseite=130;
        //eingabe="";
    }
    if anzahl == 15{
        erster_wert=840;
        zweiter_wert=990;
        erster_wert_x=1;
        zweiter_wert_x=130;
        mitte_unterseite=140;
        //eingabe="";
    }
    return(erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x,anzahl,mitte_unterseite);
}
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
use visuals::*;


pub fn create_system_and_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,systemname:&str,vec_akteure: Vec<&str>)  -> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut image=image;
    let mut systemname=systemname;
    let mut vec_akteure=vec_akteure;
    let mut done_create=false;
    let mut vektor_inhalt="".to_string();
    let mut vec_stelle=0;

    image=draw_systemborder(image,systemname);
    while !done_create {
        vektor_inhalt=vec_akteure[vec_stelle].to_string();
        let mut v: Vec<&str> = vektor_inhalt.split(';').collect();
        let mut position = v[0].parse::<i32>().unwrap();
        image = draw_akteur(image, 0, position,v[1]);//0 muss da bleiben
        image = name_akteur(image, position, v[2],v[1]);
        let mut relation = v[3].parse::<bool>().unwrap();
        if relation==true {
            let mut to_position = v[4].parse::<i32>().unwrap();
            image = draw_relationship_akteur(image, position, to_position,v[1]);
        }
        vec_stelle=vec_stelle+1;
        if vec_stelle==vec_akteure.iter().len(){
            done_create=true;
        }
    }
    let _ = image.save(Path::new("res/UML_visual_result.png")).unwrap();
    return(image);
}
pub fn create_cases(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,vec_cases: Vec<&str>)-> (image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut image=image;
    let mut vec_cases=vec_cases;
    let mut done_create=false;
    let mut vektor_inhalt="".to_string();
    let mut vec_stelle=0;

    while !done_create {
        vektor_inhalt=vec_cases[vec_stelle].to_string();
        let mut v: Vec<&str> = vektor_inhalt.split(';').collect();
        let mut place = v[0].parse::<i32>().unwrap();
        let mut extend = v[4].parse::<bool>().unwrap();
        if v[1]=="normal" {
            image = draw_case(image, place);
        }else if v[1]=="asso" {
            let mut person = v[2].parse::<i32>().unwrap();
            image = draw_case_with_assoziation(image, place, person, v[8], v[9], v[3]);
        }else if v[1]=="extend" {
            image = draw_case_extend(image, place);
        }
        let mut include = v[6].parse::<bool>().unwrap();
        if extend==true {
            let mut from_case = v[5].parse::<i32>().unwrap();
            image = draw_arrow(image, from_case, place, "'Extend'");
        }
            else if include==true{
                let mut to_case = v[7].parse::<i32>().unwrap();
                image = draw_arrow(image, place, to_case, "'Include'");
            }
        vec_stelle=vec_stelle+1;
        if vec_stelle==vec_cases.iter().len(){
            done_create=true;
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
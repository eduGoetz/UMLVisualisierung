extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate conv;
//mod decoder;

//use decoder;

use image::{GenericImage, ImageBuffer, Pixel};
use imageproc::definitions::{Clamp, Image};
use conv::ValueInto;
use std::f32;
use std::i32;

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
/// Draws colored text on an image in place. `scale` is augmented font scaling on both the x and y axis (in pixels). Note that this function *does not* support newlines, you must do this manually

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

/// Draws colored text on an image in place. `scale` is augmented font scaling on both the x and y axis (in pixels). Note that this function *does not* support newlines, you must do this manually
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
	let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(1000, 1000);//image = RgbImage::new(200, 200);
	
	//bild weiß machen
	for a in 0..1000 {
		for b in 0..1000 {
		image.get_pixel_mut(a,b).data=[255,255,255];
		}
	}
	return(image)
}




fn main() {
let mut diagramm="USE";
if diagramm=="USE" {

	let mut image=erstelle_image();
	let file = Path::new("res/Use-Case.png");

	
	let mut x_anfang=80;
	let mut y_ellipse=300;
	let mut x_ellipse=300;
	let mut person=2;
	let mut von="1";
	let mut nach="2";
	
	
	image=zeichne_case(image,y_ellipse,x_ellipse);	

	y_ellipse=400;
	x_ellipse=400;

	
	image=zeichne_systemgrenze(image,"Überschrift");
	image=zeichne_case_mit_assoziation(image,x_anfang,y_ellipse,x_ellipse,person,von,nach);
	image=zeichne_akteur(image,0,5);
	image=beschrifte_akteur(image,3,"katze");
	image=zeichne_beziehung_akteur(image,3,2);
	
	let  _ = image.save(file).unwrap();

pub fn zeichne_systemgrenze(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,name: &str)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let  font = Vec::from(include_bytes!("DejaVuSans-Bold.ttf") as &[u8]);
	let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let  schrift = Scale { x: 20.0 , y: 20.0 };
	let draw_color = Rgb([0u8, 0u8, 0u8]);
	let rect = Rect::at(200,10).of_size(790, 990);	
	let mut image=image;
	let mut name=name;
	
	draw_hollow_rect_mut(&mut image, rect, draw_color);
	draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 500, 20, schrift, &font, name);

	return(image);

}	
pub fn zeichne_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,ist_anzahl_guys: i32,soll_anzahl_guys: i32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let draw_color = Rgb([0u8, 0u8, 0u8]);
	let mut image=image;
	let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let  schrift = Scale { x: 10.0 , y: 10.0 };
	
	
	let mut ist_anzahl_guys=ist_anzahl_guys;
	let mut soll_anzahl_guys=soll_anzahl_guys;	
	let mut fertig = false;

	
	let mut x_anfang=80;
	let mut head_anfang=50;
	let mut body_anfang=60;
	let mut arm_anfang=70;
	let mut bein_anfang=90;
	let mut bein_ende=110;
	
	//personen erstellen
	while !fertig {
			//head
			draw_hollow_circle_mut(&mut image, (x_anfang as i32, head_anfang as i32), 10 as i32, draw_color) ;
			//body
			draw_line_segment_mut(&mut image,(x_anfang as f32,  body_anfang as f32),( x_anfang as f32, bein_anfang as f32), draw_color);	
			//arm rechts
			draw_line_segment_mut(&mut image,(x_anfang as f32,  arm_anfang as f32),( (90 as f32),( body_anfang as f32)), draw_color);	
			//arm links
			draw_line_segment_mut(&mut image,(x_anfang as f32,  arm_anfang as f32),( (70 as f32),( body_anfang as f32)), draw_color);	
			//bein rechts
			draw_line_segment_mut(&mut image,(x_anfang as f32,  bein_anfang as f32),( (90 as f32),( bein_ende as f32)), draw_color);	
			//bein links
			draw_line_segment_mut(&mut image,(x_anfang as f32,  bein_anfang as f32),( (70 as f32),( bein_ende as f32)), draw_color);		
			//name
			//draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 70, bein_ende+10, schrift, &font, "dute");

			
				head_anfang=head_anfang+130;
				body_anfang=body_anfang+130;
				arm_anfang=arm_anfang+130;
				bein_anfang=bein_anfang+130;
				bein_ende=bein_ende+130;
				ist_anzahl_guys=ist_anzahl_guys+1;
			
			
		if  ist_anzahl_guys==soll_anzahl_guys{
			fertig = true;
		}
	}
	
	
	return(image)	
}
pub fn beschrifte_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,person: i32,name: &str)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let  schrift = Scale { x: 10.0 , y: 10.0 };
	let mut image=image;

	let mut bein_ende=110;
	let mut person=person-1;
	let mut name=name;
	
	bein_ende=bein_ende+(130*person);
	//name
	draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 70, (bein_ende+10) as u32, schrift, &font, name);

	return(image)	
}
pub fn zeichne_case(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,y_ellipse: i32,x_ellipse: i32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let draw_color = Rgb([0u8, 0u8, 0u8]);
	let mut image=image;
	
	let mut y_ellipse=y_ellipse;
	let mut x_ellipse=x_ellipse;
	
	draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32,y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
	return(image)
}
pub fn zeichne_case_mit_assoziation(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,x_anfang: i32,y_ellipse: i32,x_ellipse: i32,person: i32,von: &str,nach: &str)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let draw_color = Rgb([0u8, 0u8, 0u8]);
	let mut image=image;
	let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let  schrift = Scale { x: 10.0 , y: 10.0 };
	
	let mut person=person-1;

	let mut anfang=75;
	
	anfang=anfang+(130*person);
	draw_line_segment_mut(&mut image,(x_anfang as f32,  anfang as f32),( (x_ellipse-50) as f32, y_ellipse as f32), draw_color);		
	draw_hollow_ellipse_mut(&mut image, (x_ellipse as i32,y_ellipse as i32), 50 as i32, 25 as i32, draw_color);
	//kardinalität von (akteur)
	draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_anfang+10) as u32, (anfang-5) as u32, schrift, &font, von);
	//kardinalität nach (use case)
	draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), (x_ellipse-60) as u32, (y_ellipse) as u32, schrift, &font, nach);
	return(image)

}				
pub fn zeichne_beziehung_akteur(image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,person_von: i32,person_nach: i32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
	let draw_color = Rgb([0u8, 0u8, 0u8]);
	let mut image=image;
	let mut person_von=person_von-1;
	let mut person_nach=person_nach-1;
	
	let mut kopf_oben_x=80; 
	let mut kopf_oben_y=50-10;
	
	
	kopf_oben_y=kopf_oben_y+(130*person_von);
	
	draw_line_segment_mut(&mut image,((kopf_oben_x) as f32, (kopf_oben_y) as f32),((kopf_oben_x) as f32,(kopf_oben_y-30) as f32), draw_color);
	//schräge rechts
	draw_line_segment_mut(&mut image,(kopf_oben_x as f32, (kopf_oben_y-50) as f32),((kopf_oben_x-10) as f32,(kopf_oben_y-30) as f32), draw_color);
	//Schräge links
	draw_line_segment_mut(&mut image,(kopf_oben_x as f32, (kopf_oben_y-50) as f32),((kopf_oben_x+10) as f32,(kopf_oben_y-30) as f32), draw_color);
	//verbindungsstrich
	draw_line_segment_mut(&mut image,((kopf_oben_x-10) as f32,(kopf_oben_y-30) as f32),((kopf_oben_x+10) as f32,(kopf_oben_y-30) as f32), draw_color);
	return(image);
}								
}								

if diagramm=="UML" {						
			pub fn klasse(ueberschrift: &str,klassentyp: &str,image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,file: &std::path::Path,anzahl: i32,vec_attribute: Vec<&str>,vec_methode: Vec<&str>)
			->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
				
					let mut eingabe_ueberschift=ueberschrift;
					let mut klassentyp=klassentyp;
					let mut file=file;
					let mut image=image;	
					let mut anzahl=anzahl;
					//erster wert sagt die höhe aus in y
					//erster wert bei get pixel sagt die höhe in x
					let mut erster_wert=30;
					let mut zweiter_wert=180;
					let mut erster_wert_x=30;
					let mut zweiter_wert_x=180;
					
					
					let mut vec_attribute=vec_attribute;
					let mut vec_methode=vec_methode;

					let mut tuple= zeichne_klasse(anzahl,"",image,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
					image=zeichne_schrift(tuple.0,eingabe_ueberschift,klassentyp,vec_attribute,vec_methode,tuple.1,tuple.2,tuple.3,anzahl);
					let mut anzahl_alt=koordinaten(anzahl);
					let  _ = image.save(file).unwrap();
					anzahl=anzahl+1;
					return(image);

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
					let mut anzahl=0;
					anzahl=nummer;
					let mut image=image;	
					

					
					while !fertig { 	

						let mut tuple=koordinaten(anzahl);
						erster_wert=tuple.0;
						zweiter_wert=tuple.1;
						erster_wert_x=tuple.2;
						zweiter_wert_x=tuple.3;


						//tabelle zeichnen
						//senkrechte striche
						for d in erster_wert..zweiter_wert {
							image.get_pixel_mut(erster_wert_x,d).data=[0,0,0];
							image.get_pixel_mut(zweiter_wert_x,d).data=[0,0,0];

						}
						
						
						//waagerechte sriche
						ab=erster_wert;
						while !done {
							for c in erster_wert_x..zweiter_wert_x{
								image.get_pixel_mut(c,ab).data=[0,0,0];
							}
							if zeile==2 || zeile == 3{
								ab=ab+65;
							}
							else {
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


			fn zeichne_schrift(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,name: &str,klassentyp: &str,vec_attribute: Vec<&str>,vec_methode: Vec<&str>,erster_wert: u32,zweiter_wert: u32,erster_wert_x: u32,anzahl: i32)->
			(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
						
						
						let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
						let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
						
						let mut anzahl=anzahl;
						let  ueberschrift = Scale { x: 13.0 , y: 13.0 };
						let mut erster_wert=erster_wert;
						let mut ab = erster_wert;
						let mut erster_wert_x = erster_wert_x;
						let mut zweiter_wert=zweiter_wert;

						//beschriftung vom bild
						let mut done=false;
						let mut done_schrift = false; 
						let mut zahl = 1;
						let mut eingabe_ueberschift=name;
						let mut image=image;	
						let mut vektor_inhalt="";
						let mut vec_attribute=vec_attribute;
						let mut vec_methode=vec_methode;

						let mut vec_stelle=0;
						let mut sichtbarkeit_ueberschrift=klassentyp;
						let mut sichtbarkeit_attribut="";
						let mut sichtbarkeit_methode="";
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
						}//abstract
						else if sichtbarkeit_ueberschrift=="Interface"{
							let mut ueberschrift = Scale { x: 12.0 , y: 12.0 };
							let  font = Vec::from(include_bytes!("DejaVuSans-Oblique.ttf") as &[u8]);
							let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
							draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, ueberschrift, &font, "<<<Interface>>>");
							draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+10, ueberschrift, &font, eingabe_ueberschift);
						}
						else if sichtbarkeit_ueberschrift=="Abstrakt"{
							let mut ueberschrift = Scale { x: 12.0 , y: 12.0 };
							let  font = Vec::from(include_bytes!("DejaVuSans-Oblique.ttf") as &[u8]);
							let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
							draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, ueberschrift, &font, "<<<Abstract>>>");
							draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+10, ueberschrift, &font, eingabe_ueberschift);
						}
						else if sichtbarkeit_ueberschrift==""{
							draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, eingabe_ueberschift);
						}
						let mut attribute = Scale { x: 8.0, y: 8.0 };
						
						ab=ab+20;
						//auf 6 attribute  
						while !done_schrift {
							if ab<=schreiben{							
								if vec_stelle < vec_attribute.iter().len(){
									
									vektor_inhalt=vec_attribute[vec_stelle];

									if vektor_inhalt.contains("static") {
										let v: Vec<&str> = vektor_inhalt.split('/').collect();
										draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  v[0]);
										
										for d in erster_wert_x+10..erster_wert_x+130{
												image.get_pixel_mut(d,ab+8).data=[0,0,0];
										}
									}
									else{
										draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  vec_attribute[vec_stelle]);
										}					
									if vec_stelle <= vec_attribute.iter().len()-1{
										vec_stelle=vec_stelle+1;
									}
								}

							}
							else if ab>schreiben{
								if vec_stelle < vec_methode.iter().len(){
								
									vektor_inhalt=vec_methode[vec_stelle];

									if vektor_inhalt.contains("static") {
										let v: Vec<&str> = vektor_inhalt.split('/').collect();
										draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  v[0]);
										
										for d in erster_wert_x+10..erster_wert_x+130{
												image.get_pixel_mut(d,ab+8).data=[0,0,0];
										}
									}
									else{
										draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  vec_methode[vec_stelle]);
										}															
									if vec_stelle <= vec_attribute.iter().len()-1{
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
				
			pub fn zeichne_pfeil(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,file: &std::path::Path,pfeilart: &str,von: i32,nach: i32,multi_von: &str,multi_nach:&str)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
						
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
							
							
							

							let mut eingabe = pfeilart;

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
							
							//gerichtete assoziation
							if eingabe == "ge_asso" {
								if anzahl_alt>0{
								//für rechts
								draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
								draw_line_segment_mut(&mut image,((mitte_unterseite) as f32, (tuple.1+20) as f32),((mitte_unterseite) as f32,(tuple.1) as f32), draw_color);
									//schräge rechts
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
									//Schräge links
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);			
								}	

							}
							//Vererbung
							if eingabe == "ver" {
							//rechts
								if anzahl_alt>0{				
								//für rechts
									draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),((mitte_unterseite) as f32,(tuple.1+35) as f32), draw_color);
									draw_line_segment_mut(&mut image,((mitte_unterseite) as f32, (tuple.1+25) as f32),((mitte_unterseite) as f32,(tuple.1+35) as f32), draw_color);
									//schräge rechts
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
									//Schräge links
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
									//verbindungsstrich
									draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+25) as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);		
									
								}					
								
							}
							
							//Aggregation
							if eingabe == "agg" {		
								if anzahl_alt>0{
								//für rechts
									draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,(tuple.1+20) as f32), draw_color);
									//schräge rechts oben
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
									//Schräge links oben
									draw_line_segment_mut(&mut image,((mitte_unterseite-10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);	
									draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+10) as f32),((mitte_unterseite) as f32,(tuple.1+20) as f32), draw_color);
									draw_line_segment_mut(&mut image,((mitte_unterseite) as f32,(tuple.1) as f32),((mitte_unterseite-10) as f32,(tuple.1+10) as f32), draw_color);
									
								}					
							
							}
									
							//Komposition
							if eingabe == "kompo" {
							
								if anzahl_alt>0{
								//für rechts
									draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,(tuple.1+20) as f32), draw_color);
									//schräge rechts oben
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
									//Schräge links oben
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
							//Implementierung
							if eingabe == "abh" {
							let draw_color_white = Rgb([255u8, 255u8, 255u8]);


									//schräge rechts
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
									//Schräge links
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);	
													
													

							
								if anzahl_alt>0{
									let mut s=0;
									let mut w=0;
									println!("x:{}y:{}",mitte_unterseite,tuple.1+20);
									let mut anfang=tuple.1+20;
									let mut ende=mitte_unterseite;
									let mut ka=mitte_oberseite;
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
												}
												else if ka < mitte_unterseite {
													if ka != ende{
														ka=ka+1;
													}
												}
											}
											else if s>8{
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
								
							//abhängigkeit

							if eingabe == "imple" {
							
							let draw_color_white = Rgb([255u8, 255u8, 255u8]);


									//schräge rechts
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);
									//Schräge links
									draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+25) as f32), draw_color);
									//verbindungsstrich
									draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+25) as f32),((mitte_unterseite-10) as f32,(tuple.1+25) as f32), draw_color);	

							
								if anzahl_alt>0{
									let mut s=0;
									let mut w=0;
									println!("x:{}y:{}",mitte_unterseite,tuple.1+20);
									let mut anfang=tuple.1+20;
									let mut ende=mitte_unterseite;
									let mut ka=mitte_oberseite;//mitte_unterseite;
									let mut ak=von.0;
									for d in 1..2000{
										if s<=8 {
												//println!("{}",ka);
												//image.get_pixel_mut(ka,ak).data=[0,0,0];
												if ak != anfang{
													ak=ak-1;
												}
												if ka > mitte_unterseite {
													if ka != ende{
														ka=ka-1;
													}
												}
												else if ka < mitte_unterseite {
													if ka != ende{
														ka=ka+1;
													}
												}
											}
											else if s>8{
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
				richtung="";
				eingabe="";
				
				let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
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
						/*if anzahl == 16{
							erster_wert=840;
							zweiter_wert=990;
							erster_wert_x=30;
							zweiter_wert_x=180;
							mitte_unterseite=150;			
							//eingabe="";
						}	*/					
						return(erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x,anzahl,mitte_unterseite);

			}
	}
}

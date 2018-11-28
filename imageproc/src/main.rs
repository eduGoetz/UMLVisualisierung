extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate conv;

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



fn main() {
	/*let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(1000, 1000);//image = RgbImage::new(200, 200);
	//png datei wird erstellt
    image.save("UML.png").unwrap();
	let arg = "UML.png";
    let path = Path::new(&arg);//der pfad zum bild welcher direkt in der cmd angegeben wird
	
	//bild weiß machen falls es nicht weiß ist
	for a in 0..1000 {
		for b in 0..1000 {
		image.get_pixel_mut(a,b).data=[255,255,255];
		}
	}	*/
	
	

	/*    let arg = if env::args().count() == 2 {
            env::args().nth(1).unwrap()
        } else {
            panic!("Please enter a target file path")
        };

    let path = Path::new(&arg);*/
	
	
	/*    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };*/

	
	//let image = image::open("test.jpg").unwrap();
	//let image = image::open(&Path::new(&file)).unwrap();
	

	let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(1000, 1000);//image = RgbImage::new(200, 200);
	//png datei wird erstellt
    image.save("daten/UML.png").unwrap();
	let arg = "daten/UML.png";
    let path = Path::new(&arg);//der pfad zum bild welcher direkt in der cmd angegeben wird
	
	//bild weiß machen falls es nicht weiß ist
	for a in 0..1000 {
		for b in 0..1000 {
		image.get_pixel_mut(a,b).data=[255,255,255];
		}
	}
	
	

				let mut vec_attributea = Vec::new();
				vec_attributea.push("Das");
				vec_attributea.push("hier");
				vec_attributea.push("sind");
				vec_attributea.push("Attribute");
				
				
				let mut vec_methodea = Vec::new();
				vec_methodea.push("Das");
				vec_methodea.push("hier");
				vec_methodea.push("sind");
				vec_methodea.push("Methoden");
	
	
	
				let mut vec_attributeb = Vec::new();
				vec_attributeb.push("Attribute");
				vec_attributeb.push("sind");
				vec_attributeb.push("hier");
				vec_attributeb.push("Das");
				
				
				let mut vec_methodeb = Vec::new();
				vec_methodeb.push("methode");
				vec_methodeb.push("sind");
				vec_methodeb.push("hier");
				vec_methodeb.push("das");
				
				
				let mut vec_attributec = Vec::new();
				vec_attributec.push("fdsa");
				vec_attributec.push("fdsa");
				vec_attributec.push("hfgdr");
				vec_attributec.push("Dgfdsas");
				
				let mut vec_methodec = Vec::new();
				vec_methodec.push("tes");
				vec_methodec.push("fds");
				vec_methodec.push("fds");
				vec_methodec.push("asdf");
				
				
	
	//es müssen der klassenname,Pfeilart,das bild,der pfad zum bild,die zahl bei welcher klasse er ist,vektor für attribute,vektor für methoden
	 let mut tuple=klasse("eins","",image,path,1,vec_attributea,vec_methodea,"",0);
	tuple=klasse("zwei","asso",tuple.0,path,2,vec_attributeb,vec_methodeb,"",tuple.2);
	//tuple=klasse("drei","ge_asso",tuple.0,path,4,vec_attributec,vec_methodec,"",tuple.2);

}
	
pub fn klasse(ueberschrift: &str,pfeil: &str,image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,file: &std::path::Path,anzahl: i32,vec_attribute: Vec<&str>,vec_methode: Vec<&str>,richtung: &str,anzahl_alt: i32)
->(image::ImageBuffer<Rgb<u8>, Vec<u8> >,i32,i32){//,HashMap<u32, i32>) {
	
		let mut eingabe_ueberschift=ueberschrift;
		let mut eingabe_pfeil=pfeil;
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
		let mut anzahl_alt=anzahl_alt;
		
		let mut fertig=false;
		let mut done = false; 
		let mut zeile=1;
		let mut pfeil_hoehe=zweiter_wert-erster_wert;
		let mut eingabe = eingabe_pfeil;
		let mut pfeil_schr=pfeil_hoehe;
		let mut pfeil_richtung=richtung; 		
		
		let mut vec_attribute=vec_attribute;
		let mut vec_methode=vec_methode;
		
		let mut tuple= zeichne_klasse(anzahl,"",image,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
		//let mut alte_werte=(tuple.1,tuple.2,tuple.3,tuple.4,anzahl);
		image=zeichne_schrift(tuple.0,eingabe_ueberschift,vec_attribute,vec_methode,tuple.1,tuple.2,tuple.3,anzahl);
		image=zeichne_pfeil(image,eingabe,tuple.1,tuple.2,tuple.3,tuple.4,anzahl,pfeil_richtung,anzahl_alt);
		let mut anzahl_alt=koordinaten(anzahl);
		let  _ = image.save(file).unwrap();
		anzahl=anzahl+1;

		return(image,anzahl,anzahl_alt.4);
	/*for (key, value) in &tuple.5 {
    println!("ausgabe der hashmap key:{} value: {}", key, value);
	}*/
	//println!("{}",tuple.5);
		//return(image,anzahl,tuple.1,tuple.2,tuple.3,tuple.4);//,tuple.5);
}

fn zeichne_klasse(nummer: i32,eingabe: &str,image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,eins:u32,zwei:u32,drei:u32,vier:u32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >,u32,u32,u32,u32){
		let mut erster_wert=eins;
		let mut zweiter_wert=zwei;
		let mut erster_wert_x=drei;
		let mut zweiter_wert_x=vier;
		//let mut mitte_unterseite_x=zweiter_wert_x-erster_wert_x;
		//let mut mitte_unterseite_y;
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
				//	mitte_unterseite_y=ab;
					zeile=1;
					done = true;
					fertig=true;
				}
			}
	}


	return (image,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
	
}


fn zeichne_schrift(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,name: &str,vec_attribute: Vec<&str>,vec_methode: Vec<&str>,erster_wert: u32,zweiter_wert: u32,erster_wert_x: u32,anzahl: i32)->
(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
			
			
			let  font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
			let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
			
			let mut anzahl=anzahl;
			let  ueberschrift = Scale { x: 13.0 , y: 13.0 };
			let mut erster_wert=erster_wert;
			let mut ab = erster_wert;
			let mut erster_wert_x = erster_wert_x;
			let mut zweiter_wert=zweiter_wert;
			let mut eingabe_ueberschift=name;
			//let mut eingabe_methode=methoden;
			//let mut eingabe_attribut=attribute;
			//beschriftung vom bild
			let mut done=false;
			let mut done_schrift = false; 
			let mut zahl = 1;
			
			let mut image=image;	
			
			let mut vec_attribute=vec_attribute;
			let mut vec_methode=vec_methode;

			let mut vec_stelle=0;
			//muss noch übergeben werden
			let mut sichtbarkeit_ueberschrift="";//sichtbarkeit_ueberschrift;
			let mut sichtbarkeit_attribut="";//sichtbarkeit_attribut;
			let mut sichtbarkeit_methode="";//sichtbarkeit_methode;
			let mut schreiben=100;
			 
			if anzahl >= 4{
				schreiben=370
			}
			if anzahl >= 8 {
				schreiben=640;
			}
			if anzahl >= 8 {
				schreiben=910;
			}
			
			
			if sichtbarkeit_ueberschrift == "Paket"{
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, "Paket::");	
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+42, ab+5, ueberschrift, &font, eingabe_ueberschift);
			}//abstract
			else if sichtbarkeit_ueberschrift=="Kursiv"{
				let  font = Vec::from(include_bytes!("DejaVuSans-Oblique.ttf") as &[u8]);
				let  font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, eingabe_ueberschift);
			}
			else if sichtbarkeit_ueberschrift=="Abstrakt"{
				let mut ueberschrift = Scale { x: 11.0 , y: 11.0 };
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, ueberschrift, &font, "<<<Abstract>>>");
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+10, ueberschrift, &font, eingabe_ueberschift);
			}
			else if sichtbarkeit_ueberschrift==""{
				draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font, eingabe_ueberschift);
			}
			let mut attribute = Scale { x: 10.0, y: 10.0 };
			
			ab=ab+20;
			//auf 6 attribute  
			while !done_schrift {
				if ab<=schreiben{
					if vec_stelle < vec_attribute.iter().len(){
						draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  sichtbarkeit_attribut); 
						draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+10, ab, attribute, &font,  vec_attribute[vec_stelle]); 
						//println!("attribute:{}",vec_attribute[vec_stelle]);
						if vec_stelle <= vec_attribute.iter().len()-1{
							vec_stelle=vec_stelle+1;
						}
					}

				}
				else if ab>schreiben{
					if vec_stelle < vec_attribute.iter().len(){
						draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab, attribute, &font,  sichtbarkeit_methode); 
						draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+10, ab, attribute, &font,  vec_methode[vec_stelle]); 
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
	
fn zeichne_pfeil(image: image::ImageBuffer<Rgb<u8>, Vec<u8> >,pfeilart: &str,erster_wert: u32,zweiter_wert: u32,erster_wert_x: u32,zweiter_wert_x: u32,anzahl: i32,richtung: &str,anzahl_alt: i32)->(image::ImageBuffer<Rgb<u8>, Vec<u8> >){
			
				let draw_color = Rgb([0u8, 0u8, 0u8]);
				let mut anzahl=anzahl;
				let mut image=image;
				
				let mut zweiter_wert=zweiter_wert;
				let mut erster_wert=erster_wert;
				let mut zweiter_wert_x=zweiter_wert_x;
				let mut erster_wert_x=erster_wert_x;
				
				let mut pfeil_hoehe=erster_wert+70;
				//eingabe = pfeil art
				let mut eingabe = pfeilart;
				let mut pfeil_schr=pfeil_hoehe; 

				let mut c=pfeil_hoehe;
				let mut richtung = richtung;
				
				let mut mitte_oberseite=0;
				if anzahl_alt==0{
					richtung="";
				 erster_wert=500;
					}
					

				let mut anzahl_alt=anzahl_alt;
				
				let mut tuple=koordinaten(anzahl_alt);
				
				
				mitte_oberseite=erster_wert_x+50;

				let mut mitte_unterseite=tuple.2+75;
				 
				/*if anzahl <3{
					richtung="rechts";
				}
				
				else if anzahl == 3 || anzahl== 7 {
					richtung ="unten";
				}
				else if anzahl>=4 && anzahl<7{
					richtung="links";
				}
				else if anzahl >= 8{
					richtung="rechts";
				}*/
				//Pfeile
				//assoziation
				if eingabe == "asso" {
					if anzahl_alt>0{
						draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);	
					}						
					if richtung == "rechts"{
						//for sagt länge an
						//beim zeichnen erster wert länge und zweiter höhe
						for d in zweiter_wert_x..zweiter_wert_x+100{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
						//draw_line_segment_mut(&mut image,(350.0,300.0),(30.0,180.0), draw_color);
						
					}
					if richtung=="unten"{
						for d in zweiter_wert..zweiter_wert+120{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}
					}
					if richtung == "links"{
						for d in erster_wert_x-100..erster_wert_x{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
					}
				}
				
				//gerichtete assoziation
				if eingabe == "ge_asso" {
					if anzahl_alt>0{
					//für rechts
						draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
						//schräge rechts
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-20) as f32,(tuple.1+20) as f32), draw_color);
						//Schräge links
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-50) as f32,(tuple.1+10) as f32), draw_color);		
					}	
					if richtung == "rechts"{
						//gerade linie
						/*for d in zweiter_wert_x..zweiter_wert_x+100{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
						erster_wert=zweiter_wert_x+100;*/
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//Strich oben
						pfeil_schr=pfeil_hoehe;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						pfeil_schr=erster_wert+70;
						pfeil_hoehe=erster_wert+70;
					}			
					
					if richtung=="unten"{
						for d in zweiter_wert..zweiter_wert+120{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich links
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+21,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	

					}
					
					if richtung=="links"{
						for d in erster_wert_x-100..erster_wert_x{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
						erster_wert=zweiter_wert_x+100;
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
				
				
					}
				}
				//Vererbung
				if eingabe == "ver" {
				//rechts
					if anzahl_alt>0{				
						draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
						//schräge rechts
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-20) as f32,(tuple.1+20) as f32), draw_color);
						//Schräge links
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-50) as f32,(tuple.1+10) as f32), draw_color);	
						//verbindungsstrich
						draw_line_segment_mut(&mut image,((mitte_unterseite-20) as f32,(tuple.1+20) as f32),((mitte_unterseite-50) as f32,(tuple.1+10) as f32), draw_color);		
						
					}					
				
				
				
					if richtung == "rechts"{
						pfeil_schr=pfeil_hoehe;

						//gerade linie
						for d in zweiter_wert_x..zweiter_wert_x+80{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						//Strich oben
						pfeil_schr=pfeil_hoehe;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}		c=c-1;
			
						//verbindungsstrich
						for d in pfeil_schr+2..pfeil_schr+41 {
							image.get_pixel_mut(erster_wert-20,d).data=[0,0,0];
						}
					
					}
					
					if richtung=="unten"{
						for d in zweiter_wert..zweiter_wert+101{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich rechts
						for d in (zweiter_wert_x-95..zweiter_wert_x-75).rev() {
							image.get_pixel_mut(d+20,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	

						//verbindungsstrich
						for d in zweiter_wert_x-93..zweiter_wert_x-56{
						image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
						}
						
					}
					
					
					if richtung=="links"{
						for d in erster_wert_x-100..erster_wert_x{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
						erster_wert=zweiter_wert_x+100;
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						//verbindungsstrich
						for d in pfeil_schr-20..pfeil_schr+20 {
							image.get_pixel_mut(erster_wert-310,d).data=[0,0,0];
						}
				
					}
					
				}
				
				//Aggregation
				if eingabe == "agg" {
				
					if anzahl_alt>0{
					//für rechts
						draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
						//schräge rechts oben
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
						//Schräge links oben
						draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+10) as f32),((mitte_unterseite-5) as f32,(tuple.1+20) as f32), draw_color);	
						draw_line_segment_mut(&mut image,((mitte_unterseite-5) as f32,(tuple.1+20) as f32),((mitte_unterseite-15) as f32,(tuple.1+7) as f32), draw_color);	
						
					}					
				
				
				
				
					if richtung == "rechts"{
					
						//gerade linie
						for d in zweiter_wert_x..zweiter_wert_x+60{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
							
						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						pfeil_schr=pfeil_hoehe;
						erster_wert=zweiter_wert_x+100;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						
						pfeil_schr=erster_wert+70;
						pfeil_hoehe=erster_wert+70;
					}
					if richtung =="unten"{
						for d in zweiter_wert..zweiter_wert+83{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich rechts unten
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+21,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links unten
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						pfeil_schr=pfeil_schr-18;
						//strich oben links
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}
						

						//strich oben rechts
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+21,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}						
					}
					if richtung=="links"{
						//gerade linie
						for d in erster_wert_x-60..erster_wert_x{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}

						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						pfeil_schr=pfeil_hoehe;
						erster_wert=zweiter_wert_x+100;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						
						pfeil_schr=erster_wert+70;
						pfeil_hoehe=erster_wert+70;						

						
						
						
					}
				}
						
				//Komposition
				if eingabe == "kompo" {
				
					if anzahl_alt>0{
					//für rechts
						draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
						//schräge rechts oben
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite+10) as f32,(tuple.1+10) as f32), draw_color);
						//Schräge links oben
						draw_line_segment_mut(&mut image,((mitte_unterseite+10) as f32,(tuple.1+10) as f32),((mitte_unterseite-5) as f32,(tuple.1+20) as f32), draw_color);	
						draw_line_segment_mut(&mut image,((mitte_unterseite-5) as f32,(tuple.1+20) as f32),((mitte_unterseite-15) as f32,(tuple.1+7) as f32), draw_color);
						
						
						let rect = Rect::at(mitte_unterseite as i32, tuple.1 as i32).of_size(20, 10);
						draw_filled_rect_mut(&mut image,rect,draw_color);
					}						
				
					if richtung == "rechts"{

						//gerade linie
						for d in zweiter_wert_x..zweiter_wert_x+100{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}
							
						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						pfeil_schr=pfeil_hoehe;
						erster_wert=zweiter_wert_x+100;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						

						//ausmalen
						let mut gemalt=false;
						let mut d=120;
						c=80;
						let mut anfang=erster_wert-20;
						let mut ende=erster_wert-19;
						
						while !gemalt {
							//oben
							for x in anfang..ende {
								image.get_pixel_mut(x,c).data=[0,0,0];
							}
							//unten
							for z in anfang..ende {
								image.get_pixel_mut(z,d).data=[0,0,0];
							}
							c=c+1;
							d=d-1;
							anfang=anfang-1;
							ende=ende+1;
							if c == 100 {
								gemalt = true;
							}
						}
						pfeil_schr=100;
						pfeil_hoehe=100;
					}
					
					if richtung =="unten"{
						for d in zweiter_wert..zweiter_wert+120{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich rechts unten
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+21,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links unten
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						pfeil_schr=pfeil_schr-18;
						//strich oben links
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}
						

						//strich oben rechts
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+21,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	

						//ausmalen
						let mut gemalt=false;
						let mut d=erster_wert+270;
						let mut c=erster_wert+270-37;
						let mut anfang=855;
						let mut ende=856;
						
						while !gemalt {
							//oben
							for x in anfang..ende {
								image.get_pixel_mut(x,c).data=[0,0,0];
							}
							//unten
							for z in anfang..ende {
								image.get_pixel_mut(z,d).data=[0,0,0];
							}
							c=c+1;
							d=d-1;
							anfang=anfang-1;
							ende=ende+1;
							if c == erster_wert+270-17 {
								gemalt = true;
							}
						}
					}
					if richtung == "links"{
						//gerade linie
						for d in erster_wert_x-100..erster_wert_x{
						image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
						}

						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						pfeil_schr=pfeil_hoehe;
						erster_wert=zweiter_wert_x+100;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						
						pfeil_schr=erster_wert+70;
						pfeil_hoehe=erster_wert+70;	
					
					
					
					
						//ausmalen
						/*let mut gemalt=false;
						let mut d=390;
						let mut c=350;
						let mut anfang=erster_wert-330;
						let mut ende=erster_wert-330;
						
						while !gemalt {
							//oben
							for x in anfang..ende {
								image.get_pixel_mut(x,c).data=[0,0,0];
							}
							//unten
							for z in anfang..ende {
								image.get_pixel_mut(z,d).data=[0,0,0];
							}
							c=c+1;
							d=d-1;
							anfang=anfang-1;
							ende=ende+1;
							if c == 370 {
								gemalt = true;
							}
						}*/
						//pfeil_schr=erster_wert+70;
						//pfeil_hoehe=erster_wert+70;
					}

					
				}
				//Implementierung
				if eingabe == "imple" {
					if anzahl_alt>0{
					//für rechts
						let mut s=0;
						let mut w=0;
						for d in mitte_oberseite..mitte_unterseite {
							if s<=5 {
								draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
								}
								else if s>5{
									//draw_line_segment_mut(&mut image,(mitte_oberseite as f32, (zweiter_wert-150) as f32),(mitte_unterseite as f32,tuple.1 as f32), draw_color);
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;								
						}	
						//schräge rechts
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-20) as f32,(tuple.1+20) as f32), draw_color);
						//Schräge links
						draw_line_segment_mut(&mut image,(mitte_unterseite as f32, tuple.1 as f32),((mitte_unterseite-50) as f32,(tuple.1+10) as f32), draw_color);		
					}					
				
				
					if richtung == "rechts"{

						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert_x..zweiter_wert_x+80{
								if s<=5 {
								image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
								}
								else if s>5{
									image.get_pixel_mut(d,pfeil_hoehe).data=[255,255,255];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}	

					
						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						//Strich oben
						pfeil_schr=pfeil_hoehe;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}		c=c-1;
			
						//verbindungsstrich
						for d in pfeil_schr+2..pfeil_schr+41 {
							image.get_pixel_mut(erster_wert-20,d).data=[0,0,0];
						}
						pfeil_schr=100;
						pfeil_hoehe=100;
					}
					
					if richtung=="unten"{
						/*for d in zweiter_wert..zweiter_wert+101{
						image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
						}*/

						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert..zweiter_wert+101{
								if s<=5 {
								image.get_pixel_mut(zweiter_wert_x-75,d).data=[255,255,255];
								}
								else if s>5{
									image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}	
						
						
						
						
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich rechts
						for d in (zweiter_wert_x-95..zweiter_wert_x-75).rev() {
							image.get_pixel_mut(d+20,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	

						//verbindungsstrich
						for d in zweiter_wert_x-93..zweiter_wert_x-56{
						image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
						}
						
					}
					
					if richtung =="links"{
					
					
						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert_x-220..zweiter_wert_x-150{
								if s<=5 {
								image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
								}
								else if s>5{
									image.get_pixel_mut(d,pfeil_hoehe).data=[255,255,255];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}						
					
					
						erster_wert=zweiter_wert_x+100;
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						
						//verbindungsstrich
						for d in pfeil_schr-20..pfeil_schr+20 {
							image.get_pixel_mut(erster_wert-310,d).data=[0,0,0];
						}
					
					
					}
					
				}
					
				//abhängigkeit

				if eingabe == "abh" {
					if richtung == "rechts"{
						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert_x..zweiter_wert_x+80{
								if s<=5 {
								image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
								}
								else if s>5{
									image.get_pixel_mut(d,pfeil_hoehe).data=[255,255,255];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}	
					
						erster_wert=zweiter_wert_x+100;
					//strich unten
					//bei for erster wert wo er anfängt in x und zweiter wo er aufhört
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//Strich oben
						pfeil_schr=pfeil_hoehe;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	
						pfeil_schr=100;
						pfeil_hoehe=100;
					}
					
					
					if richtung =="unten"{

						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert..zweiter_wert+120{
								if s<=5 {
								image.get_pixel_mut(zweiter_wert_x-75,d).data=[255,255,255];
								}
								else if s>5{
									image.get_pixel_mut(zweiter_wert_x-75,d).data=[0,0,0];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}	
						pfeil_schr=erster_wert+270-19;
						pfeil_hoehe=erster_wert+270;
						//strich rechts
						for d in (zweiter_wert_x-95..zweiter_wert_x-75).rev() {
							image.get_pixel_mut(d+20,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}

						//strich links
						pfeil_schr=pfeil_hoehe;
						for d in (zweiter_wert_x-95..zweiter_wert_x-76).rev() {
							image.get_pixel_mut(d+2,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}	


						
					}
						

					if richtung =="links"{
					
					
						let mut s=0;
						let mut w=0;
							//gerade linie

							for d in zweiter_wert_x-240..zweiter_wert_x-150{
								if s<=5 {
								image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
								}
								else if s>5{
									image.get_pixel_mut(d,pfeil_hoehe).data=[255,255,255];
									w=w+1;
									if w==10 {
										s=0;
										w=0;
									}
								}
								s=s+1;
							}						
					
					
						erster_wert=zweiter_wert_x+100;
							
						//strich unten links
						erster_wert=zweiter_wert_x+80;
						pfeil_schr=pfeil_hoehe+20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr-1;
						}
						
						//strich oben links
						pfeil_schr=pfeil_hoehe-20;
						for d in (erster_wert-20..erster_wert+1).rev() {
							image.get_pixel_mut(d-310,pfeil_schr).data=[0,0,0];
							pfeil_schr=pfeil_schr+1;
						}	
						

					
					
					}

						
				}
	richtung="";
	eingabe="";
	return(image);
}

fn koordinaten(anzahl:i32)->(u32,u32,u32,u32,i32) {

				let mut erster_wert=30;
				let mut zweiter_wert=180;
				let mut erster_wert_x=30;
				let mut zweiter_wert_x=180;

			if anzahl == 1{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=30;
				zweiter_wert_x=180;

				//eingabe="";
			}		
		
			if anzahl == 2{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=280;
				zweiter_wert_x=430;
				
				//eingabe="";
			}
			if anzahl == 3{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=530;
				zweiter_wert_x=680;
			
				//eingabe="";
			}
			if anzahl == 4{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=780;
				zweiter_wert_x=930;
		
				//eingabe="";	
			}
			if anzahl == 5{
				erster_wert=300;
				zweiter_wert=450;
				erster_wert_x=780;
				zweiter_wert_x=930;
				
				//eingabe="";
			}
			if anzahl == 6{
				erster_wert=300;
				zweiter_wert=450;
				erster_wert_x=530;
				zweiter_wert_x=680;
				
				//eingabe="";
			}

			if anzahl == 7{
				erster_wert=300;
				zweiter_wert=450;
				erster_wert_x=280;
				zweiter_wert_x=430;
			
				//eingabe="";
			}
			if anzahl == 8{
				erster_wert=300;
				zweiter_wert=450;
				erster_wert_x=30;
				zweiter_wert_x=180;
			
				//eingabe="";
			}
			if anzahl == 9{
				erster_wert=570;
				zweiter_wert=720;
				erster_wert_x=30;
				zweiter_wert_x=180;
				
				//eingabe="";
			}
			if anzahl == 10{
				erster_wert=570;
				zweiter_wert=720;
				erster_wert_x=280;
				zweiter_wert_x=430;
			
				//eingabe="";
			}
			if anzahl == 11{
				erster_wert=570;
				zweiter_wert=720;
				erster_wert_x=530;
				zweiter_wert_x=680;
			
				//eingabe="";
			}			
			if anzahl == 12{
				erster_wert=570;
				zweiter_wert=720;
				erster_wert_x=780;
				zweiter_wert_x=930;
			
				//eingabe="";
			}			
			if anzahl == 13{
				erster_wert=840;
				zweiter_wert=990;
				erster_wert_x=780;
				zweiter_wert_x=930;
			
				//eingabe="";
			}			
			if anzahl == 14{
				erster_wert=840;
				zweiter_wert=990;
				erster_wert_x=530;
				zweiter_wert_x=680;
			
				//eingabe="";
			}			
			if anzahl == 15{
				erster_wert=840;
				zweiter_wert=990;
				erster_wert_x=280;
				zweiter_wert_x=430;
			
				//eingabe="";
			}			
			if anzahl == 16{
				erster_wert=840;
				zweiter_wert=990;
				erster_wert_x=30;
				zweiter_wert_x=180;
			
				//eingabe="";
			}						
//println!("Koordinatien:: anzahl:{}, erste:{}, zweite:{}, erste x:{}, zweite x:{}",anzahl,erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x);
			return(erster_wert,zweiter_wert,erster_wert_x,zweiter_wert_x,anzahl);

}


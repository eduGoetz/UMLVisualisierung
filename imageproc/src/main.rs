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
use rusttype::{Font, Scale, point, PositionedGlyph};

use std::path::Path;
use std::env;
use std::io::*;
use image::{Rgb};//, RgbImage};
use rusttype::{FontCollection};

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

	klasse("klassenname","agg","attributwert","methodenwert");

}
	
fn klasse(ueberschrift: &str,pfeil: &str,attribut: &str, methode: &str) {	



    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(1000, 1000);//image = RgbImage::new(200, 200);
	//png datei wird erstellt
    image.save("UML.png").unwrap();
	let arg = "UML.png";
    let path = Path::new(&arg);//der pfad zum bild welcher direkt in der cmd angegeben wird

	//hier tabelle zeichnen
	//bild weiß machen falls es nicht weiß ist
	for a in 0..1000 {
		for b in 0..1000 {
		image.get_pixel_mut(a,b).data=[255,255,255];
		}
	}

		
	
	println!("kontrollausgabe = {},{},{},{}", ueberschrift,pfeil,attribut,methode);
		let mut eingabe_ueberschift=ueberschrift;
		let mut eingabe_pfeil=pfeil;
		let mut eingabe_attribut=attribut;
		let mut eingabe_methode=methode;
	

		let mut anzahl=0;
		//erster wert sagt die höhe aus in y
		//erster wert bei get pixel sagt die höhe in x
		let mut erster_wert=30;
		let mut zweiter_wert=180;
		let mut erster_wert_x=30;
		let mut zweiter_wert_x=180;
		let mut fertig=false;
		let mut ab = erster_wert;
		let mut done = false; 
		let mut zeile=1;
		//pfeile
		let mut pfeil_hoehe=100;
		//eingabe = pfeil art
		let mut eingabe = eingabe_pfeil;
		let mut pfeil_schr=100; 
		//let mut ende_linie=0;
		
		while !fertig { 	
			if anzahl == 1{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=280;
				zweiter_wert_x=430;
				eingabe="";
			}
			if anzahl == 2{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=530;
				zweiter_wert_x=680;
				eingabe="";
			}
			if anzahl == 3{
				erster_wert=30;
				zweiter_wert=180;
				erster_wert_x=780;
				zweiter_wert_x=930;
			}
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
				}
			}


			
			let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
			let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
		
			let mut ueberschrift = Scale { x: 15.0 , y: 15.0 };
			
			ab=erster_wert;
			//beschriftung vom bild
			let mut done_schrift = false; 
			let mut zahl = 1;
			draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+5, ab+5, ueberschrift, &font,  eingabe_ueberschift);
			let mut attribute = Scale { x: 10.0, y: 10.0 };
			
			ab=50;
			//auf 6 attribute  
			while !done_schrift {
				if ab<=100{
					draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+10, ab, attribute, &font,  eingabe_methode);
				}
				else if ab>100{
					draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), erster_wert_x+10, ab, attribute, &font,  eingabe_attribut);
				}
				
				if ab==100{
					ab=110;
				}
				ab=ab+10;
				zahl=zahl+1;
				if ab == zweiter_wert {
					ab=erster_wert;
					zahl=1;
					done_schrift = true;
				}
			}
			anzahl=anzahl+1;
			done=false;
			//beschränkt auf 4 klassen
			if anzahl==4{
				fertig=true;
			}
		


				let mut c=100;
				//Pfeile
				//assoziation
				if eingabe == "asso" {
					//for sagt länge an
					//beim zeichnen erster wert länge und zweiter höhe
					for d in zweiter_wert_x..zweiter_wert_x+100{
					image.get_pixel_mut(d,pfeil_hoehe).data=[0,0,0];
					}
				} 	
				
				//gerichtete assoziation
				if eingabe == "ge_asso" {
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

					//Strich oben
					pfeil_schr=pfeil_hoehe;
					for d in (erster_wert-20..erster_wert+1).rev() {
						image.get_pixel_mut(d,pfeil_schr).data=[0,0,0];
						pfeil_schr=pfeil_schr-1;
					}	
					pfeil_schr=100;
					pfeil_hoehe=100;
				}
				
				//Vererbung
				if eingabe == "ver" {
					pfeil_schr=pfeil_hoehe;

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
				
				//Aggregation
				if eingabe == "agg" {
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
					
					pfeil_schr=100;
					pfeil_hoehe=100;
				}
					
						
				//Komposition
				if eingabe == "kompo" {
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
				//Implementierung
				if eingabe == "imple" {
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
					
				//abhängigkeit

				if eingabe == "abh" {
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
			eingabe="";
		}
		let _ = image.save(path).unwrap();
	}


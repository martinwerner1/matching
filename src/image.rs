#![allow(warnings)]
use image::{RgbImage, Rgb, RgbaImage, Rgba};
use imageproc::drawing::{draw_text_mut,text_size};
use image::DynamicImage::ImageRgb8;
use image::io::Reader;
use ab_glyph::{FontRef,PxScale};
pub fn img_bp_matrix(bp_matrix:&Vec<Vec<bool>>,path:String)->RgbImage{
	let w_:u32=777;
	let h_:u32=777;
	let n:usize=bp_matrix.len();
	let pw:usize=(w_ as f64/n as f64) as usize;
	let ph:usize=(h_ as f64/n as f64) as usize;
	let w:u32=(pw*n) as u32;
	let h:u32=(ph*n) as u32;
	let mut img:RgbImage=RgbImage::new(w,h);
	img=fill_pic(&img);
	for i in 0..bp_matrix.len(){
		let y_i:usize=ph*i;
		for j in 0..bp_matrix[i].len(){
			let x_j:usize=pw*j;
			let mut val:u8=0;
			if bp_matrix[i][j]{
				val=1;
			}
			let col_ij=Rgb([0,(120)*val,0]);
			for k in 0..ph{
				let y:u32=(y_i+k) as u32;
				for m in 0..pw{
					let x:u32=(x_j+m) as u32;
					img.put_pixel(x,y,col_ij);
				}
			}
		}
	}
	img.save(path);
	img
}
pub fn img_bp_matrix_lines(bp_matrix:&Vec<Vec<bool>>,path:String)->RgbImage{
	let w_:u32=777;
	let h_:u32=777;
	let n:usize=bp_matrix.len();
	let pw:usize=(w_ as f64/n as f64) as usize;
	let ph:usize=(h_ as f64/n as f64) as usize;
	let w:u32=(pw*n) as u32;
	let h:u32=(ph*n) as u32;
	let color_line=Rgb([255,0,0]);
	let mut img:RgbImage=RgbImage::new(w,h);
	img=fill_pic(&img);
	for i in 0..bp_matrix.len(){
		let y_i:usize=ph*i;
		for j in 0..bp_matrix[i].len(){
			let x_j:usize=pw*j;
			let mut val:u8=0;
			if bp_matrix[i][j]{
				val=1;
			}
			let col_ij=Rgb([0,(120)*val,0]);
			for k in 0..ph{
				let y:u32=(y_i+k) as u32;
				for m in 0..pw{
					let x:u32=(x_j+m) as u32;
					img.put_pixel(x,y,col_ij);
				}
			}
		}
	}
	let nsqrt:usize=n.isqrt();
	for k in 0..nsqrt{
		let line_x:u32=((k+1)*(pw*nsqrt)) as u32;
		let line_y:u32=((k+1)*(ph*nsqrt)) as u32;
		if line_x<w{
			for m in 0..h{
				img.put_pixel(line_x,m,color_line);
			}
		}
		if line_y<h{
			for m in 0..w{
				img.put_pixel(m,line_y,color_line);
			}
		}
	}
	img.save(path);
	img
}
pub fn fill_pic(rgb:&RgbImage)->RgbImage{
	let (w,h) = rgb.dimensions();
	let mut new = RgbImage::new(w,h);
	let col = Rgb([255,255,255]);
	for i in 0..w{
		for j in 0..h{
			new.put_pixel(i,j,col);
		}
	}
	return new;
}

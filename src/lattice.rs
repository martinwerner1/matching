#![allow(warnings)]
use std::rc::Rc;
use std::cell::RefCell;
use super::helper::{IOClass,Generate};
pub struct lattice{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	adj:Vec<Vec<[usize;2]>>,
	bp:Vec<Vec<bool>>,
}
impl lattice{
	pub fn init()->Self{
		let m_:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
		let w_:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
		let adj_:Vec<Vec<[usize;2]>>=IOClass::create_adj(&m_,&w_);
		let bp_:Vec<Vec<bool>>=IOClass::bp_matrix(&adj_);
		Self{
			m:m_,
			w:w_,
			adj:adj_,
			bp:bp_,
		}
	}	
	pub fn print_all_pref(&self){
		self.print_pref(0);
		self.print_pref(1);
	}
	pub fn print_pref(&self,mode:usize){
		let mut pref:Vec<Vec<usize>>=self.m.clone();
		let pattern:Vec<&str>=vec!["Men","Women"];
		let mut pref_pattern:&str=pattern[0];
		if mode==1{
			pref=self.w.clone();
			pref_pattern=pattern[mode];
		}
		else{
			if mode !=0{
				println!("Problem in lattice::print_pref()!");
			}
		}
		println!("{}",pref_pattern);
		for i in 0..pref.len(){
			println!("{}:\t{:?}",i,pref[i]);
		}
	}
	pub fn get_bp(&self)->Vec<Vec<bool>>{
		return self.bp.clone();
	}
	pub fn get_adj(&self)->Vec<Vec<[usize;2]>>{
		return self.adj.clone();
	}
	pub fn show_hash(){
		let hash=Generate::generate_hashvalue(7);
		println!("HASH: {}",hash);
	}
}














struct chain{
	hash:usize,
	head:usize,
	//subchain:Option<chain>,
	subchain:Option<Rc<RefCell<chain>>>,
}
impl chain{
	fn init(){
		
	}
}

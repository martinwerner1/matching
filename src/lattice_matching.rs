#![allow(warnings)]
use super::helper::{IOClass};
use std::collections::{HashSet,HashMap};
pub const N:usize=2;
pub fn test(){
	println!("LATTICE MATCHING");
	let mut ltc:lattice_matching=lattice_matching::new();
	ltc.init();
	let init:Vec<[usize;N]>=ltc.create_init_vec();
	//let first_layer:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=ltc.create_1st_layer2(&init);
	let first_layer:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=ltc.create_1st_layer2(&init);
	let matches=ltc.run3(&first_layer,1);

	println!("LATTICE MATCHES:");
	for i in 0..matches.len(){
		println!("{:?}",matches[i]);
	}
	/*
	let mut lattice:lattice_matching=lattice_matching::new();
	lattice.init();
	let init:Vec<[usize;N]>=lattice.create_init_vec();
	//let first_layer:HashMap<[usize;N],Vec<[usize;N]>>=lattice.create_1st_layer(&init);
	let first_layer:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=lattice.create_1st_layer2(&init);
	println!("first layer:\n{:?}",first_layer);
	// iter() is also possible!!!!!!!
	//for el in first_layer.iter(){
	for el in first_layer.keys(){
		//println!("{:?} --> {:?}",el,first_layer[&el]);
		println!("{:?} --> {:?}",el,first_layer[el]);
	}
	let matches=lattice.run3(&first_layer,1);
	*/
	
}

struct lattice_matching{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	adj:Vec<Vec<[usize;2]>>,
	stabm:Vec<Vec<bool>>,
}
impl lattice_matching{
	fn new()->Self{
		Self{
			m:vec![],
			w:vec![],
			adj:vec![],
			stabm:vec![],
		}
	}
	fn init(&mut self){
		self.m=IOClass::read_txt("pref/m.txt".to_string());
		self.w=IOClass::read_txt("pref/w.txt".to_string());
		self.adj=IOClass::create_adj(&self.m,&self.w);
		self.stabm=IOClass::bp_matrix(&self.adj);
	}	
	fn create_init_vec(&self)->Vec<[usize;N]>{
		let mut init_vec:Vec<[usize;N]>=vec![];
		let n:usize=self.m.len();
		for i in 0..self.m.len(){
			let mut init_i:Vec<usize>=vec![];
			for j in 0..self.w.len(){
				//init_vec.push(vec![i,j]);
				// commented!
				//init_vec.push([i,j]);
			}
		}
		init_vec
	}
	fn check_compatibility(a:&[usize;N],b:&[usize;N])->bool{
		for i in 0..N{
			if a[i]==b[i]{
				return false;
			}
		}
		true
	}
	fn check_stability(&self,a:&[usize;N],b:&[usize;N])->bool{
		let nm:usize=self.m.len();
		let nw:usize=self.w.len();
		let i:usize=a[0]*nm+a[1];
		let j:usize=b[0]*nm+b[1];
		if !self.stabm[i][j]{
			return false;
		}
		true
	}
	fn create_1st_layer(&self,init:&Vec<[usize;N]>)->HashMap<[usize;N],Vec<[usize;N]>>{
	//fn create_1st_layer(&self,init:&Vec<[usize;N]>)->HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>{
		let nm:usize=self.m.len();
		let nw:usize=self.w.len();
		let mut hash:HashMap<[usize;N],Vec<[usize;N]>>=HashMap::new();
		//let mut hash:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=HashMap::new();
		for i in 0..nm-1{
			for j in (i)*nm..(i+1)*nm{
				let mut vec:Vec<[usize;N]>=vec![];
				let mut is_stable:bool=false;
				for k in (i+1)*nm..(i+2)*nm{
					if Self::check_compatibility(&init[j],&init[k]){
						if self.check_stability(&init[j],&init[k]){
							vec.push(init[k]);
							is_stable=true;
						}
					}
				}
				if is_stable{
					hash.insert(init[j],vec);
				}
			}
		}
		hash
	}
	fn create_1st_layer2(&self,init:&Vec<[usize;N]>)->HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>{
		let nm:usize=self.m.len();
		let nw:usize=self.w.len();
		let mut hash:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=HashMap::new();
		for i in 0..nm-1{
			for j in (i)*nm..(i+1)*nm{
				let mut vec:Vec<Vec<[usize;N]>>=vec![];
				let mut is_stable:bool=false;
				for k in (i+1)*nm..(i+2)*nm{
					println!("i:{}, j:{}, k:{}, nm:{}, INIT: {:?}",i,j,k,nm,init);
					if Self::check_compatibility(&init[j],&init[k]){
						if self.check_stability(&init[j],&init[k]){
							vec.push(vec![init[j],init[k]]);
							is_stable=true;
						}
					}
				}
				if is_stable{
					hash.insert(vec![init[j]],vec);
				}
			}
		}
		hash
	}	
	
	fn print_hash(hash:&HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>){
		for el in hash.keys(){
			println!("{:?} --> {:?}",el,hash[el]);
		}
		
	}
	fn run(&self,hash:&HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>)->Vec<Vec<[usize;N]>>{
		// later maxn like in cap_matrix for N-sided matching
		let n:usize=self.m.len();
		let mut matches:Vec<Vec<[usize;N]>>=vec![];
		let mut new_hash:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=HashMap::new();
		//println!("hash.keys():{:?}",hash.keys());
		if hash.keys().len()>0{
			for el in hash.keys(){
				//println!("el:{:?}",el);
				if el.len()==n-1{
					for i in 0..hash[el].len(){
						matches.push(hash[el][i].clone());	
					}
				}
				else{
					let first:[usize;N]=el[0];
					println!("first:{:?}",first);
					for i in 0..hash[el].len(){
					//for i in 0..el_len{
						let el_len:usize=hash[el][i].len();
						//let mut entry:Vec<[usize;N]>=hash[el][i].clone();
						//println!
						let last:[usize;N]=hash[el][i][el_len-1];
						if Self::check_compatibility(&first,&last){
							if self.check_stability(&first,&last){
								let mut new_vec:Vec<[usize;N]>=vec![first];
								//println!("i:{}, hash[{:?}]:{:?}",i,el,hash[el]);
								let mut hash_el_i:Vec<[usize;N]>=hash[el][i].clone();
								println!("hash_el_i:{:?}, hash[{:?}][{}]:{:?}",hash_el_i,el,i,hash[el][i]);
								//new_vec.append(&mut hash[el][i].clone());
								new_vec.append(&mut hash_el_i.clone());
								// be careful!
								//if hash[el][i] in new_hash.keys().iter(){
								//if new_hash.keys().iter().collect().contains(&hash[el][i]){
								if !new_hash.contains_key(&hash[el][i]){	
									//new_hash.insert(hash[el][i],vec![new_vec]);									
									//println!("hash_el_i:{:?}\next_vec:{:?}",hash_el_i,new_vec);
									new_hash.insert(hash_el_i,vec![new_vec]);									
								}
								else{
									//new_hash.insert(hash[el][i],new_vec);
									let mut ext_vec:Vec<Vec<[usize;N]>>=new_hash.get(&hash[el][i]).unwrap().to_vec();
									ext_vec.push(new_vec);
									//println!("hash_el_i:{:?}\next_vec:{:?}",hash_el_i,ext_vec);
									new_hash.insert(hash_el_i,ext_vec);
									
								}
							}
						}
					}
				}
			}
			println!("new_hash:\n{:?}",new_hash);
			matches=self.run(&new_hash);
		}
		matches
	}
	// WORKS BUT SLOW!!!!!!!
	fn run2(&self,hash:&HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>)->Vec<Vec<[usize;N]>>{
		//let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		// later maxn like in cap_matrix for N-sided matching
		let n:usize=self.m.len();
		let mut matches:Vec<Vec<[usize;N]>>=vec![];
		let mut new_hash:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=HashMap::new();
		//println!("hash.keys():{:?}",hash.keys());
		if hash.keys().len()>0{
			for el in hash.keys(){
				//println!("el:{:?}",el);
				if el.len()==n-1{
					println!("MATCH: {:?}",hash[el]);
					for i in 0..hash[el].len(){
						matches.push(hash[el][i].clone());	
					}
				}
				else{
					let first:[usize;N]=el[0];
					//println!("first:{:?}",first);
					for i in 0..hash[el].len(){
					//for i in 0..el_len{
						let mut tail:Vec<[usize;N]>=hash[el][i].clone();
						tail.remove(0);
						//println!("tail:{:?}",tail);
						if hash.contains_key(&tail){
							for j in 0..hash[&tail].len(){
								let el_len:usize=hash[&tail][j].len();
								//let mut entry:Vec<[usize;N]>=hash[el][i].clone();
								//println!
								//let last:[usize;N]=hash[el][j][el_len-1];
								let last:[usize;N]=hash[&tail][j][el_len-1];
								if Self::check_compatibility(&first,&last){
									if self.check_stability(&first,&last){
										//let mut new_vec:Vec<[usize;N]>=vec![first];
										let mut new_vec:Vec<[usize;N]>=hash[el][i].clone();;
										//println!("i:{}, hash[{:?}]:{:?}",i,el,hash[el]);
										let mut hash_el_i:Vec<[usize;N]>=hash[el][i].clone();
										//println!("hash_el_i:{:?}, hash[{:?}][{}]:{:?}",hash_el_i,el,i,hash[el][i]);
										//new_vec.append(&mut hash[el][i].clone());
										
										// not append but push(last)!
										//new_vec.append(&mut hash_el_i.clone());
										new_vec.push(last);
										//println!("new_vec:{:?}",new_vec);
										// be careful!
										//if hash[el][i] in new_hash.keys().iter(){
										//if new_hash.keys().iter().collect().contains(&hash[el][i]){
										if !new_hash.contains_key(&hash[el][i]){
											//println!("not contains!");
											//new_hash.insert(hash[el][i],vec![new_vec]);									
											//println!("hash_el_i:{:?}\next_vec:{:?}",hash_el_i,new_vec);
											new_hash.insert(hash_el_i,vec![new_vec]);									
										}
										else{
											//println!("already contains!");
											//new_hash.insert(hash[el][i],new_vec);
											let mut ext_vec:Vec<Vec<[usize;N]>>=new_hash.get(&hash[el][i]).unwrap().to_vec();
											//let mut ext_vec:Vec<Vec<[usize;N]>>=new_hash.get(&hash[&tail][i]).unwrap().to_vec();
											//println!("ext_vec BEFORE:{:?}",ext_vec);
											ext_vec.push(new_vec);
											//println!("ext_vec AFTER:{:?}",ext_vec);
											//println!("hash_el_i:{:?}\next_vec:{:?}",hash_el_i,ext_vec);
											new_hash.insert(hash_el_i,ext_vec);
											
										}
									}
								}
							}
						}
					}
				}
			}
			//println!("hello?");
			//println!("new_hash:\n{:?}",new_hash);
			//Self::print_hash(&new_hash);
			println!("new_hash.keys().len(): {}",new_hash.keys().len());
			//println!("matches:{:?}",matches);
			matches=self.run2(&new_hash);
		}
		matches
	}	
	fn run3(&self,hash:&HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>,depth:usize)->Vec<Vec<[usize;N]>>{
		//let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		// later maxn like in cap_matrix for N-sided matching
		let n:usize=self.m.len();
		let mut matches:Vec<Vec<[usize;N]>>=vec![];
		let mut new_hash:HashMap<Vec<[usize;N]>,Vec<Vec<[usize;N]>>>=HashMap::new();
		//println!("hash.keys():{:?}",hash.keys());
		println!("DEPTH: {}",depth);
		if hash.keys().len()>0{
			for el in hash.keys(){
				//println!("el:{:?}",el);
				if el.len()==n-1{
					println!("MATCH: {:?}",hash[el]);
					for i in 0..hash[el].len(){
						matches.push(hash[el][i].clone());	
					}
				}
				else{
					let first:[usize;N]=el[0];
					//println!("first:{:?}",first);
					for i in 0..hash[el].len(){
					//for i in 0..el_len{
						let mut tail:Vec<[usize;N]>=hash[el][i].clone();
						tail.remove(0);
						if hash.contains_key(&tail){
							for j in 0..hash[&tail].len(){
								let el_len:usize=hash[&tail][j].len();
								let last:[usize;N]=hash[&tail][j][el_len-1];
								if Self::check_compatibility(&first,&last){
									if self.check_stability(&first,&last){
										let mut new_vec:Vec<[usize;N]>=hash[el][i].clone();;
										//let mut hash_el_i:Vec<[usize;N]>=hash[el][i].clone();
										let mut hash_el_i:&Vec<[usize;N]>=&hash[el][i];
										new_vec.push(last);
										//println!("new_vec:{:?}",new_vec);
										// be careful!
										//if hash[el][i] in new_hash.keys().iter(){
										//if new_hash.keys().iter().collect().contains(&hash[el][i]){
										if !new_hash.contains_key(&hash[el][i]){
											//new_hash.insert(hash_el_i,vec![new_vec]);									
											new_hash.insert(hash_el_i.to_vec(),vec![new_vec]);									
										}
										else{
											let mut ext_vec:Vec<Vec<[usize;N]>>=new_hash.get(&hash[el][i]).unwrap().to_vec();
											ext_vec.push(new_vec);
											//new_hash.insert(hash_el_i,ext_vec);
											new_hash.insert(hash_el_i.to_vec(),ext_vec);
											
										}
									}
								}
							}
						}
					}
				}
			}
			Self::print_hash(&new_hash);
			println!("new_hash.keys().len(): {}",new_hash.keys().len());
			matches=self.run3(&new_hash,depth+1);
		}
		matches
	}	
}

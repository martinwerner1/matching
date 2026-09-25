#![allow(warnings)]
use std::path::Path;
use rand::Rng;
use std::{
	fs::{self, File},
	io::{self, BufRead},
	//path::Path,
};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::{OpenOptions};
use std::io::{Write};
use super::image;
pub const N:usize=2;
#[derive(Debug)]
pub struct rank_tree{
	//nodes:Vec<usize>, // JUST FOR TESTS
	//nodes:Vec<rank_tree>,
	depth: usize,
	endvalue:Option<[usize;N]>,
	nodes:Vec<Rc<RefCell<rank_tree>>>,
}
impl rank_tree{
	pub fn new()->Self{
		Self{
			nodes:vec![],
			depth:0,
			endvalue:None
		}
	}
	// reference values?
	pub fn add_tree(&mut self,pos:usize,to_add:Rc<RefCell<rank_tree>>){
		if pos>self.nodes.len()-1{
			println!("Parameter 'pos' is too high! Please choose a smaller one!");
		}
		else{
			self.nodes[pos]=to_add;
			self.nodes[pos].borrow_mut().depth=self.depth+1;
		}
	}
	pub fn push_tree(&mut self,to_add:Rc<RefCell<rank_tree>>){
		let len:usize=self.nodes.len();
		self.nodes.push(to_add);
		self.nodes[len].borrow_mut().depth=self.depth+1;
	}
	pub fn add_endvalue(&mut self,val:[usize;N]){
		self.endvalue=Some(val);
	}
	pub fn rank_initialize(&mut self,depth_rem:usize,n:usize){
		if depth_rem>0{
			for i in 0..n{
				let mut new_tree:rank_tree=rank_tree::new();
				let ref_tree:Rc<RefCell<rank_tree>>=Rc::new(RefCell::new(rank_tree::new()));
				self.push_tree(ref_tree);
				self.nodes[self.nodes.len()-1].borrow_mut().rank_initialize(depth_rem-1,n);
			}
		}
		else{
			let init_arr:[usize;N]=[0;N];
			self.add_endvalue(init_arr);
		}
	}
	pub fn rank_initialize_nvec(&mut self,depth_rem:usize,nvec:&Vec<usize>,cursor:usize){
		if depth_rem>0{
		//if cursor<N{
			//for i in 0..n{
			for i in 0..nvec[cursor]{
				let mut new_tree:rank_tree=rank_tree::new();
				let ref_tree:Rc<RefCell<rank_tree>>=Rc::new(RefCell::new(rank_tree::new()));
				self.push_tree(ref_tree);
				//self.nodes[self.nodes.len()-1].borrow_mut().rank_initialize(depth_rem-1,n);
				self.nodes[self.nodes.len()-1].borrow_mut().rank_initialize_nvec(depth_rem-1,&nvec,cursor+1);
			}
		}
		else{
			let init_arr:[usize;N]=[0;N];
			self.add_endvalue(init_arr);
		}
	}	
	pub fn rank_update(&mut self,set:[usize;2],val:usize,cursor:usize,arr:[usize;N-1]){
		if cursor<N{
			let mut nextpos:usize=0; //exception!
			if cursor<set[0]{
				nextpos=arr[cursor];
			}
			if cursor==set[0]{
				nextpos=set[1];
			}
			if cursor>set[0]{
				nextpos=arr[cursor-1];
			}
			self.nodes[nextpos].borrow_mut().rank_update(set,val,cursor+1,arr);
		}
		else{
			if cursor==N{
				//println!("cursor==N_SIDED!!!");
				let mut tmp_val:[usize;N]=self.endvalue.clone().expect("");
				tmp_val[set[0]]=val;
				self.endvalue=Some(tmp_val);
			}
			else{
				println!("ERROR IN rank_update!!!");
			}
		}
	}
	pub fn rank_update2(&mut self,set:[usize;2],val:usize,cursor:usize,arr:[usize;N]){
		if cursor<N{
			let mut nextpos:usize=0; //exception!
			/*
			if cursor<set[0]{
				nextpos=arr[cursor];
			}
			if cursor==set[0]{
				nextpos=set[1];
			}
			if cursor>set[0]{
				nextpos=arr[cursor-1];
			}
			*/
			nextpos=arr[cursor];
			self.nodes[nextpos].borrow_mut().rank_update2(set,val,cursor+1,arr);
		}
		else{
			if cursor==N{
				//println!("cursor==N_SIDED!!!");
				let mut tmp_val:[usize;N]=self.endvalue.clone().expect("");
				tmp_val[set[0]]=val;
				self.endvalue=Some(tmp_val);
			}
			else{
				println!("ERROR IN rank_update!!!");
			}
		}
	}
	pub fn retrieve_endvalue(&self,path:&Vec<usize>,cursor:usize)->[usize;N]{
		let endvalue:[usize;N]=[0;N];
		if cursor>path.len()-1{
			return self.endvalue.expect("");
		}
		else{
			return self.nodes[path[cursor]].borrow_mut().retrieve_endvalue(&path,cursor+1);
		}
		//endvalue
	}
	pub fn retrieve_endvalue_arr(&self,path:&[usize;N],cursor:usize)->[usize;N]{
		let endvalue:[usize;N]=[0;N];
		if cursor>path.len()-1{
			return self.endvalue.expect("");
		}
		else{
			return self.nodes[path[cursor]].borrow_mut().retrieve_endvalue_arr(&path,cursor+1);
		}
		//endvalue
	}	
}


pub struct IOClass{

}
impl IOClass{
	// read preferences stored in folder 'pref'
	pub fn read_txt(to_pref: String) -> Vec<Vec<usize>> {
		let path = Path::new(&to_pref);
		println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		let mut idx:usize = 0;
		let mut matrix: Vec<Vec<usize>> = vec![];
		let base10: usize = 10;
		let pattern1: [char;9] = ['1','2','3','4','5','6','7','8','9'];
		let pattern2: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];
		let non_num: [char;5] = ['[',',',' ',']','.'];
		for line in reader.lines() {
			let line = line.expect("Could not read file");
			let mut arr: Vec<usize> = vec![];
			let mut ychars: Vec<char> = vec![];
			for y in line.chars() {
				ychars.push(y);
			}
			let mut i: usize=0;
			while i < ychars.len() -1{
				if pattern2.contains(&ychars[i]) {
					let mut j_start: usize = i+1;
					for j in i+1..ychars.len() {
						if ['[',',',' ',']','.'].contains(&ychars[j]) {
							j_start = j;
							let mut num: usize = 0;
							for k in i..j {
								let digit = (&ychars[k].to_string()).parse::<usize>().unwrap();
								let j_conv = j as u32;
								let k_conv = k as u32;
								num += digit * base10.pow(j_conv - k_conv-1);
							}
							arr.push(num);
							i=j_start+1;
							break;
						}
					}
				}
				i+=1;
			}
			matrix.push( arr );
			idx += 1;
		}
		return matrix;
	}
	
	
	pub fn get_bp_automatically()->Vec<Vec<bool>>{
		let m:Vec<Vec<usize>>=Self::read_txt("pref/m.txt".to_string());
		let w:Vec<Vec<usize>>=Self::read_txt("pref/w.txt".to_string());
		let adj:Vec<Vec<[usize;2]>>=Self::create_adj(&m,&w);
		let bp:Vec<Vec<bool>>=Self::bp_matrix(&adj);
		bp
	}
	pub fn update_bp_manymany(bp_:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
		let mut bp:Vec<Vec<bool>>=bp_.clone();
		let n:usize=bp.len().isqrt();
		// agents of outer grid have quotas
		for i in 0..n{
			for j in 0..n{
				for k in 0..n{
					bp[i*n+j][i*n+k]=true;
				}
			}
		}
		for i in 0..n{
			for j in 0..n{
				for k in 0..n{
					bp[i*n+j][k*n+j]=true;
				}
			}
		}
		println!("bp manymany:{:?}",bp);
		bp
	}
	pub fn create_zero_bp(n:usize)->Vec<Vec<bool>>{
		let mut bp:Vec<Vec<bool>>=vec![vec![false;n*n];n*n];
		bp
	}
	pub fn create_adj( m_: &Vec<Vec<usize>>, w_: &Vec<Vec<usize>> ) -> Vec<Vec<[usize; 2]>> {
		let mut adj = vec![];
		let l = m_.len();
		for i in 0..m_.len() {
			adj.push( vec![] );
			for j in 0..m_.len() {
				adj[i].push([l,l]);
			}
		}
		for i in 0..l {
			for j in 0..m_.len() {
				let mij = m_[i][j];
				let wij = w_[i][j];
				let posmw = m_[i].iter().position(|&x| x == mij).unwrap();
				let poswm = w_[i].iter().position(|&x| x == wij).unwrap();
				adj[i][mij][0] = posmw;
				adj[wij][i][1] = poswm;
			}
		}
		return adj;
	}

	pub fn bp_matrix(adj:&Vec<Vec<[usize;2]>>)->Vec<Vec<bool>>{
		let mut mat:Vec<Vec<bool>>=vec![];
		let n:usize=adj.len();
		for i in 0..n{		
			for j in 0..n{
				let mut row:Vec<bool>=vec![];
				for k in 0..n{
					for m in 0..n{
						if !(i==k || j==m){
							if !Self::bp_efficient(&adj,&[i,j],&[k,m],0){
								row.push(true);
							}
							else{
								row.push(false);
							}
						}
						else{
							row.push(false);
						}					
					}
				}
				mat.push(row);			
			}
		}
		mat
	}
	pub fn bp_matrix_lower_half(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
		let mut new:Vec<Vec<bool>>=bp.clone();
		for i in 0..new.len(){
			for j in i..new[i].len(){
				new[i][j]=false;
			}
		}
		new
	}
	pub fn bp_matrix_half(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
		let mut new:Vec<Vec<bool>>=bp.clone();
		for i in 0..new.len(){
			for j in 0..i{
				new[i][j]=false;
			}
		}
		new
	}	
	pub fn bp_mat_count(bp:&Vec<Vec<bool>>)->Vec<usize>{
		let mut new:Vec<Vec<bool>>=vec![];
		let mut order:Vec<usize>=vec![];
		let mut st_count:Vec<usize>=vec![];
		for i in 0..bp.len(){
			let mut count:usize=0;
			order.push(i);
			for j in 0..bp[i].len(){
				if bp[i][j]{
					count+=1;
				}
			}
			st_count.push(count);
		}
		//new
		st_count
	}
	pub fn find_max_pos_line(vec_a:&Vec<usize>,b:usize)->usize{
		let mut pos:usize=0;
		// this holds for ordered elems (desc)
		//println!("start find_pos2: b: {:?}, vec_a: {:?}",b,vec_a);
		if vec_a.len()>0{
			for i in 0..vec_a.len(){
				/*
				if b<vec_a[i]{
					pos=i;
					break;
				}
				if i==vec_a.len()-1{
					pos=i+1;
				}
				*/
				//println!("i: {}, b: {:?}, vec_a: {:?}",i,b,vec_a);
				if b>vec_a[i]{
					pos=i;
					//println!("pos=i {}, break;",pos);
					break;				
				}
				if i==vec_a.len()-1{
					pos=i+1;
				}
			}
		}
		return pos;
	}
	pub fn bp_reshuffle(bp:&Vec<Vec<bool>>)->(Vec<Vec<bool>>,Vec<usize>){
		//let mut new:Vec<Vec<bool>>=vec![];
		let mut new:Vec<Vec<bool>>=bp.clone();
		let cn:Vec<usize>=Self::bp_mat_count(&bp);
		let mut order:Vec<usize>=vec![];
		let mut val:Vec<usize>=vec![];
		for i in 0..new.len(){
			println!("bp_reshuffle i:{}",i);
			let max_pos:usize=Self::find_max_pos_line(&val,cn[i]);
			val.insert(max_pos,cn[i]);
			order.insert(max_pos,i);
			let mut del_pos:usize=i;
			if max_pos<i{
				del_pos=i+1;
			}
			new.insert(max_pos,new[i].clone());
			new.remove(del_pos);
			for j in 0..new.len(){
				let val_ij=new[j][i];
				//new[j].insert(max_pos,new[j][i].clone());
				new[j].insert(max_pos,val_ij);
				new[j].remove(del_pos);
			}
		}
		println!("NEW ORDER IN BP RESHUFFLE:\n{:?}",order);
		(new,order)
	}
	pub fn bp_matrix_2usize(bp:&Vec<Vec<bool>>)->Vec<usize>{
		let mut bits:Vec<usize>=vec![];
		let n:usize=bp.len();
		for i in 0..bp.len(){
			let mut val:usize=0;
			for j in 0..bp[i].len(){
				if bp[i][j]{
					// BEFORE: I32, NOW: U128 !!!!!!!
					val+=(2_u128.pow((n-j-1) as u32)) as usize;
				}
			}
			bits.push(val);
		}
		bits
	}






	pub fn bp_matrix_2usize_block(bp:&Vec<Vec<bool>>,bs_:usize)->Vec<Vec<usize>>{
		let mut bits:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len();
		let sq_n:usize=n.isqrt();
		let mut bs:usize=bs_;
		
		// WHY SQUAREROOT?
		//if bs>sq_n{
		if bs>n{
			//bs=sq_n;
			bs=n;
		}	
		let mut block:usize=n/bs;
		if block*bs<n{
			block+=1;
		}
		for i in 0..bp.len(){
			let mut val:usize=0;
			let mut bits_i:Vec<usize>=vec![];
			let mut i_b:usize=0;
			for j in 0..bp[i].len(){
				/*			
				if j==i_b*bs && j!=0{
					i_b+=1;
				}
				*/
				if bp[i][j]{
					//val+=(2_i32.pow((n-j-1) as u32)) as usize;
					//println!("bs: {}, j: {}, i_b: {}, val: {}, term:",bs,j,i_b,val);
					//println!("{}",bs+i_b*bs-1-j);
					//val+=(2_i32.pow((bs-j+i_b*bs-1) as u32)) as usize;
					
					// CHANGED i32 TO u32
					//val+=(2_i32.pow((bs+i_b*bs-1-j) as u32)) as usize;
					val+=(2_u32.pow((bs+i_b*bs-1-j) as u32)) as usize;
				}
				// RUNTIME ERROR?
				/*
				if j==i_b*bs-1|| j==n-1{
					i_b+=1;
					bits_i.push(val as usize);
				}
				*/
				
				if j==bs-1 && i_b==0{
					i_b+=1;
					bits_i.push(val as usize);
					val=0;
				}
				else{
					//println!("test point bp_matrix_2usize_block");
					
					if j<bs{
						continue;
					}
					/*
					if j>bs{
						println!("j>bs");
					}
					*/
					
					// BE VERY VERY VERY VEY CAREFUL LATER !!!!!!!!!!!!
					if j==(i_b+1)*bs-1 || j==n-1{
						//println!("increase");
						i_b+=1;
						bits_i.push(val as usize);
						val=0;
					}
				}


			}
			//bits.push(val);
			bits.push(bits_i);
		}
		bits
	}



	pub fn bp_matrix_2usize_block2(bp:&Vec<Vec<bool>>,bs_:usize)->Vec<Vec<usize>>{
		let mut bits:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len();
		let sq_n:usize=n.isqrt();
		let mut bs:usize=bs_;
		
		// WHY SQUAREROOT?
		//if bs>sq_n{
		if bs>n{
			//bs=sq_n;
			bs=n;
		}	
		let mut block:usize=n/bs;
		if block*bs<n{
			block+=1;
		}
		for i in 0..bp.len(){
			let mut val:usize=0;
			let mut bits_i:Vec<usize>=vec![];
			let mut i_b:usize=0;
			for j in 0..bp[i].len(){
				/*			
				if j==i_b*bs && j!=0{
					i_b+=1;
				}
				*/
				if bp[i][j]{
					//val+=(2_i32.pow((n-j-1) as u32)) as usize;
					//println!("bs: {}, j: {}, i_b: {}, val: {}, term:",bs,j,i_b,val);
					//println!("{}",bs+i_b*bs-1-j);
					//val+=(2_i32.pow((bs-j+i_b*bs-1) as u32)) as usize;
					
					// CHANGED i32 TO u32
					//val+=(2_i32.pow((bs+i_b*bs-1-j) as u32)) as usize;
					
					//println!("test point");
					
					// CHANGED FROM I32 TO U32 TO USIZE TO U128 !!!!!!! WORKS !!!!!!!
					
					//val+=(2_u32.pow((bs+i_b*bs-1-j) as u32)) as usize;
					//val+=(2_usize.pow((bs+i_b*bs-1-j) as u32)) as usize;
					val+=(2_u128.pow((bs+i_b*bs-1-j) as u32)) as usize;
				}
				// RUNTIME ERROR?
				/*
				if j==i_b*bs-1|| j==n-1{
					i_b+=1;
					bits_i.push(val as usize);
				}
				*/
				
				if j==bs-1 && i_b==0{
					i_b+=1;
					bits_i.push(val as usize);
					val=0;
				}
				else{
					//println!("test point bp_matrix_2usize_block");
					//println!("j:{},i{},i_b:{},bs:{},n:{}, (i_b+1)*bs-1:{}",j,i,i_b,bs,n,(i_b+1)*bs-1);
					//println!("j:{},i{},i_b:{},bs:{},n:{}, (i_b+1)*bs-1:{}",j,i,i_b,bs,n,(i_b+1)*bs-1);
					
					if j<bs{
						continue;
					}
					
					/*
					if j>=bs{
						println!("j>bs");
					}
					*/
					
					//println!("j:{},i{},i_b:{},bs:{},n:{}, (i_b+1)*bs-1:{}",j,i,i_b,bs,n,(i_b+1)*bs-1);
					
					// BE VERY VERY VERY VEY CAREFUL LATER !!!!!!!!!!!!
					if (j==(i_b+1)*bs-1) || (j==n-1){
						//println!("increase");
						i_b+=1;
						bits_i.push(val as usize);
						val=0;
					}
				}


			}
			//bits.push(val);
			bits.push(bits_i);
		}
		bits
	}
	pub fn bp_efficient(adj:&Vec<Vec<[usize;2]>>,a:&[usize;2],b:&[usize;2],diff:usize)->bool{
		if adj[a[0]][b[1]][0]+diff<adj[a[0]][a[1]][0]{
			if adj[a[0]][b[1]][1]+diff<adj[b[0]][b[1]][1]{
				return true;
			}
		}
		if adj[b[0]][a[1]][0]+diff<adj[b[0]][b[1]][0]{
			if adj[b[0]][a[1]][1]+diff<adj[a[0]][a[1]][1]{
				return true;
			}
		}	
		return false;
	}	
	pub fn decompose_matches(matches:&Vec<Vec<usize>>,n:usize)->Vec<Vec<[usize;2]>>{
		let mut decomposed_matches:Vec<Vec<[usize;2]>>=vec![];
		
		for i in 0..matches.len(){
			let mut match_i:Vec<[usize;2]>=vec![];
			for j in 0..matches[i].len(){
				let (m,w)=Self::decompose_ij(matches[i][j],n);
				match_i.push([m,w]);
			}
			decomposed_matches.push(match_i);
		}
		decomposed_matches
	}
	pub fn decompose_ij(len:usize,n:usize)->(usize,usize){
		let mut i:usize=len/n;
		let mut j:usize=len-i*n;
		println!("len: {}, n: {}, i: {}, j: {}",len,n,i,j);
		(i,j)
	}
	pub fn print_matches_decomposed(matches:&Vec<Vec<[usize;2]>>){
		println!("ALL STABLE MATCHES:");
		for i in 0..matches.len(){
			for j in 0..matches[i].len(){
				let pair=Self::transform_pair_2string(matches[i][j]);
				
				print!("{{{},{}}}",pair.0,pair.1);
				if j<matches.len()-1{
					//print!(",");
				}
				else{
					print!("");
				}
			}
			println!();
		}
	}
	pub fn transform_pair_2string(pair:[usize;2])->(String,String){
		let mut m:String="m".to_string();
		m+=&(pair[0]+1).to_string();
		let mut w:String="w".to_string();
		w+=&(pair[1]+1).to_string();
		(m,w)
	}

}
pub struct Generate{
	
}
impl Generate{
	pub fn generate_hashvalue(len:usize)->String{
		let mut hash:&str="";
		for i in 0..len{
			//let mut randval=rand::rng().gen_range(1..10000);
			let mut randval:u32=rand::random_range(1..4000000000);
			println!("{:?}",randval);
		}
		hash.to_string()
	}
}

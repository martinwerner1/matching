#![allow(warnings)]
use image::*;
use imageproc::*;
use rand::seq::SliceRandom;
use std::path::Path;

use super::helper::{IOClass};
use std::{
	fs::{self, File},
	io::{self, BufRead},
	//path::Path,
};
//use rustplex::prelude::{Model,SolverError,Maximize};
use rustplex::prelude::{Model,LinearExpr,Maximize,VariableKey,SolverError};
use image::{RgbImage, Rgb, RgbaImage, Rgba};
use imageproc::drawing::{draw_text_mut,text_size};
use image::DynamicImage::ImageRgb8;
use image::io::Reader;
//use highs::*;
//use simplex::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::OpenOptions;
use highs::*;
use std::io::Write;
use bitvec::prelude::BitVec;
use bitvec::prelude::*;




const N:usize=4;
fn main(){
	test_deepsearch_n_sided();
}

pub fn test_deepsearch_n_sided(){
	let mut n_sided:n_sided_matching=n_sided_matching::new();
	n_sided.init(3);
	n_sided.show_pref();
	//let mut chain:Vec<[usize;N]>=vec![[0,1,2],[2,3,1]];
	//let b:[usize;N]=[1,2,3];
	//let stable:bool=n_sided.local_search_blocking_coalition(&chain,&b);
	//let stable:bool=n_sided.local_search_blocking_coalition_intersection_wrapper(&chain);
	//println!("STABLE? ### {} ####",stable);
	//println!("DECOMPOSE: {:?}",n_sided.decompose(45,&vec![4,4,4]));
	//println!("COMPOSE:{}",n_sided.compose(&[2,3,2]));
	// COMMENTED!
	// DUAL WORKS!
	
	//n_sided.lp_eq(&vec![2,3,4,2]); // COMMENTED!
	//n_sided.lp_run(); //HIGHS // COMMENTED!
	
	//n_sided.lp_simplex_run();
	
	// COMMENTED!
	// DUAL WORKS!
	//n_sided.lp_rustplex_run(); // COMMENTED!
	//n_sided.lp_dual_lp_highs_run(); // COMMENTED!
	
	
	println!("DEEP SEARCH N-SIDED!");
	let matches_deepsearch:Vec<Vec<[usize;N]>>=n_sided.deepsearch(&vec![],&[0;N],0,&n_sided.nvec);
	let matches_clean:Vec<Vec<[usize;N]>>=n_sided.remove_multiple_solutions(&matches_deepsearch);
	println!("MATCHES DEEP SEARCH:");
	//for i in 0..matches_deepsearch.len(){
	for i in 0..matches_clean.len(){
		//println!("MATCH {}: {:?}",i+1,matches_deepsearch[i]);
		println!("MATCH {}: {:?}",i+1,matches_clean[i]);
	}
	
}
/* This struct is for creating matching outcomes for many-to-one-to-many N-sided matchings.
 * We have n-vector for multiple n values for each set of agents S_i. We also have a cap vector for 
 * many-to-one matching with multiple constellations of agents of the same set!
*/
struct stablesync_n_sided{
	pref:Vec<Vec<Vec<[usize;N]>>>,
	capvec:Vec<Vec<Vec<usize>>>,
	cap_1dim:Vec<usize>,
	nvec:Vec<usize>,
	maxn:usize,
	dec_nvec:Vec<usize>,
	rank:rank_tree,
	stabm:Vec<Vec<bool>>,
	bitmatrix:Vec<Vec<usize>>,
	ordervec_sync:Vec<Vec<usize>>,
}
impl stablesync_n_sided{
	fn new()->Self{
		Self{
			pref:vec![],
			capvec:vec![],
			cap_1dim:vec![],
			nvec:vec![],
			maxn:0,
			dec_nvec:vec![],
			rank:rank_tree::new(),
			stabm:vec![],
			bitmatrix:vec![],
			ordervec_sync:vec![],
		}
	}
	fn init(){
		
	}
	fn get_nvec(pref:&Vec<Vec<Vec<[usize;N]>>>)->Vec<usize>{
		let mut nvec:Vec<usize>=vec![];
		for i in 0..pref.len(){
			nvec.push(pref[i].len());
		}
		nvec
	}
	fn get_max_n(nvec:&Vec<usize>)->usize{
		let mut maxn:usize=0;
		for i in 0..nvec.len(){
			if maxn<nvec[i]{
				maxn=nvec[i];
			}
		}
		maxn
	}
	fn get_product_of_nvec(nvec:&Vec<usize>)->usize{
		let mut prod:usize=0;
		if nvec.len()>0{
			prod=nvec[0];
			if nvec.len()>1{
				for i in 1..nvec.len(){
					prod*=nvec[i];
				}
			}
		}
		prod
	}
	fn get_all_groups_nvec(tmp:&[usize;N],nvec:&Vec<usize>,cursor:usize)->Vec<[usize;N]>{
		let mut groups:Vec<[usize;N]>=vec![];
		if cursor>N{
			return vec![*tmp];
		}
		else{
			for i in 0..nvec[cursor]{
				let mut new_tmp:[usize;N]=tmp.clone();
				new_tmp[cursor]=i;
				groups.append(&mut Self::get_all_groups_nvec(&new_tmp,&nvec,cursor+1));
			}
		}
		groups
	}
	fn get_ordervec(&self,boolvec:&Vec<Vec<bool>>)->Vec<Vec<usize>>{
		let mut ordervec:Vec<Vec<usize>>=vec![];
		for i in 0..boolvec.len(){
			let mut ordervec_i:Vec<usize>=vec![];
			for j in 0..boolvec[i].len(){
				if boolvec[i][j]{
					ordervec_i.push(j);
				}
			}
			ordervec.push(ordervec_i);
		}
		ordervec
	}	
	fn create_random_pref_stack(nvec:&Vec<usize>)->Vec<Vec<Vec<[usize;N]>>>{
		let mut prefs:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for i in 0..N{
			let mut pref:Vec<Vec<[usize;N]>>=vec![];
			for j in 0..nvec[i]{
				let mut pref_row:Vec<[usize;N]>=Self::get_all_groups_nvec(&[0;N],&nvec,0);
				let mut rng=rand::rng();
				pref_row.shuffle(&mut rng);
				pref.push(pref_row);
			}			
			prefs.push(pref);
		}
		prefs
	}
	fn read_pref(txtfile:String)->Vec<Vec<[usize;N]>>{
		let mut pref:Vec<Vec<[usize;N]>>=vec![];		
		let path = Path::new(&txtfile);
		//println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			let mut pref_i:Vec<[usize;N]>=vec![];
			let mut vecchar:Vec<char>=line.expect("").chars().collect();
			vecchar.remove(0);
			vecchar.remove(vecchar.len()-1);
			//println!("last:{:?}",vecchar);
			let tmp_string:String=vecchar.clone().into_iter().collect();
			for arr in tmp_string.split(']'){
				//println!("arr:{:?}",arr);
				let mut word:String=arr.to_string();
				let mut word_char:Vec<char>=word.chars().collect();
				//if arr.contains(&","){
				//if word.chars().collect().get(0)==','{
				if arr.len()>0{
					if word_char[0]==','{
						//word.remove(0);
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					if word_char[0]=='['{
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					// for 4-sided matching we have [0,0,0]. for 5-sided matching we have [0,0,0,0].
					let mut el:[usize;N]=[0;N];
					let mut idx:usize=0;
					for part in word.split(','){
						let mut word_part:String=part.to_string();
						if part.contains(&" "){
							word_part=(&word_part[1..word_part.len()]).to_string();
						}
						let digit = word_part.parse::<usize>().unwrap();
						el[idx]=digit;
						idx+=1;
						let part_chars:Vec<char>=word_part.chars().collect();
						//println!("part:{:?}",part_chars);
					}
					pref_i.push(el);
					//println!("word:{:?}\nword_char:{:?}",word,word_char);
				}
			}
			pref.push(pref_i);			
		}
		println!("PREF:\n{:?}",pref);
		pref
	}
	fn read_all_pref(folder:String)->Vec<Vec<Vec<[usize;N]>>>{
		let mut prefs:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for i in 0..N{
			let mut filepath:String="nsided/pref_".to_string();
			filepath+=&i.to_string();
			filepath+=".txt";
			let pref:Vec<Vec<[usize;N]>>=Self::read_pref(filepath);
			prefs.push(pref);
		}
		prefs
	}
	fn write_pref(&self,folder:String){
		for i in 0..self.pref.len(){
			let mut txt:String="".to_string();
			for j in 0..self.pref[i].len(){
				txt+=&format!("{:?}",self.pref[i][j]).to_string();
				txt+="\n";
			}
			let mut filepath:String=folder.clone();
			let filepath_chars:Vec<char>=filepath.chars().collect();
			if filepath_chars[filepath.len()-1]=='/'{
				filepath+="pref_";
			}
			else{
				filepath+="/pref_";
			}
			filepath+=&i.to_string();
			let mut fs=File::create(filepath);
			fs.expect("File cannot be created").write_all(txt.as_bytes());
		}
	}
	
	fn create_rank_tree2()->rank_tree{
		let mut rank:rank_tree=rank_tree::new();
		rank
	}
	fn create_rank_tree(&mut self){
		// be careful!
		let n:usize=self.pref[0].len();
		//let mut adj:Vec
		let nvec:Vec<usize>=self.nvec.clone();
		let mut rank:rank_tree=rank_tree::new();
		rank.rank_initialize_nvec(N,&nvec,0);
		//adj.rank_update(1,2,0,[1,0]);
		//adj.rank_update([1,0],2,0,[1,0]);
		for i in 0..self.pref.len(){
			for j in 0..self.pref[i].len(){
				for k in 0..self.pref[i][j].len(){
					let val:usize=self.pref[i][j].iter().position(|&x| x==self.pref[i][j][k]).expect("");
					rank.rank_update2([i,j],val,0,self.pref[i][j][k]);
				}
			}
		}
		//dbg!(&rank);
		self.rank=rank;
	}
	// EXAMPLE!
	// maybe we need it later!?
	// 4 x 7 x 2 = 56 (n1=4, n2=7, n3=2)
	// prod=56 (total product of all n over all set of S_1,..,S_N)!
	// dec_nvec[0]=14, prod=14
	// dec_nvec[1]=2,  prod=2
	// dec_nvec[2]=1,  prod=1
	fn create_decompose_nvec(&self,nvec:&Vec<usize>)->Vec<usize>{
		let mut dec_nvec:Vec<usize>=vec![];
		let mut prod:usize=Self::get_product_of_nvec(&nvec);
		for i in 0..nvec.len(){
			prod/=nvec[i];
			dec_nvec.push(prod);
		}
		dec_nvec
	}
	// is completely wrong! decompose_nvec has to be created and to be inserted! please think about it! 
	// take it from the object variable dec_nvec!
	
	// do we really need it? we can store the whole decompose-vector into the object such that we 
	// can retrieve the group by i_val->group([usize;N])!
	fn decompose(num:usize,nvec:&Vec<usize>)->[usize;N]{
		// not nvec should be taken but self.dec_nvec! it's created by create_decompose_nvec!
		let mut pattern:[usize;N]=[0;N];
		let mut rest:usize=num;
		for i in 0..N{
			let i_val:usize=rest/nvec[i];
			pattern[i]=i_val;
			rest=rest-i_val*nvec[i];
		}
		pattern
	}
	fn create_stability_matrix(&mut self){
		let total_n:usize=Self::get_product_of_nvec(&self.nvec);
		for i in 0..total_n{
			let group_ref:[usize;N]=Self::decompose(i,&self.nvec);
			let mut stabm_i:Vec<bool>=vec![];
			for j in 0..total_n{
				let group:[usize;N]=Self::decompose(j,&self.nvec);
				if self.is_stable3(&group_ref,&group){
					stabm_i.push(true);
				}
				else{
					stabm_i.push(false);					
				}
			}
		}
	}
	fn get_all_binvec(&mut self,tmp:[usize;N],cursor:usize)->Vec<[usize;N]>{
		let mut res:Vec<[usize;N]>=vec![];
		//if tmp.len()==N_SIDED{
		if cursor==N{
			return vec![tmp.clone()];
		}
		else{
			let mut tmp0:[usize;N]=tmp.clone();
			tmp0[cursor]=0;
			res.append(&mut self.get_all_binvec(tmp0,cursor+1));
			let mut tmp1:[usize;N]=tmp.clone();
			tmp1[cursor]=1;
			res.append(&mut self.get_all_binvec(tmp1,cursor+1));
		}
		res
	}
	fn get_all_binvec_vec(&mut self,tmp:&Vec<usize>,cursor:usize,bitlen:usize)->Vec<Vec<usize>>{
		let mut res:Vec<Vec<usize>>=vec![];
		if cursor==bitlen{
			return vec![tmp.clone()];
		}
		else{
			let mut tmp0:Vec<usize>=tmp.clone();
			tmp0.push(0);
			res.append(&mut self.get_all_binvec_vec(&tmp0,cursor+1,bitlen));
			let mut tmp1:Vec<usize>=tmp.clone();
			tmp1.push(1);
			res.append(&mut self.get_all_binvec_vec(&tmp1,cursor+1,bitlen));
		}
		res
	}		
	fn get_poss_bp(&self,a:&[usize;N],b:&[usize;N],binvec:&[usize;N])->[usize;N]{
		let mut res:[usize;N]=[0;N];
		for i in 0..binvec.len(){
			if binvec[i]==0{
				res[i]=a[i];
			}
			else{
				res[i]=b[i];
			}
		}
		res
	}
	fn check_2_groups_integrity(&self,a:&Vec<usize>,b:&Vec<usize>)->bool{
		for i in 0..N{
			if a[i]==b[i]{
				return false;
			}
		}
		true
	}
	// returns true if bp DOES affect the stability of a and b, i.e. it return true if a and b are NOT stable for this bp!
	fn bp_compare(&self,vala:&[usize;N],valb:&[usize;N],val_bp:&[usize;N],binvec:&[usize;N])->bool{
		for i in 0..binvec.len(){
			if binvec[i]==0{
				if val_bp[i]>vala[i]{
					return false
				}
			}
			else{
				if val_bp[i]>valb[i]{
					return false
				}				
			}
		}
		true
	}
	fn bp_compare2(&self,vala:&[usize;N],valb:&[usize;N],val_bp:&[usize;N],binvec:&[usize;N])->bool{
		//:{:?}",binvec);
		for i in 0..binvec.len(){
			if binvec[i]==0{
				if val_bp[i]>vala[i]{
					return false
				}
			}
			else{
				if val_bp[i]>valb[i]{
					return false
				}				
			}
		}
		true
	}
	// TO TEST !!!!!!!
	/*
	fn is_stable2(&mut self,a:&Vec<usize>,b:&Vec<usize>)->bool{
		if !self.check_2_groups_integrity(&a,&b){
			return false;
		}
		let binvec:Vec<[usize;N]>=self.get_all_binvec([0;N],0);
		//println!("a:{:?}, b:{:?}",a,b);
		//println!("binvec:\n{:?}",binvec);
		let mut bpvec:Vec<[usize;N]>=vec![];
		for i in 1..binvec.len()-1{
			bpvec.push(self.get_poss_bp(&a,&b,&binvec[i]));
		}
		//println!("bpvec:\n{:?}",bpvec);
		let mut bpvec_endvalues:Vec<[usize;N]>=vec![];
		for i in 0..bpvec.len(){
			bpvec_endvalues.push(self.rank.retrieve_endvalue(&bpvec[i].to_vec(),0));
		}
		//println!("bpvec_endvalues:\n{:?}",bpvec_endvalues);
		let val_a:[usize;N]=self.rank.retrieve_endvalue(&a,0);
		let val_b:[usize;N]=self.rank.retrieve_endvalue(&b,0);
		//println!("val_a:{:?}, val_b:{:?}",val_a,val_b);
		for i in 0..bpvec.len(){
			// the last index is very important!!! the indices of binvec and bpvec_endvalues are completely shifted (by 1)!!!
			if self.bp_compare2(&val_a,&val_b,&bpvec_endvalues[i],&binvec[i+1]){
				return false;
			}
		}
		true
	}
	*/	
	fn is_stable3(&mut self,a:&[usize;N],b:&[usize;N])->bool{
		/*
		if !self.check_2_groups_integrity(&a,&b){
			return false;
		}
		*/
		let binvec:Vec<[usize;N]>=self.get_all_binvec([0;N],0);
		//println!("a:{:?}, b:{:?}",a,b);
		//println!("binvec:\n{:?}",binvec);
		let mut bpvec:Vec<[usize;N]>=vec![];
		for i in 1..binvec.len()-1{
			//bpvec.push(self.get_poss_bp(&a,&b,&binvec[i]));
			bpvec.push(self.get_poss_bp(&a,&b,&binvec[i]));
			//bpvec.push(self.get_poss_bp(a,b,&binvec[i]));
		}
		//println!("bpvec:\n{:?}",bpvec);
		let mut bpvec_endvalues:Vec<[usize;N]>=vec![];
		for i in 0..bpvec.len(){
			bpvec_endvalues.push(self.rank.retrieve_endvalue(&bpvec[i].to_vec(),0));
		}
		let val_a:[usize;N]=self.rank.retrieve_endvalue_arr(&a,0);
		let val_b:[usize;N]=self.rank.retrieve_endvalue_arr(&b,0);
		for i in 0..bpvec.len(){
			if self.bp_compare2(&val_a,&val_b,&bpvec_endvalues[i],&binvec[i+1]){
				return false;
			}
		}
		true
	}
	fn transform_stability_matrix_2usize(&mut self,bitlen:usize){
		self.bitmatrix=IOClass::bp_matrix_2usize_block2(&self.stabm,bitlen);
	}
	fn run(&self,tmp:&Vec<usize>,mask:Vec<usize>,icursor:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		if tmp.len()==self.maxn{
			return vec![tmp.to_vec()];
		}
		else{
			
		}
		matches
	}
	// We operate with cap vector (1-dimension), later we add cap matrix over the whole sets of agents (2-dimensions!)
	fn stable_sync_bs(&self,bp:&Vec<Vec<usize>>,/*posvec:&Vec<Vec<usize>>,*/
		mask:Vec<usize>,tmp:Vec<usize>,
		i_pos:usize,
		bs_:usize,
		n:usize
	)->Vec<Vec<usize>>{
		println!("sync tmp:{:?}",tmp);
		let mut matches:Vec<Vec<usize>>=vec![];
		let mut bs:usize=bs_;
		if tmp.len()==n{
			println!("\nEND {:?}\n",tmp);
			matches.push(tmp);
			return matches;
		}
		else{			
			let mut val_vec:Vec<usize>=vec![];			
			for i in 0..mask.len(){												
				let val:usize=bp[i_pos][i] & mask[i];
				val_vec.push(val);
			}
			for i in 0..val_vec.len(){
				let val:usize=val_vec[i];				
				if val>0{
					let next:&Vec<usize>=&self.ordervec_sync[val];
					for j in 0..next.len(){						
						let j_pos:usize=i*bs+next[j];						
						let mut tmp_j:Vec<usize>=tmp.clone();												
						tmp_j.push(j_pos);
						let mut matches_j=self.stable_sync_bs(&bp,/*&posvec,*/val_vec.clone(),tmp_j,j_pos,bs,n);
						matches.append(&mut matches_j);					
					}
				}
			}		
		}
		matches
	}
	fn get_min_val(&self,vec:&Vec<usize>,non_i:usize)->usize{
		let mut minval:usize=0;
		if vec.len()>0{
			minval=vec[0];
			for i in 1..vec.len(){
				if i != non_i{
					if minval>vec[i]{
						minval=vec[i];
					}
				}
			}
		}
		minval
	}
	fn check_caps(&self,group:&[usize;N],cap:&Vec<Vec<Vec<usize>>>)->bool{
		for i in 0..N{
			if self.get_min_val(&cap[i][group[i]],i)==0{
				return false;
			}
		}
		true
	}
	fn decrement_caps(&self,cap:&Vec<Vec<Vec<usize>>>,group:&[usize;N])->Vec<Vec<Vec<usize>>>{
		let mut res:Vec<Vec<Vec<usize>>>=cap.clone();
		for i in 0..N{
			for j in 0..res[i][group[i]].len(){
				if j != i{
					res[i][group[i]][j]-=1;
				}
			}
		}
		res
	}
	fn stable_sync_bs_cap_matrix(&self,
		bp:&Vec<Vec<usize>>,/*posvec:&Vec<Vec<usize>>,*/
		mask:Vec<usize>,
		tmp:Vec<usize>,
		cap:Vec<Vec<Vec<usize>>>,
		i_pos:usize,
		bs_:usize,
		n:usize
	)->Vec<Vec<usize>>{
		//println!("STABLE SYNC !!!!!!!");
		println!("sync tmp:{:?}",tmp);
		let mut matches:Vec<Vec<usize>>=vec![];
		let mut bs:usize=bs_;
		if tmp.len()==self.maxn{
			println!("\nEND {:?}\n",tmp);
			matches.push(tmp);
			return matches;
		}
		else{			
			let mut val_vec:Vec<usize>=vec![];			
			for i in 0..mask.len(){												
				let val:usize=bp[i_pos][i] & mask[i];
				val_vec.push(val);
			}
			for i in 0..val_vec.len(){
				let val:usize=val_vec[i];				
				if val>0{
					let next:&Vec<usize>=&self.ordervec_sync[val];
					for j in 0..next.len(){
						let j_pos:usize=i*bs+next[j];						
						let group:[usize;N]=Self::decompose(j_pos,&self.nvec);
						if self.check_caps(&group,&cap){
							let mut tmp_j:Vec<usize>=tmp.clone();												
							tmp_j.push(j_pos);
							// LATER: IT CAN BE IMPROVED BY REVERSING THIS OPERATION, SO WE WOULD HAVE INCREMENT AFTER DECREMENT!
							let new_cap=self.decrement_caps(&cap,&group);						
							let mut matches_j=self.stable_sync_bs_cap_matrix(&bp,/*&posvec,*/val_vec.clone(),tmp_j,new_cap,j_pos,bs,n);
							matches.append(&mut matches_j);
						}
					}
				}
			}		
		}
		matches
	}	
	fn stable_sync_bs_general(&self,bp:&Vec<Vec<usize>>,bs:usize,n:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		for i in 0..bp.len()-n{			
			let tmp:Vec<usize>=vec![i];
			println!("BS TEST {}",i);
			let mut matches_i=self.stable_sync_bs_cap_matrix(&bp,bp[i].clone(),tmp,self.capvec.clone(),i,bs,n);
			matches.append(&mut matches_i);		
		}
		println!("N-SIDED MATCHES STABLE SYNC BLOCKSIZE:\n{:?}",matches);
		matches
	}	
}



struct n_sided_matching{
	m_pref: Vec<Vec<[usize;N]>>,
	w_pref: Vec<Vec<[usize;N]>>,
	k_pref: Vec<Vec<[usize;N]>>,
	//pref_notranslated: Vec<Vec<Vec<[usize;N-1]>>>,
	pref: Vec<Vec<Vec<[usize;N]>>>,
	nvec:Vec<usize>,
	min_n:usize,
	max_n:usize,
	adj: rank_tree,
	all_groups:Vec<[usize;N]>,
	stability_matrix_bool:Vec<Vec<bool>>,
	stability_matrix_num:Vec<Vec<usize>>,
	boolvec_sync:Vec<Vec<bool>>,
	bitvec_sync:Vec<Vec<usize>>,
	ordervec_sync:Vec<Vec<usize>>,
	matches: Vec<Vec<[usize;N]>>
}
impl n_sided_matching {
	fn new()-> Self{
		Self{
			m_pref:vec![],
			w_pref:vec![],
			k_pref:vec![],
			//pref_notranslated:vec![],
			pref:vec![],
			nvec:vec![],
			min_n:0,
			max_n:1,
			adj:rank_tree::new(),
			all_groups:vec![],
			stability_matrix_bool:vec![],
			stability_matrix_num:vec![],
			boolvec_sync:vec![],
			bitvec_sync:vec![],
			ordervec_sync:vec![],
			matches:vec![]
		}
	}
	fn init(&mut self,n:usize){
		/* COMMENTED FOR TESTING PURPOSES !!!!!!!
		self.m_pref=Self::read_pref2("3D_PREF/m_3dpref.txt".to_string());
		self.w_pref=Self::read_pref2("3D_PREF/w_3dpref.txt".to_string());
		self.k_pref=Self::read_pref2("3D_PREF/k_3dpref.txt".to_string());
		let n:usize=self.m_pref.len();
		*/
		//self.pref_notranslated.push(self.m_pref.clone());
		//self.pref_notranslated.push(self.w_pref.clone());
		//self.pref_notranslated.push(self.k_pref.clone());
		//self.adj_matrix(self.pref_notranslated.clone(),n);
		
		//	COMMENTED!
		//self.pref=vec![self.m_pref.clone(),self.w_pref.clone(),self.k_pref.clone()];
		
		//let n:usize=7;
		self.pref=Self::create_random_pref2_stack(n);
		//self.pref=Self::read_all_pref("kpref/".to_string());
		self.nvec=vec![n;N];
		(self.min_n,self.max_n)=self.get_minmax_n(&self.nvec);
		self.adj_matrix2(n);
		self.all_groups=self.get_all_poss_groups(&[0;N],0,n);
		//println!("self.pref_notranslated:\n{:?}",self.pref_notranslated);
		//self.translate_pref(self.pref_notranslated.clone());
		println!("self.pref:\n{:?}",self.pref);
		
	}
	fn get_minmax_n(&self,nvec:&Vec<usize>)->(usize,usize){
		let mut min:usize=nvec[0];
		let mut max:usize=nvec[0];
		for i in 1..nvec.len(){
			if nvec[i]<min{
				
				min=nvec[i];
			}
			if nvec[i]>max{
				max=nvec[i];
			}
		}
		(min,max)
	}
	fn read_all_pref(folder:String)->Vec<Vec<Vec<[usize;N]>>>{
		let mut prefs:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for i in 0..N{
			//let mut filepath:String="nsided/pref_".to_string();
			let mut filepath:String="./kpref/pref_".to_string();
			filepath+=&i.to_string();
			filepath+=".txt";
			let pref:Vec<Vec<[usize;N]>>=Self::read_pref3(filepath);
			prefs.push(pref);
		}
		prefs
	}
	fn read_pref(txtfile:String)->Vec<Vec<[usize;2]>>{
		let mut pref:Vec<Vec<[usize;2]>>=vec![];		
		let path = Path::new(&txtfile);
		//println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			let mut pref_i:Vec<[usize;2]>=vec![];			
			let mut vecchar:Vec<char>=line.expect("").chars().collect();
			vecchar.remove(0);
			vecchar.remove(vecchar.len()-1);
			//println!("last:{:?}",vecchar);
			let tmp_string:String=vecchar.clone().into_iter().collect();
			for arr in tmp_string.split(']'){
				//println!("arr:{:?}",arr);
				let mut word:String=arr.to_string();
				let mut word_char:Vec<char>=word.chars().collect();
				//if arr.contains(&","){
				//if word.chars().collect().get(0)==','{
				if arr.len()>0{
					if word_char[0]==','{
						//word.remove(0);
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					if word_char[0]=='['{
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					// for 4-sided matching we have [0,0,0]. for 5-sided matching we have [0,0,0,0].
					let mut el:[usize;2]=[0,0];
					let mut idx:usize=0;
					for part in word.split(','){
						let mut word_part:String=part.to_string();
						if part.contains(&" "){
							word_part=(&word_part[1..word_part.len()]).to_string();
						}
						let digit = word_part.parse::<usize>().unwrap();
						el[idx]=digit;
						idx+=1;
						let part_chars:Vec<char>=word_part.chars().collect();
						//println!("part:{:?}",part_chars);
					}
					pref_i.push(el);
					//println!("word:{:?}\nword_char:{:?}",word,word_char);
				}
			}
			pref.push(pref_i);			
		}
		println!("PREF:\n{:?}",pref);
		pref
	}
	fn show_pref(&self){
		println!("SHOW PREFERENCE TABLES");
		for i in 0..self.pref.len(){
			println!("PREFERENCE OF {}",i);
			for j in 0..self.pref[i].len(){
				println!("{:?}",self.pref[i][j]);
			}
		}
	}
	// WRONG!
	// BE VERY CAREFUL !!!!!!!
	//fn local_search_blocking_coalition(&self,a:&[usize;N],b:&[usize;N])->bool{
	fn local_search_blocking_coalition(&self,chain:&Vec<[usize;N]>,b:&[usize;N])->bool{
		let mut stable:bool=true;
		for i in 0..chain.len(){
			let mut approved:usize=0;
			for j in 0..N{
				let pos:usize=self.pref[j][chain[i][j]].iter().position(|x| x==b).unwrap();
				if self.pref[j][chain[i][j]].iter().position(|x| x==b) < self.pref[j][chain[i][j]].iter().position(|x| *x==chain[i]){
					approved+=1;
				}				
				/*
				if position_N(&self.pref[j][chain[i][j]],&b) < position_N(&self.pref[j][chain[i][j]],&b){
					approved+=1;
				}
				*/
			}
			if approved==N{
				return false;
			}
		}
		true
	}
	
	// returns true if the chain is stable!
	fn local_search_blocking_coalition_intersection_wrapper(&self,chain:&Vec<[usize;N]>)->bool{
		for i in 0..chain.len(){
			let player:usize=chain[i][0];
			let pos:usize=self.pref[0][player].iter().position(|x| *x==chain[i]).unwrap();
			let intersct:&Vec<[usize;N]>=&self.pref[0][player][0..pos].to_vec();
			if !self.local_search_blocking_coalition_intersection(&chain,1,&intersct){
				return false;
			}
		}
		true
	}
	// can be improved (difference of chain-lenghts) but this is failsafe!
	// returns true if chain is stable!
	fn local_search_blocking_coalition_intersection(&self,chain:&Vec<[usize;N]>,idx:usize,intersct:&Vec<[usize;N]>)->bool{
		if idx==N{
			//return intersct.clone();
			if intersct.len()>0{
				return false;
			}
		}
		else{
			// no intersct, no blocking coalitions!
			if intersct.len()==0{
				return true;
			}
			else{
				for i in 0..chain.len(){
					let player:usize=chain[i][idx];
					let pos:usize=self.pref[idx][player].iter().position(|x| *x==chain[i]).unwrap();
					let slice:&Vec<[usize;N]>=&self.pref[idx][player][0..pos].to_vec(); // all coalitions that are more preferred than the current one.
					let intersct_i:Vec<[usize;N]>=Self::intersection_coalition(&intersct,&slice);
					if !self.local_search_blocking_coalition_intersection(&chain,idx+1,&intersct_i){
						return false;
					}	
				}
			}
		}		
		true
	}	
	fn decrement_quota(&self){
		
	}
	fn increment_quota(&self){
		
	}
	fn local_search_blocking_coalition_intersection_quota(&self,chain:&Vec<[usize;N]>,idx:usize,intersct:&Vec<[usize;N]>)->bool{
		if idx==N{
			//return intersct.clone();
			if intersct.len()>0{
				return false;
			}
		}
		else{
			// no intersct, no blocking coalitions!
			if intersct.len()==0{
				return true;
			}
			else{
				for i in 0..chain.len(){
					let player:usize=chain[i][idx];
					let pos:usize=self.pref[idx][player].iter().position(|x| *x==chain[i]).unwrap();
					let slice:&Vec<[usize;N]>=&self.pref[idx][player][0..pos].to_vec(); // all coalitions that are more preferred than the current one.
					let intersct_i:Vec<[usize;N]>=Self::intersection_coalition(&intersct,&slice);
					if !self.local_search_blocking_coalition_intersection_quota(&chain,idx+1,&intersct_i){
						return false;
					}	
				}
			}
		}		
		true
	}	

	fn intersection_usize(vec1:&Vec<usize>,vec2:&Vec<usize>)->Vec<usize>{
		let mut intersection:Vec<usize>=vec![];
		for i in 0..vec1.len(){
			if vec2.contains(&vec1[i]){
				intersection.push(vec1[i]);
			}
		}
		intersection
	}
	fn intersection_coalition(vec1:&Vec<[usize;N]>,vec2:&Vec<[usize;N]>)->Vec<[usize;N]>{
		let mut intersection:Vec<[usize;N]>=vec![];
		for i in 0..vec1.len(){
			if vec2.contains(&vec1[i]){
				intersection.push(vec1[i]);
			}
		}
		intersection
	}
	fn deepsearch(&self,chain:&Vec<[usize;N]>,tmp:&[usize;N],idx:usize,nvec:&Vec<usize>)->Vec<Vec<[usize;N]>>{
		//println!("chain:{:?}, tmp:{:?}, idx:{}, nvec:{:?}, self.min_n:{}, self.max_n:{}",chain,tmp,idx,nvec,self.min_n,self.max_n);
		let mut matches:Vec<Vec<[usize;N]>>=vec![];
		
		// MAX_N ?
		if chain.len()==self.min_n{
			return vec![chain.to_vec()];
		}
		else{
			if idx==N{
				if chain.len()==0{
						matches.append(&mut self.deepsearch(&vec![*tmp],&[0;N],0,&nvec));					
				}
				else{
					
					let mut tmp_chain:Vec<[usize;N]>=chain.clone();
					tmp_chain.push(*tmp);
					if self.local_search_blocking_coalition_intersection_wrapper(&tmp_chain){
						matches.append(&mut self.deepsearch(&tmp_chain,&[0;N],0,&nvec));
					}
				}
			}			
			else{
				for i in 0..nvec[idx]{
					let mut is_ok:bool=true;
					for j in 0..chain.len(){
						if chain[j][idx]==i{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						let mut new_tmp:[usize;N]=tmp.clone();
						new_tmp[idx]=i;
						matches.append(&mut self.deepsearch(&chain,&new_tmp,idx+1,&nvec));
					}
				}
			}
		}
		matches
	}
	fn remove_multiple_solutions(&self,matches:&Vec<Vec<[usize;N]>>)->Vec<Vec<[usize;N]>>{
		let mut clean:Vec<Vec<[usize;N]>>=vec![];
		if matches.len()>0{
			clean.push(matches[0].clone());
			for i in 1..matches.len(){
				let mut already_contained:bool=false;
				for j in 0..clean.len(){
					let mut already_count:usize=0;
					for k in 0..matches[i].len(){
						if clean[j].contains(&matches[i][k]){
							already_count+=1;
						}
					}
					if already_count==matches[0].len(){
						//clean.push(matches[i]);
						already_contained=true;
						break;
					}
				}
				if !already_contained{
					clean.push(matches[i].clone());
				}
			}
		}
		clean
	}
	fn stable_in_corpus(&self,head1:&[usize;N],head2:&[usize;N])->bool{
		
		true
	}
	
	fn decompose_pref_str(txt:String)->Vec<String>{
		let mut strvec:Vec<String>=vec![];
		/*
		for i in 0..txt.len(){
			
		}
		*/
		//for 
		strvec
	}
	fn get_prod_from_nvec(&self,nvec:&Vec<usize>)->usize{
		let mut prod:usize=nvec[0];
		for i in 1..nvec.len(){
			prod*=nvec[i];
		}
		prod
	}
	fn decompose(&self,idx:usize,nvec:&Vec<usize>)->[usize;N]{
		let mut prod:usize=self.get_prod_from_nvec(&nvec);		
		let mut rest:usize=idx;
		let mut node:[usize;N]=[0;N];
		//println!("rest:{}, node:{:?}, prod:{}",rest,node,prod);
		for i in 0..N{			
			//println!("i:{}, prod:{}, rest:{}, node:{:?}",i,prod,rest,node);
			prod/=nvec[i];
			node[i]=rest/prod;
			rest=rest-node[i]*prod;
			//println!("i:{}, prod:{}, rest:{}, node:{:?} AFTER",i,prod,rest,node);			
		}
		node
	}
	
	fn compose(&self,node:&[usize;N])->usize{
		let nvec:Vec<usize>=self.nvec.clone();
		let mut prod:usize=self.get_prod_from_nvec(&nvec);
		let mut idx:usize=0;
		for i in 0..N{
			prod/=nvec[i];
			idx+=node[i]*prod;
		}
		idx
	}
	
	fn lp_stability(&self)->Vec<Vec<i8>>{
		let mut lp:Vec<Vec<i8>>=vec![];
		let prod:usize=self.get_prod_from_nvec(&self.nvec);
		for i in 0..prod{
			let mut row:Vec<i8>=vec![0;prod];
			let node:[usize;N]=self.decompose(i,&self.nvec);
			row[i]=1;
			for j in 0..N{
				let player:usize=node[j];
				let pos:usize=self.pref[j][player].iter().position(|x| *x==node).unwrap();
				for k in 0..pos{
					let idx_k:usize=self.compose(&self.pref[j][player][k]);
					row[idx_k]=1;
				}
			}
			lp.push(row);
		}
		lp
	}
	fn lp_eq(&self,nvec:&Vec<usize>)->Vec<Vec<i8>>{
		let mut lp:Vec<Vec<i8>>=vec![];
		let mut prod:usize=self.get_prod_from_nvec(&nvec);
		let mut prod_right:usize=prod;
		let mut prod_left:usize=1;
		//println!("prod_right:{}, nvec:{:?}",prod_right,nvec);
		//println!("prod_right:{}, AFTER",prod_right);
		//println!("PROD: {}",prod);
		for i in 0..nvec.len(){
			//println!("i:{}, prod_left:{}, prod_right:{}",i,prod_left,prod_right);
			let n:usize=nvec[i];
			prod_right/=n;
			for j in 0..n{
				//println!("i:{}, j:{}",i,j);
				let mut row:Vec<i8>=vec![0;prod];
				for k in 0..prod_left{
					//println!("i:{}, j:{}, k:{}",i,j,k);
					for m in 0..prod_right{
						let val:usize=k*prod/prod_left+j*prod_right+m;																		
						row[val]=1;
					}
					//printlN!("val:{}, row:{:?}",k*prod/prod_left+j*prod_right,row);
				}
				lp.push(row);
			}			
			prod_left*=n;
		}
		println!("LP EQ");
		for i in 0..lp.len(){
			println!("{:?}",lp[i]);
		}
		lp
	}
	
	
	
	
	
	
	
	
	// get the gamma matrix
	fn lp_dual_A_ub(&self)->Vec<Vec<i8>>{
		let mut lp:Vec<Vec<i8>>=vec![];
		let prod:usize=self.get_prod_from_nvec(&self.nvec);
		for i in 0..prod{
			let mut row:Vec<i8>=vec![0;prod];
			row[i]=1;
			let coalition:[usize;N]=self.decompose(i,&self.nvec);
			for j in 0..N{
				let player:usize=coalition[j];
				let pos:usize=self.pref[j][player].iter().position(|x| *x==coalition).unwrap();
				// MORE PREFERRED!
				/*
				for k in 0..pos{
					let more_preferred:[usize;N]=self.pref[j][player][k];
					let idx_k:usize=self.compose(&more_preferred);
					row[idx_k]=1;
				}
				*/
				// LESS PREFERRED!
				if pos<self.pref[j][player].len()-1{
					for k in pos+1..self.pref[j][player].len(){
						let less_preferred:[usize;N]=self.pref[j][player][k];
						let idx_k:usize=self.compose(&less_preferred);
						row[idx_k]=1;
					}
				}
			}
			lp.push(row);
		}
		lp
	}
	// get the alpha, beta, ... matrix
	fn lp_dual_alphamatrix(&self)->Vec<Vec<i8>>{
		let mut lp:Vec<Vec<i8>>=vec![];
		let prod:usize=self.get_prod_from_nvec(&self.nvec);
		for i in 0..prod{
			/*
			let mut row:Vec<i8>=vec![0;prod];
			row[i]=1;
			lp.push(row);
			*/
			let coalition:[usize;N]=self.decompose(i,&self.nvec);
			let row:Vec<i8>=self.get_alpha_idx_from_coalition(&coalition,&self.nvec);
			lp.push(row);
		}
		lp
	}
	fn lp_dual_problemvariables(&self)->Vec<i8>{
		let sum:usize=self.get_sum_nvec(&self.nvec);
		let prod:usize=self.get_prod_from_nvec(&self.nvec);
		let mut c1:Vec<i8>=vec![1;sum];
		let mut c2:Vec<i8>=vec![-1;prod];
		c1.append(&mut c2);
		c1
	}
	// sum over nvec
	fn get_sum_nvec(&self,nvec:&Vec<usize>)->usize{
		let mut sum:usize=0;
		for i in 0..nvec.len(){
			sum+=nvec[i];
		}
		sum
	}
	fn get_alpha_idx_from_coalition(&self,coalition:&[usize;N],nvec:&Vec<usize>)->Vec<i8>{
		let mut idx:usize=0;
		let sum:usize=self.get_sum_nvec(&self.nvec);
		let mut row:Vec<i8>=vec![0;sum];
		let mut tmp_sum:usize=0;
		for i in 0..N{
			// be very careful!
			//idx+=coalition[i];
			row[tmp_sum+coalition[i]]=1;
			tmp_sum+=nvec[i];
		}
		row
	}

	// Both matrices A_ub and alphavec will be merged for the constraints.
	fn lp_dual_combine_A_ub_alphavec(&self)->Vec<Vec<i8>>{
		let mut lp:Vec<Vec<i8>>=vec![];
		let mut alphamatrix:Vec<Vec<i8>>=self.lp_dual_alphamatrix();
		let mut A_ub:Vec<Vec<i8>>=self.lp_dual_A_ub();
		for i in 0..A_ub.len(){
			let mut row:Vec<i8>=alphamatrix[i].clone();
			let mut neg_row:Vec<i8>=vec_i8_switch_2_negativity(&A_ub[i]);
			row.append(&mut neg_row);
			lp.push(row);
		}
		lp
	}
	// BUGGY ! SOLUTIONS NOT CORRECT!
	fn lp_dual_lp_highs_run(&self){
		let lp:Vec<Vec<i8>>=self.lp_dual_combine_A_ub_alphavec();
		let c:Vec<i8>=self.lp_dual_problemvariables();
		println!("DUAL LP HIGHS RUN");
		println!("LP-MATRIX");
		for i in 0..lp.len(){
			println!("{:?}",lp[i]);
		}
		println!("PROBLEM VARIABLES\n{:?}",c);
		//let c:Vec<usize>=vec![1;A_eq[0].len()];
		let mut pb = RowProblem::new();		
		let mut vars:Vec<Col>=vec![];
		for i in 0..c.len(){
			vars.push(pb.add_column(c[i] as f64* 1.0,0.0..));
		}
		for i in 0..lp.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..lp[i].len(){
				row.push((vars[j],lp[i][j] as f64));
			}
			//pb.add_row(..=b_eq[i] as f64,&row);	
			//pb.add_row(1..1,&row);	
			pb.add_row(1..,&row);	
		}
		/*
		for i in 0..A_eq_toggle.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_eq_toggle[i].len(){
				row.push((vars[j],A_eq_toggle[i][j] as f64));
			}
			//pb.add_row(..=b_eq[i] as f64,&row);	
			pb.add_row(..m.len() as f64 -1.0,&row);	
		}
		*/
		// the off vector holds if we have a quadratic matrix with n x n rows/columns!
		//let mut off:Vec<usize>=vec![];
		

		println!("DUAL MATCHING BP LP HIGHS");
		//let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		let solution = pb.optimise(Sense::Minimise).solve().get_solution();
		println!("DUAL SOLUTION:\n{:?}",solution);		
		//let matchres:Vec<usize>=transform_to_match(solution.columns(),mlen);
		//let matchres:Vec<[usize;N]>=self.transform_lp_2_node(&solution.columns().to_vec());
		let matchres:Vec<[usize;N]>=self.transform_lp_2_node(&solution.dual_rows().to_vec());
		println!("DUAL MATCH RESULT:\n{:?}",matchres);
		
		// DO IT CORRECTLY!
		if matchres.len()==4{
			let is_stable:bool=self.local_search_blocking_coalition_intersection_wrapper(&matchres);
			println!("DUAL SOLUTION IS: {}",is_stable);
		}
		//let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		let pos_idx:Vec<usize>=vec![];
		
	}
	//fn lp_run(&self,lp_ge:&Vec<Vec<i8>>,lp_eq:&Vec<Vec<i8>>){
	
	fn lp_run(&self){
//fn stability_bp_LP_HIGHS_IMPORTANT_fractional_test()->Vec<usize>{
		println!("####### STABILITY BP HIGHS FRACTIONAL TEST");
		println!("INTEGER SOLUTION BUT NO FRACTIONAL SOLUTION UNFORTUNATELY!");

		//let A_ub:Vec<Vec<i8>>=lp_ge.clone();
		let A_ub:Vec<Vec<i8>>=self.lp_stability();
		//let (mut c,mat):(Vec<i8>,Vec<Vec<i8>>)=self.fractional_setup_rothblum93_3();
		//let A_eq:Vec<Vec<usize>>=matchLP.get_a_eq();
		//let A_eq:Vec<Vec<usize>>=lp_eq.clone();
		//let A_eq:Vec<Vec<i8>>=lp_eq.clone();
		let A_eq:Vec<Vec<i8>>=self.lp_eq(&self.nvec);
		//let (m_A_eq,w_A_eq):(Vec<Vec<usize>>,Vec<Vec<usize>>)=matchLP.get_a_eq_2();
		//let c:Vec<usize>=vec![1;lp_ge[0].len()];
		let c:Vec<usize>=vec![1;A_eq[0].len()];
		let mut pb = RowProblem::new();		
		let mut vars:Vec<Col>=vec![];
		for i in 0..c.len(){
			vars.push(pb.add_column(1.0,0.0..));
		}
		for i in 0..A_eq.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_eq[i].len(){
				row.push((vars[j],A_eq[i][j] as f64));
			}
			//pb.add_row(..=b_eq[i] as f64,&row);	
			//pb.add_row(1..1,&row);	
			pb.add_row(..1,&row);	
		}
		/*
		for i in 0..A_eq_toggle.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_eq_toggle[i].len(){
				row.push((vars[j],A_eq_toggle[i][j] as f64));
			}
			//pb.add_row(..=b_eq[i] as f64,&row);	
			pb.add_row(..m.len() as f64 -1.0,&row);	
		}
		*/
		// the off vector holds if we have a quadratic matrix with n x n rows/columns!
		//let mut off:Vec<usize>=vec![];
		for i in 0..A_ub.len(){
			//if !off.contains(&i){
				let mut row:Vec<(Col,f64)>=vec![];
				for j in 0..A_ub[i].len(){
					row.push((vars[j],A_ub[i][j] as f64));
				}
				//pb.add_row(..0,&row);	
				//pb.add_row(..0,&row);	
				//pb.add_row(..m.len() as f64,&row);	
				//pb.add_row(0.0..(m.len()-1) as f64,&row);	
				

				// FRACTIONAL VALUES !!!!!!! VERY VERY IMPORTANT !!!!!!!
				pb.add_row(1.0..,&row);	
				
				//pb.add_row(..1.0,&row);	
			//}
		}

		println!("MATCHING BP LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);		
		//let matchres:Vec<usize>=transform_to_match(solution.columns(),mlen);
		let matchres:Vec<[usize;N]>=self.transform_lp_2_node(&solution.columns().to_vec());
		println!("MATCH RESULT:\n{:?}",matchres);
		
		// DO IT CORRECTLY!
		if matchres.len()==4{
			let is_stable:bool=self.local_search_blocking_coalition_intersection_wrapper(&matchres);
			println!("SOLUTION IS: {}",is_stable);
		}
		//let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		let pos_idx:Vec<usize>=vec![];
		//pos_idx	
//}				
	}
	
	/*
	fn lp_simplex_run(&self){
		println!("LP SIMPLEX RUN");
		let program = Simplex::minimize(&vec![-3.0, 1.0, -2.0]).with(vec![
			SimplexConstraint::LessThan(vec![2.0, -2.0, 3.0], 5.0),
			SimplexConstraint::LessThan(vec![1.0, 1.0, -1.0], 3.0),
			SimplexConstraint::LessThan(vec![1.0, -1.0, 1.0], 2.0),
		]);
		let simplex = program.unwrap();		
		let A_eq_=self.lp_eq(&self.nvec);
		let A_ub_=self.lp_stability();
		let A_eq:Vec<Vec<f64>>=mat_i8_2_f64(&A_eq_);
		let A_ub=mat_i8_2_f64(&A_ub_);
		//let mut c:Vec<f64>=vec![1.0;A_eq[0].len()];
		let mut c:Vec<f64>=vec![-1.0;A_eq[0].len()];
		
		let mut constraints:Vec<SimplexConstraint>=vec![];
		// A_ub
		for i in 0..A_ub.len(){
			//constraints.push(SimplexConstraint::GreaterThan(A_ub[i],1.0));
			constraints.push(SimplexConstraint::LessThan(vec_switch_2_negativity(&A_ub[i]),-1.0));
		}
		// A_eq
		for i in 0..A_eq.len(){
			//constraints.push(SimplexConstraint::Equal(A_eq[i],1.0));			
			constraints.push(SimplexConstraint::Equal(vec_i8_2_f64(&A_eq_[i]),1.0));			
		}
		//let lp_program=Simplex::maximize(&c).with(constraints);
		let lp_program=Simplex::minimize(&c).with(constraints);
		let mut lp_simplex=lp_program.unwrap();
		println!("SIMPLEX SOLUTION:\n{:?}",lp_simplex.solve());
		let mut solution:Vec<Option<f64>>=vec![];
		for i in 0..A_eq[0].len(){
			solution.push(lp_simplex.get_var(i));
		}
		println!("SIMPLEX SOLUTION VAR:\n{:?}",solution);
	}
	*/
	fn lp_rustplex_run(&self)->Result<(), SolverError>{
		let mut model=Model::new();
		let A_eq=self.lp_eq(&self.nvec);
		let A_ub=self.lp_stability();
		let mut var_key:Vec<VariableKey>=vec![];
		for i in 0..A_eq[0].len(){
			let var=model.add_variable().non_negative().real();
			var_key.push(var);
		}		
		// A_eq
		for i in 0..A_eq.len(){
			let mut expr:LinearExpr<VariableKey>=LinearExpr::new();
			//expr+=A_eq[i][0].into();
			expr+=var_key[0].into();
			for j in 1..A_eq[i].len(){
				expr+=var_key[j]*A_eq[i][j].into();
			}
			model.add_constraint(expr).eq(1.0);
		}
		for i in 0..A_ub.len(){
			let mut expr:LinearExpr<VariableKey>=LinearExpr::new();
			//expr+=A_eq[i][0].into();
			expr+=var_key[0].into();
			for j in 1..A_ub[i].len(){
				expr+=var_key[j]*A_ub[i][j].into();
			}
			model.add_constraint(expr).ge(1.0);
		}
		let mut obj:LinearExpr<VariableKey>=LinearExpr::new();
		for i in 0..var_key.len(){
			obj+=var_key[i].into();
		}
		model.set_objective(Maximize,obj);
		//model.set_objective(Minimize,obj);
		let mut f64_solution:Vec<f64>=vec![];
		let solution=model.solve()?;
		if solution.status().is_optimal() {
			println!("Objective Value: {}", solution.objective_value().unwrap());
			for i in 0..var_key.len(){
				f64_solution.push(solution[var_key[i]]);
			}
			//println!("var_key[0]:{:?}",solution[var_key[0]]);

			// Print full detailed report
			
			println!("{}", model.format(&solution));
		} else {
			println!("Solver failed: {}", solution.status());
		}
		println!("SOLUTION OF RUSTPLEX:\n{:?}",f64_solution);
		Ok(())
	}
	fn lp_rustplex_test(&self)->Result<(), SolverError>{
		let mut model = Model::new();
		//let x1 = rustplex::modeling::model::add_variable().name("x1").bounds(2.0..=5.0).real();
		let x1 = model.add_variable().name("x1").bounds(2.0..=5.0).real();
		let x2 = model.add_variable().name("x2").non_negative().real();
		let x3 = model.add_variable().name("x3").upper_bound(1.0).real();
		let x4 = model.add_variable().name("x4").real(); // Unbounded (free)

		// BOTH WORKS!
		let mut var:Vec<rustplex::modeling::variable::Variable>=vec![];
		let mut var_key:Vec<VariableKey>=vec![];
		//let key:rustplex::common::expression::ExprVariable=VariableKey::new();
		//let mut var_key:Vec<VariableKey>=vec![VariableKey::new()];
		let mut var_bd:Vec<rustplex::modeling::variable::VariableBuilder>=vec![];
		var_key.push(x1);
		

		// 3. Set the objective function: Maximize x1 + x2 + x3 - x4
		model.set_objective(
			Maximize,
			x1 + x2 + x3 - x4,
		);
		
		let mut own_expr:LinearExpr<VariableKey>=LinearExpr::new();
		own_expr+=var_key[0].into();

		// 4. Add constraints using natural syntax
		//    x1 + x3 <= x2
		model.add_constraint(x1 + x3).le(x2);

		//    x2 + x3 == 5.0
		model.add_constraint(x2 + x3).eq(5.0);

		//    x4 + x1 >= 10.0
		model.add_constraint(x4 + x1).ge(10.0);

		// 5. Solve the model
		//let solution = model.solve()?;
		let solution = model.solve()?;

		// 6. Inspect the results
		if solution.status().is_optimal() {
			println!("Objective Value: {}", solution.objective_value().unwrap());

			// Retrieve variable values safely
			println!("x1 = {}", solution[x1]);
			println!("x2 = {}", solution[x2]);

			println!("var_key[0]:{:?}",solution[var_key[0]]);

			// Print full detailed report
			println!("{}", model.format(&solution));
		} else {
			println!("Solver failed: {}", solution.status());
		}

		Ok(())
		//}
	}
	fn transform_lp_2_node(&self,sol:&Vec<f64>)->Vec<[usize;N]>{
		let mut transf:Vec<[usize;N]>=vec![];
		for i in 0..sol.len(){
			if sol[i]>0.0{
				let node:[usize;N]=self.decompose(i,&self.nvec);
				transf.push(node);
			}
		}
		println!("TRANSFORMED SOLUTION:\n{:?}",transf);
		transf
		
	}
	fn read_pref2(txtfile:String)->Vec<Vec<[usize;N]>>{
		let mut pref:Vec<Vec<[usize;N]>>=vec![];		
		let path = Path::new(&txtfile);
		//println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			let mut pref_i:Vec<[usize;N]>=vec![];
			let mut vecchar:Vec<char>=line.expect("").chars().collect();
			vecchar.remove(0);
			vecchar.remove(vecchar.len()-1);
			//println!("last:{:?}",vecchar);
			let tmp_string:String=vecchar.clone().into_iter().collect();
			for arr in tmp_string.split(']'){
				//println!("arr:{:?}",arr);
				let mut word:String=arr.to_string();
				let mut word_char:Vec<char>=word.chars().collect();
				//if arr.contains(&","){
				//if word.chars().collect().get(0)==','{
				if arr.len()>0{
					if word_char[0]==','{
						//word.remove(0);
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					if word_char[0]=='['{
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					// for 4-sided matching we have [0,0,0]. for 5-sided matching we have [0,0,0,0].
					let mut el:[usize;N]=[0;N];
					let mut idx:usize=0;
					for part in word.split(','){
						let mut word_part:String=part.to_string();
						if part.contains(&" "){
							word_part=(&word_part[1..word_part.len()]).to_string();
						}
						let digit = word_part.parse::<usize>().unwrap();
						el[idx]=digit;
						idx+=1;
						let part_chars:Vec<char>=word_part.chars().collect();
						//println!("part:{:?}",part_chars);
					}
					pref_i.push(el);
					//println!("word:{:?}\nword_char:{:?}",word,word_char);
				}
			}
			pref.push(pref_i);			
		}
		println!("PREF:\n{:?}",pref);
		pref
	}
	fn read_pref3(txtfile:String)->Vec<Vec<[usize;N]>>{
		let mut pref:Vec<Vec<[usize;N]>>=vec![];		
		let path = Path::new(&txtfile);
		println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			let mut pref_i:Vec<[usize;N]>=vec![];
			let mut vecchar:Vec<char>=line.expect("").chars().collect();
			vecchar.remove(0);
			vecchar.remove(vecchar.len()-1);
			//println!("last:{:?}",vecchar);
			let tmp_string:String=vecchar.clone().into_iter().collect();
			for arr in tmp_string.split(']'){
				//println!("arr:{:?}",arr);
				let mut word:String=arr.to_string();
				let mut word_char:Vec<char>=word.chars().collect();
				//if arr.contains(&","){
				//if word.chars().collect().get(0)==','{
				if arr.len()>0{
					if word_char[0]==','{
						//word.remove(0);
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					if word_char[0]=='['{
						word=word[1..word.len()].to_string();
						word_char.remove(0);
					}
					// for 4-sided matching we have [0,0,0]. for 5-sided matching we have [0,0,0,0].
					let mut el:[usize;N]=[0;N];
					let mut idx:usize=0;
					for part in word.split(','){
						let mut word_part:String=part.to_string();
						if part.contains(&" "){
							word_part=(&word_part[1..word_part.len()]).to_string();
						}
						if word_part.contains(&"["){
							word_part=word_part[1..word_part.len()].to_string();
						}
						println!("word:{:?}, word_part:{:?}",word,word_part);
						let digit = word_part.parse::<usize>().unwrap();
						el[idx]=digit;
						idx+=1;
						let part_chars:Vec<char>=word_part.chars().collect();
						//println!("part:{:?}",part_chars);
					}
					pref_i.push(el);
					//println!("word:{:?}\nword_char:{:?}",word,word_char);
				}
			}
			pref.push(pref_i);			
		}
		println!("PREF:\n{:?}",pref);
		pref
	}

	fn create_random_pref(n:usize)->Vec<Vec<[usize;2]>>{
		let mut pref:Vec<Vec<[usize;2]>>=vec![];
		for el in 0..n{
			let mut pref_i:Vec<[usize;2]>=vec![];
			for i in 0..n{
				for j in 0..n{
					pref_i.push([i,j]);
				}
			}
			let mut rng = rand::rng();
			pref_i.shuffle(&mut rng);
			pref.push(pref_i);
			println!("{:?}", pref[el]);
		}		
		pref		
	}
	fn get_all_groups_pref_i(n:usize,pcursor:usize,pval:usize,tmp:&[usize;N],cursor:usize)->Vec<[usize;N]>{
		let mut res:Vec<[usize;N]>=vec![];
		if cursor==N{
			return vec![*tmp];
		}
		else{
			let mut new_tmp:[usize;N]=tmp.clone();
			if cursor != pcursor{
				for i in 0..n{
					new_tmp[cursor]=i;
					res.append(&mut Self::get_all_groups_pref_i(n,pcursor,pval,&new_tmp,cursor+1));
				}
			}
			else{
				new_tmp[pcursor]=pval;
				res.append(&mut Self::get_all_groups_pref_i(n,pcursor,pval,&new_tmp,cursor+1));
			}
		}
		res
	}
	fn create_random_pref2(n:usize,pcursor:usize)->Vec<Vec<[usize;N]>>{
		let mut pref:Vec<Vec<[usize;N]>>=vec![];
		for i in 0..n{
			let mut pref_i:Vec<[usize;N]>=Self::get_all_groups_pref_i(n,pcursor,i,&[0;N],0);
			let mut rng=rand::rng();
			pref_i.shuffle(&mut rng);
			pref.push(pref_i);			
		}
		pref
	}
	fn create_random_pref2_stack(n:usize)->Vec<Vec<Vec<[usize;N]>>>{
		let mut prefs:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for i in 0..N{
			prefs.push(Self::create_random_pref2(n,i));
		}
		prefs
	}
	fn write_pref(matrix:&Vec<Vec<[usize;N]>>,path:String){
		fn matrix_string(a:&Vec<[usize;N]>)->String{
			let mut res:String=String::new();
			res="[".to_string();
			for i in 0..a.len(){
				let result_i = format!("[{}]", a[i].iter()
					.map(|&x| x.to_string())
					.collect::<Vec<String>>().join(", "));
				res+=&result_i;
				if i<a.len()-1{
					res+=",";
				}
			}
			res+="]";
			return res;
		}				
		let mut fs = File::create(path);
		let mut txt = String::new();
		for i in 0..matrix.len() {
			txt+=&matrix_string(&matrix[i]);
			if i<matrix.len()-1{
				txt+="\n";
			}
		}
		fs.expect("Problem with the file!").write_all(txt.as_bytes());
	}
	fn create_samples(n:usize,samplesize:usize){
		for i in 0..samplesize{
			
		}
	}
	fn translate_pref(&mut self,tmp_pref:Vec<Vec<Vec<[usize;N-1]>>>){
		//let mut pcursor:usize=0;
		for pcursor in 0..tmp_pref.len(){
			let mut pref:Vec<Vec<[usize;N]>>=vec![];
			let n:usize=tmp_pref[pcursor].len();
			for i in 0..n{
				let mut pref_i:Vec<[usize;N]>=vec![]; 
				for j in 0..tmp_pref[pcursor][i].len(){
					let arr:[usize;N-1]=tmp_pref[pcursor][i][j];
					let mut new_arr:[usize;N]=[n;N];
					new_arr[pcursor]=i;
					for k in 0..arr.len(){
						if k<pcursor{
							new_arr[k]=arr[k];
						}
						else{
							new_arr[k+1]=arr[k];
						}
					}
					pref_i.push(new_arr);
				}
				pref.push(pref_i);
			}
			self.pref.push(pref);
		}
	}
	fn check_pref_integrity(){
		
	}
	// N-sided matching: Tree structure! later....!!!!!!! NOW!!!!!!!
	// WORKS!!!!!!!
	//fn adj_matrix(&mut self,pref:&Vec<Vec<Vec<[usize;N_SIDED-1]>>>,n:usize){
	fn adj_matrix(&mut self,pref:Vec<Vec<Vec<[usize;N-1]>>>,n:usize){
		//let mut adj:Vec
		let mut adj:rank_tree=rank_tree::new();
		adj.rank_initialize(3,n);
		//adj.rank_update(1,2,0,[1,0]);
		//adj.rank_update([1,0],2,0,[1,0]);
		for i in 0..pref.len(){
			for j in 0..pref[i].len(){
				for k in 0..pref[i][j].len(){
					let val:usize=pref[i][j].iter().position(|&x| x==pref[i][j][k]).expect("");
					adj.rank_update([i,j],val,0,pref[i][j][k]);
				}
			}
		}
		dbg!(&adj);
		self.adj=adj;
	}
	fn adj_matrix2(&mut self,n:usize){
		//let mut adj:Vec
		let mut adj:rank_tree=rank_tree::new();
		adj.rank_initialize(N,n);
		//adj.rank_update(1,2,0,[1,0]);
		//adj.rank_update([1,0],2,0,[1,0]);
		for i in 0..self.pref.len(){
			for j in 0..self.pref[i].len(){
				for k in 0..self.pref[i][j].len(){
					let val:usize=self.pref[i][j].iter().position(|&x| x==self.pref[i][j][k]).expect("");
					adj.rank_update2([i,j],val,0,self.pref[i][j][k]);
				}
			}
		}
		dbg!(&adj);
		self.adj=adj;		
	}
	
	fn get_all_binvec(&mut self,tmp:[usize;N],cursor:usize)->Vec<[usize;N]>{
		let mut res:Vec<[usize;N]>=vec![];
		//if tmp.len()==N_SIDED{
		if cursor==N{
			return vec![tmp.clone()];
		}
		else{
			let mut tmp0:[usize;N]=tmp.clone();
			tmp0[cursor]=0;
			res.append(&mut self.get_all_binvec(tmp0,cursor+1));
			let mut tmp1:[usize;N]=tmp.clone();
			tmp1[cursor]=1;
			res.append(&mut self.get_all_binvec(tmp1,cursor+1));
		}
		res
	}
	fn get_all_binvec_vec(&mut self,tmp:&Vec<usize>,cursor:usize,bitlen:usize)->Vec<Vec<usize>>{
		let mut res:Vec<Vec<usize>>=vec![];
		//if tmp.len()==N_SIDED{
		//if cursor==N_SIDED{
		if cursor==bitlen{
			return vec![tmp.clone()];
		}
		else{
			//let mut tmp0:[usize;N_SIDED]=tmp.clone();
			let mut tmp0:Vec<usize>=tmp.clone();
			//tmp0[cursor]=0;
			tmp0.push(0);
			res.append(&mut self.get_all_binvec_vec(&tmp0,cursor+1,bitlen));
			//let mut tmp1:[usize;N_SIDED]=tmp.clone();
			let mut tmp1:Vec<usize>=tmp.clone();
			//tmp1[cursor]=1;
			tmp1.push(1);
			res.append(&mut self.get_all_binvec_vec(&tmp1,cursor+1,bitlen));
		}
		res
	}
	
	fn get_poss_bp(&self,a:&Vec<usize>,b:&Vec<usize>,binvec:&[usize;N])->[usize;N]{
		let mut res:[usize;N]=[0;N];
		for i in 0..binvec.len(){
			if binvec[i]==0{
				res[i]=a[i];
			}
			else{
				res[i]=b[i];
			}
		}
		res
	}
	//fn check_2_groups_integrity(&self,a:[usize;N],b:&[usize;N])->bool{
	fn check_2_groups_integrity(&self,a:&Vec<usize>,b:&Vec<usize>)->bool{
		//println!("a:{:?}, b:{:?}",a,b);
		for i in 0..N{
			//println!("i:{}",i);
			//println!("a[{}]:{}, b[{}]:{}",i,a[i],i,b[i]);
			if a[i]==b[i]{
				return false;
			}
		}
		true
	}
	// returns true if bp DOES affect the stability of a and b, i.e. it return true if a and b are NOT stable for this bp!
	fn bp_compare(&self,vala:&[usize;N],valb:&[usize;N],val_bp:&[usize;N],binvec:&[usize;N])->bool{
		for i in 0..binvec.len(){
			if binvec[i]==0{
				if val_bp[i]>vala[i]{
					return false
				}
			}
			else{
				if val_bp[i]>valb[i]{
					return false
				}				
			}
		}
		true
	}
	fn bp_compare2(&self,vala:&[usize;N],valb:&[usize;N],val_bp:&[usize;N],binvec:&[usize;N])->bool{
		//:{:?}",binvec);
		for i in 0..binvec.len(){
			if binvec[i]==0{
				if val_bp[i]>vala[i]{
					return false
				}
			}
			else{
				if val_bp[i]>valb[i]{
					return false
				}				
			}
		}
		true
	}
	
	//fn stable(&mut self,a:&[usize;N_SIDED],b:&[usize;N_SIDED]){
	// VERY BUGGY !!!!!!!
	fn is_stable(&mut self,a:&Vec<usize>,b:&Vec<usize>)->bool{
		if !self.check_2_groups_integrity(&a,&b){
			return false;
		}
		let binvec:Vec<[usize;N]>=self.get_all_binvec([0;N],0);
		//println!("binvec:\n{:?}",binvec);
		let mut bpvec:Vec<[usize;N]>=vec![];
		for i in 1..binvec.len()-1{
			bpvec.push(self.get_poss_bp(&a,&b,&binvec[i]));
		}
		println!("bpvec:\n{:?}",bpvec);
		let mut bpvec_endvalues:Vec<[usize;N]>=vec![];
		for i in 0..bpvec.len(){
			bpvec_endvalues.push(self.adj.retrieve_endvalue(&bpvec[i].to_vec(),0));
		}
		println!("bpvec_endvalues:\n{:?}",bpvec_endvalues);
		let val_a:[usize;N]=self.adj.retrieve_endvalue(&a,0);
		let val_b:[usize;N]=self.adj.retrieve_endvalue(&b,0);
		for i in 0..bpvec.len(){
			if self.bp_compare(&val_a,&val_b,&bpvec_endvalues[i],&binvec[i]){
				return false;
			}
		}
		true
	}
	// TO TEST !!!!!!!
	fn is_stable2(&mut self,a:&Vec<usize>,b:&Vec<usize>)->bool{
		if !self.check_2_groups_integrity(&a,&b){
			return false;
		}
		let binvec:Vec<[usize;N]>=self.get_all_binvec([0;N],0);
		//println!("a:{:?}, b:{:?}",a,b);
		//println!("binvec:\n{:?}",binvec);
		let mut bpvec:Vec<[usize;N]>=vec![];
		for i in 1..binvec.len()-1{
			bpvec.push(self.get_poss_bp(&a,&b,&binvec[i]));
		}
		//println!("bpvec:\n{:?}",bpvec);
		let mut bpvec_endvalues:Vec<[usize;N]>=vec![];
		for i in 0..bpvec.len(){
			bpvec_endvalues.push(self.adj.retrieve_endvalue(&bpvec[i].to_vec(),0));
		}
		//println!("bpvec_endvalues:\n{:?}",bpvec_endvalues);
		let val_a:[usize;N]=self.adj.retrieve_endvalue(&a,0);
		let val_b:[usize;N]=self.adj.retrieve_endvalue(&b,0);
		//println!("val_a:{:?}, val_b:{:?}",val_a,val_b);
		for i in 0..bpvec.len(){
			// the last index is very important!!! the indices of binvec and bpvec_endvalues are completely shifted (by 1)!!!
			if self.bp_compare2(&val_a,&val_b,&bpvec_endvalues[i],&binvec[i+1]){
				return false;
			}
		}
		true
	}

	// later: instead of n, we have a vector for different |S| in Si!
	fn get_groupvec(&self,tmp:&Vec<[usize;N]>,vert:&Vec<Vec<usize>>,groupcursor:&Vec<usize>,cursor:usize,n:usize)->Vec<Vec<usize>>{
		//let mut arr:[Vec<usize>;N_SIDED]=[vec![0];N_SIDED];
		let mut res:Vec<Vec<usize>>=vec![];
		if groupcursor.len()==N{
			return vec![groupcursor.to_vec()];
		}
		else{
			for i in 0..n{
				if !vert[cursor].contains(&i){
					let mut groupcursor_i:Vec<usize>=groupcursor.clone();
					groupcursor_i.push(i);
					res.append(&mut self.get_groupvec(&tmp,&vert,&groupcursor_i,cursor+1,n));
				}
			}
			//return vec![groupcursor];
			return res;
		}
	}
	//fn check_group_integrity(&self,groupvec:&Vec<Vec<[usize;N]>>,group:[usize;N])->bool{
	fn check_group_integrity(&self,group_arr:&mut [Option<Vec<usize>>;N],group:[usize;N])->bool{
		let mut ok:bool=true;
		
		// this is just for tests whether vec can be part of an array!
		/*
		//let mut arr:[Vec<usize>;N]=[vec![0],vec![0],vec![0]];
		let mut arr:[Option<Vec<usize>>;N]=[const{None};N];
		for i in 0..N{
			arr[i]=Some(vec![]);
		}
		// works! test what is faster!!!
		for i in 0..N{
			for j in 0..4{
				//arr[i].clone().expect("").push(j);
				arr[i].as_mut().expect("").push(j);
			}
		}
		*/
		
		//for i in 0..groupvec.len(){
		for i in 0..N{
			if group_arr[i].as_mut().expect("").contains(&group[i]){
			//if group_arr[i].clone().expect("").contains(&group[i]){
				ok=false;
				break;
			}
		}
		ok
	}
	// assumption: n is equal for all sets S1,S2,....,S_N!
	fn get_all_poss_groups(&mut self,tmp:&[usize;N],cursor:usize,n:usize)->Vec<[usize;N]>{
		let mut res:Vec<[usize;N]>=vec![];
		if cursor==N{
			return vec![tmp.clone()];
		}
		else{
			for i in 0..n{
				let mut new_tmp=tmp.clone();
				new_tmp[cursor]=i;
				res.append(&mut self.get_all_poss_groups(&new_tmp,cursor+1,n));
			}
		}
		//println!("res(all_poss_groups):\n{:?}",res);
		//println!("total len: {}",res.len());
		res
	}
	fn create_stability_matrix(&mut self,n:usize)->Vec<Vec<bool>>{
		let mut stabm:Vec<Vec<bool>>=vec![];
		let grp_vec:Vec<[usize;N]>=self.get_all_poss_groups(&[0;N],0,n);
		for i in 0..grp_vec.len(){
			let mut stabm_i:Vec<bool>=vec![];
			let a:Vec<usize>=grp_vec[i].to_vec();
			for j in 0..grp_vec.len(){
				let mut stable:bool=false;
				let b:Vec<usize>=grp_vec[j].to_vec();
				if self.is_stable2(&a,&b){
					stable=true;
				}
				stabm_i.push(stable);
			}
			stabm.push(stabm_i);
		}
		stabm=bp_matrix_half(&stabm);
		stabm
	}
	//fn convert_stability_matrix(&mut self,stabm:&Vec<Vec<bool>>/*,bit:usize,binvec:&Vec<Vec<usize>>*/)->Vec<usize>{
	fn convert_stability_matrix(&mut self,stabm:&Vec<Vec<bool>>/*,bit:usize,binvec:&Vec<Vec<usize>>*/,bitlen:usize)->Vec<Vec<usize>>{
		//let bitlen:usize=stabm.len();
		//let bitlen:usize=8;
		//let mut stabm_num:Vec<usize>=vec![];
		let mut stabm_num:Vec<Vec<usize>>=vec![];
		//let stabm_bit:Vec<Vec<usize>>=
		let bitvec:Vec<Vec<usize>>=self.get_all_binvec_vec(&vec![],0,bitlen);
		//let mut bitvec_small:Vec<Vec<usize>>=vec![];
		let mut upperbound:usize=stabm.len()/bitlen;
		let mut modulo:usize=stabm.len()-upperbound*bitlen;
		if modulo>0{
			upperbound+=1;
			//bitvec_small=self.get_all_binvec_vec(&vec![],0,modulo);
		}
		let mut bool_vec:Vec<Vec<bool>>=vec![];
		
		// bool_vec_small should be avoided since it leads to completely different values!
		//let mut bool_vec_small:Vec<Vec<bool>>=vec![];
		for i in 0..bitvec.len(){
			let bool_vec_i:Vec<bool>=self.transform_bin2bool_vec(&bitvec[i]);
			bool_vec.push(bool_vec_i);
		}
		self.bitvec_sync=bitvec.clone();
		self.boolvec_sync=bool_vec.clone();
		self.ordervec_sync=self.get_ordervec(&self.boolvec_sync);
		/*
		for i in 0..bitvec_small.len(){
			let bool_vec_small_i:Vec<bool>=self.transform_bin2bool_vec(&bitvec_small[i]);
			bool_vec_small.push(bool_vec_small_i);
		}
		*/		
		for i in 0..stabm.len(){
			// this was for Vec<usize>, i.e. the whole bool row was represented as ONE usize vale, not a vec!
			/*
			let pos:usize=bool_vec.iter().position(|x| *x==stabm[i]).unwrap();
			stabm_num.push(pos);
			*/
			
			//  THINK TWICE!!!
			let mut stabm_i:Vec<usize>=vec![];
			// BE CAREFUL !!! THINK OF THE REST!!!!
			//if stabm.len()/bitlen-upperbound>0{
			println!("upperbound:{}, modulo:{}, stabm.len()/bitlen:{}",upperbound,modulo,stabm.len()/bitlen);
			//for j in 0..stabm.len()/bitlen{
			for j in 0..upperbound{
				println!("stabm.len():{}, bitlen:{}, i:{}, j:{}",stabm.len(),bitlen,i,j);
				let mut upper_slice:usize=(j+1)*bitlen;
				if j==upperbound-1 && modulo>0{
					//upper_slice=j*bitlen+modulo;
					upper_slice=stabm.len();
				}
				//let slice:Vec<bool>=stabm[i][j*bitlen..(j+1)*bitlen].to_vec();
				let mut slice:Vec<bool>=stabm[i][j*bitlen..upper_slice].to_vec();
				let mut pos:usize=0;
				println!("slice:{:?}",slice);
				//println!("boolvec_small:\n{:?}",bool_vec_small);
				/*
				if j==upperbound-1 && modulo>0{
					pos=bool_vec_small.iter().position(|x| *x==slice).unwrap();					
				}
				else{
					pos=bool_vec.iter().position(|x| *x==slice).unwrap();
				}
				*/
				if j==upperbound-1 && modulo>0{
					//pos=bool_vec_small.iter().position(|x| *x==slice).unwrap();
					for k in 0..bitlen-modulo{
						slice.push(false);
					}
				}
				else{
				}
				pos=bool_vec.iter().position(|x| *x==slice).unwrap();

				stabm_i.push(pos);
			}
			stabm_num.push(stabm_i);
		}
		println!("stabm_num:\n{:?}",stabm_num);
		stabm_num
	}
	fn transform_bin2bool_vec(&self,bin:&Vec<usize>)->Vec<bool>{
		let mut bool_vec:Vec<bool>=vec![];
		for i in 0..bin.len(){
			if bin[i]==0{
				bool_vec.push(false);
			}
			else{
				bool_vec.push(true);
			}
		}
		bool_vec
	}
	fn get_ordervec(&self,boolvec:&Vec<Vec<bool>>)->Vec<Vec<usize>>{
		let mut ordervec:Vec<Vec<usize>>=vec![];
		for i in 0..boolvec.len(){
			let mut ordervec_i:Vec<usize>=vec![];
			for j in 0..boolvec[i].len(){
				if boolvec[i][j]{
					ordervec_i.push(j);
				}
			}
			ordervec.push(ordervec_i);
		}
		ordervec
	}
	fn stable_sync(&self,bp:&Vec<usize>,mask:usize,tmp:Vec<usize>,i_pos:usize,bs:usize)->Vec<Vec<usize>>{
		//println!("STABLE SYNC !!!!!!!");
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		//if i_pos==n{
		if tmp.len()==n{
			println!("\nEND {:?}\n",tmp);
			//if bp[i_pos]&mask>0{
				matches.push(tmp);
			//}
			return matches;
		}
		else{
			println!("mask: {}",mask);
			let val:usize=bp[i_pos] & mask;
			println!("val: {}",val);
			if val>0{
			//let next:Vec<usize>=get_id_from_num(bp[i_pos],64);
			let next:Vec<usize>=get_id_from_num(val,n*n);
			println!("next {:?}",next);
			for i in 0..next.len(){
				
							
				let mut tmp_i:Vec<usize>=tmp.clone();
				
				
				tmp_i.push(next[i]);
				
				//matches.append(&mut stable_sync(&bp,val,tmp_i,i_pos+1,bs));
				//matches.append(&mut stable_sync(&bp,val,tmp_i,next[i],bs));
				let mut matches_i=stable_sync(&bp,val,tmp_i,next[i],bs);
				matches.append(&mut matches_i);
			
			}
			}
		
		}
		println!("matches: {:?}",matches);
		matches
	}	
	fn stable_sync_bs(&self,bp:&Vec<Vec<usize>>,/*posvec:&Vec<Vec<usize>>,*/mask:Vec<usize>,tmp:Vec<usize>,i_pos:usize,bs_:usize,n:usize)->Vec<Vec<usize>>{
		//println!("STABLE SYNC !!!!!!!");
		println!("sync tmp:{:?}",tmp);
		let mut matches:Vec<Vec<usize>>=vec![];
		//let n:usize=bp.len().isqrt();
		
		//if i_pos==n{
		let mut bs:usize=bs_;
		/*
		if bs>n{
			bs=n;
		}
		*/
		//println!("n:{}",n);
		if tmp.len()==n{
			println!("\nEND {:?}\n",tmp);
			//if bp[i_pos]&mask>0{
				matches.push(tmp);
			//}
			return matches;
		}
		else{			
			let mut val_vec:Vec<usize>=vec![];			
			for i in 0..mask.len(){												
				//println!("mask: {}",mask[i]);
				let val:usize=bp[i_pos][i] & mask[i];
				//println!("val: {}",val);
				val_vec.push(val);
			}
			for i in 0..val_vec.len(){
				let val:usize=val_vec[i];				
				//if val_vec[i]>0{
				if val>0{
					//let next:Vec<usize>=get_id_from_num(bp[i_pos],64);
					
					// REWRITE FOR BLOCKSIZE !!!!!!!
					//let next:Vec<usize>=get_id_from_num(val,n*n);
					
					// REWRITTEN! NOW IT'S POSVEC FROM THE PARAMS!!!
					//let next:Vec<usize>=get_id_from_num(val,bs);
					//let next:&Vec<usize>=&posvec[val];
					let next:&Vec<usize>=&self.ordervec_sync[val];
					//println!("next {:?}",next);
					for j in 0..next.len(){
						
						let j_pos:usize=i*bs+next[j];						
						let mut tmp_j:Vec<usize>=tmp.clone();						
						
						//tmp_j.push(next[j]);
						tmp_j.push(j_pos);
						
						//matches.append(&mut stable_sync(&bp,val,tmp_i,i_pos+1,bs));
						//matches.append(&mut stable_sync(&bp,val,tmp_i,next[i],bs));
						//let mut matches_j=stable_sync(&bp,val_vec,tmp_j,next[j],bs);
						let mut matches_j=self.stable_sync_bs(&bp,/*&posvec,*/val_vec.clone(),tmp_j,j_pos,bs,n);
						matches.append(&mut matches_j);					
					}
				}
			}		
		}
		//println!("matches: {:?}",matches);
		matches
	}	
	fn stable_sync_bs_general(&self,bp:&Vec<Vec<usize>>,bs:usize,n:usize)->Vec<Vec<usize>>{
		//println!("self.bitvec:\n{:?}",self.bitvec_sync);
		//println!("self.boolvec:\n{:?}",self.boolvec_sync);
		//println!("self.ordervec:\n{:?}",self.ordervec_sync);
		let mut matches:Vec<Vec<usize>>=vec![];
		//let n:usize=bp.len().isqrt();
		for i in 0..bp.len()-n{			
			let tmp:Vec<usize>=vec![i];
			println!("BS TEST {}",i);
			let mut matches_i=self.stable_sync_bs(&bp,bp[i].clone(),tmp,i,bs,n);
			matches.append(&mut matches_i);		
		}
		println!("N-SIDED MATCHES STABLE SYNC BLOCKSIZE:\n{:?}",matches);
		matches
	}
	fn print_sync_matches(&self,matches:&Vec<Vec<usize>>)->Vec<Vec<[usize;N]>>{
		let mut matchres:Vec<Vec<[usize;N]>>=vec![];
		for i in 0..matches.len(){
			let mut matchres_i:Vec<[usize;N]>=vec![];
			for j in 0..matches[i].len(){
				matchres_i.push(self.all_groups[matches[i][j]]);
			}
			println!("Match {}:\t{:?}",i,matchres_i);
			matchres.push(matchres_i);
		}
		matchres
	}
	fn run_loop(&self,tmp:Vec<[usize;N]>)->Vec<Vec<[usize;N]>>{
		let mut matchres:Vec<Vec<[usize;N]>>=vec![];
		
		matchres
	}
	fn run(&mut self){
		
	}
	
	// 3-sided: 1 side have optimal, the other 2 sides have pessimal solutions(?)
	// lattice structure: 3 times gale-shapley! three points in a row (in preference for each agent)! the outer points are the
	// boundaries!(?) -> proof!
	// this algo does not always lead to successful outcomes. it is sensitive to the proposal order.
	fn galeshapley(&self){
		// pcursor defines the index of the proposing set of agents in pref. the other sets are the ones that have to accept/refuse
		// proposals from the set of pcursor!
		
		let mut pcursor:usize=1;
		let mut unmatched:Vec<usize>=vec![];
		let mut proposals:Vec<usize>=vec![];
		let mut tmp:Vec<Vec<[usize;N]>>=vec![];
		let mut matchres:Vec<Vec<usize>>=vec![];
		let n:usize=self.pref[pcursor].len();
		// initialize all vectors
		for i in 0..n{
			unmatched.push(i);
			proposals.push(0);
		}
		// since this algorithm is sensitive to the proposal order, we have to do some tests with respect to the proposal order!
		
		let mut rng = rand::rng();
		unmatched.shuffle(&mut rng);
		println!("unmatched:{:?}",unmatched);
		
		for i in 0..N{
			let mut tmp_i:Vec<[usize;N]>=vec![];
			for j in 0..n{
				tmp_i.push([n;N]);
			}
			tmp.push(tmp_i);
		}
		
		while unmatched.len()>0{
			let first:usize=unmatched[0];
			// the first holds for self.pref_notranslated
			//let candvec:[usize;N_SIDED-1]=self.pref_notranslated[pcursor][first][proposals[first]].clone();
			// candidates for forming a group
			
			// ERROR!
			println!("first:{}, proposals:{:?}",first,proposals);
			let cand:[usize;N]=self.pref[pcursor][first][proposals[first]].clone();
			let mut acc:bool=true;
			let mut net_gain:i32=0;
			for i in 0..cand.len(){
				if i !=pcursor{
					if tmp[i][cand[i]]==[n;N]{
						
					}
					else{
						let curr_grp:[usize;N]=tmp[i][cand[i]];
						let pos_curr:usize=self.pref[i][cand[i]].iter().position(|&x| x==curr_grp).unwrap();
						let pos_cand:usize=self.pref[i][cand[i]].iter().position(|&x| x==cand).unwrap();
						// without inventing the ideo of net gain!
						
						if pos_curr<pos_cand{
							acc=false;
							break;
						}
						
						
						// imposing the idea of net gain!
						//net_gain-=pos_cand as i32;
						//net_gain+=pos_curr as i32;						
					}
				}
			}
			/*
			if net_gain<0{
				acc=false;
			}
			*/
			if acc{
				let mut to_reject:Vec<[usize;N]>=vec![];
				for i in 0..cand.len(){
					if i !=pcursor{
						if tmp[i][cand[i]]==[n;N]{
							tmp[i][cand[i]]=cand.clone();
						}
						else{
							let curr_grp:[usize;N]=tmp[i][cand[i]];
							to_reject.push(curr_grp);
							tmp[i][cand[i]]=cand;
							/*
							let pos_curr:usize=self.pref[i][cand[i]].iter().position(|&x| x==curr_grp).unwrap();
							let pos_cand:usize=self.pref[i][cand[i]].iter().position(|&x| x==cand).unwrap();
							if pos_curr<pos_cand{
								acc=false;
								break;
							}
							*/
							
						}
					}					
				}
				for i in 0..to_reject.len(){
					for j in 0..N{
						if to_reject[i][j]!=cand[j]{
							let r:usize=to_reject[i][j];
							if j !=pcursor{
									tmp[j][to_reject[i][j]]=[n;N];
								
							}
							else{
								unmatched.push(r);
								proposals[r]+=1;
							}
						}
					}
				}
				unmatched.remove(0);
			}
			else{
				proposals[first]+=1;
			}
		}
		//self.matchres=proposals.clone();
		println!("Gale-Shapley {}-sided matching:\n{:?}",N,proposals);
	}
}

#[derive(Debug)]
struct rank_tree{
	//nodes:Vec<usize>, // JUST FOR TESTS
	//nodes:Vec<rank_tree>,
	depth: usize,
	endvalue:Option<[usize;N]>,
	nodes:Vec<Rc<RefCell<rank_tree>>>,
}
impl rank_tree{
	fn new()->Self{
		Self{
			nodes:vec![],
			depth:0,
			endvalue:None
		}
	}
	// reference values?
	fn add_tree(&mut self,pos:usize,to_add:Rc<RefCell<rank_tree>>){
		if pos>self.nodes.len()-1{
			println!("Parameter 'pos' is too high! Please choose a smaller one!");
		}
		else{
			self.nodes[pos]=to_add;
			self.nodes[pos].borrow_mut().depth=self.depth+1;
		}
	}
	fn push_tree(&mut self,to_add:Rc<RefCell<rank_tree>>){
		let len:usize=self.nodes.len();
		self.nodes.push(to_add);
		self.nodes[len].borrow_mut().depth=self.depth+1;
	}
	fn add_endvalue(&mut self,val:[usize;N]){
		self.endvalue=Some(val);
	}
	fn rank_initialize(&mut self,depth_rem:usize,n:usize){
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
	fn rank_initialize_nvec(&mut self,depth_rem:usize,nvec:&Vec<usize>,cursor:usize){
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
	fn rank_update(&mut self,set:[usize;2],val:usize,cursor:usize,arr:[usize;N-1]){
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
	fn rank_update2(&mut self,set:[usize;2],val:usize,cursor:usize,arr:[usize;N]){
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
	fn retrieve_endvalue(&self,path:&Vec<usize>,cursor:usize)->[usize;N]{
		let endvalue:[usize;N]=[0;N];
		if cursor>path.len()-1{
			return self.endvalue.expect("");
		}
		else{
			return self.nodes[path[cursor]].borrow_mut().retrieve_endvalue(&path,cursor+1);
		}
		//endvalue
	}
	fn retrieve_endvalue_arr(&self,path:&[usize;N],cursor:usize)->[usize;N]{
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

fn test_samples(){
	let pref=samples::create_random_pref_arr(7,0);
	let pref2=samples::create_random_pref_arr(7,1);
	let txt=samples::write_pref_str(&vec![pref,pref2]);
	let prefs=samples::read_pref_str(txt);
	for i in 0..prefs.len(){
	println!("\n\nprefs:\n{:?}",prefs[i]);
	}
	println!("len:{}",prefs[1][2].len());
	let sample=samples::create_sample_set(7,30);
	samples::write_sample(&sample,"samples/sample1.txt".to_string());
	let sample=samples::read_sample_file("samples/sample1.txt".to_string());
	println!("SAMPLE 1:\n{:?}",sample);
}

struct samples{
	sample:Vec<Vec<Vec<Vec<[usize;N]>>>>
}
impl samples{
	fn new()->Self{
		Self{
			sample:vec![]
		}
	}
	fn create_random_pref(n:usize)->Vec<Vec<usize>>{
		let mut pref:Vec<Vec<usize>>=vec![];
		for i in 0..n{
			let mut pref_i:Vec<usize>=vec![];
			for j in 0..n{
				pref_i.push(j);
			}
			let mut rng=rand::rng();
			pref_i.shuffle(&mut rng);
			pref.push(pref_i);
		}
		pref
	}
	// works for 2-sided prefs but not for higher ones. NOT FINISHED!!!!!!!
	fn create_random_pref_arr(n:usize,cursor:usize)->Vec<Vec<[usize;N]>>{
		let mut pref:Vec<Vec<[usize;N]>>=vec![];
		for i in 0..n{
			//let mut pref_i:Vec<[usize;N]>=vec![];
			let mut pref_i:Vec<[usize;N]>=Self::create_all_groups(&[0;N],n,0,cursor,i);
			
			/*
			for j in 0..n{
				let mut arr:[usize;N]=[n;N];
				arr[cursor]=i;
				pref_i.push(arr);
			}
			*/
			let mut rng=rand::rng();
			pref_i.shuffle(&mut rng);
			pref.push(pref_i);
		}
		pref		
	}
	fn create_random_prefs_arr_compl(n:usize)->Vec<Vec<Vec<[usize;N]>>>{
		let mut one_sample:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for i in 0..N{
			one_sample.push(Self::create_random_pref_arr(n,i));
		}
		one_sample
	}
	fn create_sample_set(n:usize,samplesize:usize)->Vec<Vec<Vec<Vec<[usize;N]>>>>{
		let mut sampleset:Vec<Vec<Vec<Vec<[usize;N]>>>>=vec![];
		for i in 0..samplesize{
			sampleset.push(Self::create_random_prefs_arr_compl(n));
		}
		sampleset
	}
	fn create_all_groups(arr:&[usize;N],n:usize,cursor:usize,pcursor:usize,pvalue:usize)->Vec<[usize;N]>{
		let mut groups:Vec<[usize;N]>=vec![];
		if cursor==N{
			return vec![*arr];
		}
		else{
			let mut new_arr:[usize;N]=arr.clone();
			if cursor !=pcursor{
				for i in 0..n{
					new_arr[cursor]=i;
					groups.append(&mut Self::create_all_groups(&new_arr,n,cursor+1,pcursor,pvalue));
				}
			}
			else{
				new_arr[pcursor]=pvalue;
				groups.append(&mut Self::create_all_groups(&new_arr,n,cursor+1,pcursor,pvalue));
			}
		}
		groups
	}
	fn write_pref_str(prefs:&Vec<Vec<Vec<[usize;N]>>>)->String{
		let mut txt:String=String::new();
		for p in 0..prefs.len(){
			let pref:&Vec<Vec<[usize;N]>>=&prefs[p];
			for i in 0..pref.len(){
				for j in 0..pref[i].len(){
					for k in 0..pref[i][j].len(){
						txt+=&pref[i][j][k].to_string();
						if k<pref[i][j].len()-1{
							txt+=",";
						}
					}
					if j<pref[i].len()-1{
						txt+=";";
					}
				}
				if i<pref.len()-1{
					txt+="%";
				}
			}
			if p<prefs.len()-1{
				txt+="@";
			}
		}
		println!("txt:\n{:?}",txt);
		txt
	}
	fn write_sample(sample:&Vec<Vec<Vec<Vec<[usize;N]>>>>,path:String)->String{
		//let mut txt:String=String::new();
		let mut txt:String="".to_string();
		for i in 0..sample.len(){
			txt+=&Self::write_pref_str(&sample[i]);
			txt+="\n";
		}


		let mut fs = File::create(path);
		fs.expect("Problem with the file!").write_all(txt.as_bytes());		
		txt
	}
	fn read_pref_str(txt:String)->Vec<Vec<Vec<[usize;N]>>>{
		let mut prefs:Vec<Vec<Vec<[usize;N]>>>=vec![];
		for word_pref in txt.split('@'){
			let mut pref:Vec<Vec<[usize;N]>>=vec![];
			for word_line in word_pref.split('%'){
				let mut pref_i:Vec<[usize;N]>=vec![];
				for word_group in word_line.split(';'){
					let mut arr:[usize;N]=[0;N];
					let mut cursor:usize=0;
					for word_num in word_group.split(','){
						let num:usize=word_num.parse::<usize>().unwrap();
						arr[cursor]=num;
						cursor+=1;
					}
					pref_i.push(arr);
				}
				pref.push(pref_i);
			}
			prefs.push(pref);
		}
		prefs		
	}
	fn read_sample_file(txtfile:String)->Vec<Vec<Vec<Vec<[usize;N]>>>>{
		let mut sample:Vec<Vec<Vec<Vec<[usize;N]>>>>=vec![];

		let path = Path::new(&txtfile);
		//println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			let prefs:Vec<Vec<Vec<[usize;N]>>>=Self::read_pref_str(line.expect("Line not readable"));
			sample.push(prefs);
		}
		sample
	}
}

/*
fn test_testrun(){
	let mut testres:Vec<Vec<f64>>=vec![];
	testres.push(vec![12.0,4.0,3.0,4.0,4.0,9.0,4.0,2.0,5.0,7.8,8.9]);
	testres.push(vec![3.4,4.5,5.7,5.8,7.0,8.1,9.0,10.1,12.0,14.8,17.7]);
	let margin:[u32;4]=[50,70,70,50];
	//let margin:[u32;4]=[0;4];
	testrun::create_graph_time(&testres,&margin,true,true,"pic/testrun_graph_test.jpg".to_string());
}

struct testrun{
	sample:samples,
	result:Vec<Vec<Vec<[usize;N]>>>,
	time: Vec<Vec<f64>>,
}
impl testrun{
	fn new()->Self{
		Self{
			sample:samples::new(),
			result:vec![],
			time:vec![],
		}
	}
	fn read_timeresult_file(txtfile:String){
		let path = Path::new(&txtfile);
		//println!("path: {:?}", path);
		let file = File::open(&path).expect("file not found");
		let reader = io::BufReader::new(file);
		for line in reader.lines(){
			
		}
	}
	fn get_minmax_f64(vecf64:&Vec<Vec<f64>>)->[f64;2]{
		let mut min:f64=vecf64[0][0];
		let mut max:f64=0.0;
		for i in 0..vecf64.len(){
			for j in 0..vecf64[i].len(){
				if vecf64[i][j]<min{
					min=vecf64[i][j];
				}
				if vecf64[i][j]>max{
					max=vecf64[i][j];
				}				
			}
		}
		[min,max]
	}
	fn create_graph_time(
		time:&Vec<Vec<f64>>,
		margin:&[u32;4],
		minbool:bool,
		markerbool:bool,
		filepath:String
	)->RgbImage{
		//let mut img=image::Rgb((255,255,255).into());
		// margin: [top,bottom,left,right]
		let height:u32=700;
		let width:u32=700;
		let minmax:[f64;2]=Self::get_minmax_f64(&time);
		let mut hspace:f64=minmax[1];
		if minbool{
			hspace=minmax[1]-minmax[0];
		}
		//let h_one:u32=(height as f64/hspace) as u32;
		let h_one:f64=(height-margin[0]-margin[1]) as f64/hspace;
		//let w_one:u32=(width as f64/time[0].len() as f64) as u32;
		let w_one:f64=(width-margin[2]-margin[3]) as f64/(time[0].len()-1)as f64;
		let hspace_u32:u32=(hspace+2.0) as u32;
		//let new_height:u32=h_one*hspace_u32;
		//let new_width:u32=w_one*(time[0].len()-1) as u32+1;
		let mut img:RgbImage=RgbImage::new(width,height);
		img=fill_pic(&img);
		let col1=Rgb([0,255,0]);
		let col2=Rgb([0,255,255]);
		let mut col:Vec<Rgb<u8>>=vec![col1,col2];
		//println!("new_width:{}, new_height:{}",new_width,new_height);
		println!("minmax:{:?}",minmax);
		
		// FONT RELATED CONFIGURATION !!!!!!!
		//let font=FontRef::try_from_slice(include_bytes!("../assets/arial.ttf")).unwrap();
		let font=FontRef::try_from_slice(include_bytes!("../assets/arial.ttf")).unwrap();
		//let font_bl=FontRef::try_from_slice(include_bytes!("../assets/arial_bold.ttf")).unwrap();
		let font_bl=FontRef::try_from_slice(include_bytes!("../assets/arial_bold.ttf")).unwrap();
		
		// for infile fonts!
		//let font_bytes:&[u8]=&[0,0,1,0,1,0,0,1,......,1,0,1];
		//let font_bl_bytes:&[u8]=&[0,0,1,0,1,0,0,1,......,1,0,1];
		//let font=FontRef::try_from_slice(font_bytes).unwrap();
		//let font_bl=FontRef::try_from_slice(font_bl_bytes).unwrap();
		
		let mut text_height=18.0;
		let mut scale=PxScale{x:18.0,y:18.0};
		// END FONT
		
		if time.len()>0{
			for i in 0..time.len(){
				//let mut x0:u32=0;
				let mut x0:u32=margin[2];
				println!("time[i][0]:{}",time[i][0]);
				let mut y0:u32=height-margin[1]-((time[i][0]-minmax[0])*h_one) as u32;
				if !minbool{
					y0=height-margin[1]-(h_one*time[i][0]) as u32;
				}
				//for j in 0..hspace_u32/h_one{
				for j in 1..time[i].len(){
					println!("i:{}, j:{}",i,j);
					//let mut x1:u32=(j as f64*w_one) as u32;
					let mut x1:u32=(j as f64*w_one) as u32+margin[2];
					if x1==width{
						x1-=1;
					}
					println!("height:{}, time[{}][{}]:{:?}, h_one:{}, time[{}][{}]*h_one:{}",height,i,j,time[i][j],h_one,i,j,(time[i][j]-minmax[0])*h_one);
					let mut	y1:u32=height-margin[1]-((time[i][j]-minmax[0])*h_one) as u32;
					if !minbool{
						y1=height-margin[1]-(time[i][j]*h_one) as u32;
					}
					println!("x0:{}, y0:{}; x1:{}, y1:{}",x0,y0,x1,y1);
					draw_line3((x0,y0),(x1,y1),col[i],&mut img);
					x0=x1;
					y0=y1;
				}
			}	
		}
		if markerbool{
			let mut x0:u32=margin[2];
			let mut x1:u32=x0;
			let mut x2:u32=width-margin[3];
			let mut y0:u32=margin[0];
			let mut y1:u32=height-margin[1];
			let mut y2:u32=y1;
			println!("x0:{}, y0:{}; x1:{}, y1:{}",x0,y0,x1,y1);
			draw_line3((x0,y0),(x1,y1),Rgb([0,0,0]),&mut img);
			draw_line3((x1,y1),(x2,y2),Rgb([0,0,0]),&mut img);
			let markerwidth:u32=20;
			let mcol:Rgb<u8>=Rgb([0,0,0]);
			let y_intervall:f64=2.0;
			let x_intervall:usize=3;
			let y_lines:usize=(hspace/y_intervall) as usize;
			for i in 0..y_lines+1{
				let x0:u32=margin[2]-1;
				let mut x1:u32=0;
				if markerwidth<x0{
					x1=x0-markerwidth;
				}
				let yi:u32=height-margin[1]-(i as f64*h_one*y_intervall) as u32;
				
				// ADDING FONT TO MARKER!!!!!!!
				println!("yi:{}",yi);
				draw_line3((x0,yi),(x1,yi),mcol,&mut img);
				// TEMPLATE!
				/*
				let mut scale_bl=PxScale{
					x:pw as f32 - margin as f32,
					y:pw as f32 - margin as f32
				};			
				//draw_text_mut(&mut img, Rgb([255u8,255u8,255u8]), (valx) as i32, (valy) as i32, scale_bl,&font_bl, &match_[j].to_string());
				draw_text_mut(&mut img, Rgb([255u8,255u8,255u8]), (valx) as i32, (valy) as i32, scale_bl,&font_bl, &match_[j].to_string());
				*/
				//let scale=PxScale{x:x0,y:yi};
				let mut x_txt:i32=(x1-20) as i32;
				if i as f64*y_intervall<10.0{
					x_txt=x1 as i32-11;
				}	
				//draw_text_mut(&mut img, Rgb([0u8,0u8,0u8]),x1 as i32-20,yi as i32-9,scale,&font_bl,&(i as f64*y_intervall).to_string());
				//draw_text_mut(&mut img, Rgb([0u8,0u8,0u8]),x_txt-20,yi as i32-9,scale,&font_bl,&(i as f64*y_intervall).to_string());
				draw_text_mut(&mut img, Rgb([0u8,0u8,0u8]),x_txt-20,yi as i32-9,scale,&font,&format!("{:?}",i as f64*y_intervall));
				println!("test output:{:?}",&(format!("{:3}",12.4)).to_string());
			}
			let x_lines:usize=time[0].len()/x_intervall;
			for i in 0..x_lines+1{
				let xi:u32=margin[2]+(((i*x_intervall) as f64)*w_one) as u32;
				let y0:u32=height-margin[1];
				let mut y1:u32=height-1;
				if markerwidth<margin[1]{
					y1=y0+markerwidth;
				}
				if xi<width{
					draw_line3((xi,y0),(xi,y1),mcol,&mut img);
				}
			}



			
		}
		
		img.save(filepath);
		img
	}
}
*/
fn bp_matrix(adj:&Vec<Vec<[usize;2]>>)->Vec<Vec<bool>>{
	let mut mat:Vec<Vec<bool>>=vec![];
	let n:usize=adj.len();
	//for i in 0..n*n{
	for i in 0..n{		
		println!("bp_matrix i:{}, len:{}",i,adj.len());
		//for j in 0..i{
		for j in 0..n{
			let mut row:Vec<bool>=vec![];
			//let mut idx:usize=i*n+j;
			for k in 0..n{
				for m in 0..n{
					/*
					let mut idx_2:usize=k*n+m;
					if idx_2<idx{
						
					}
					*/
					// BE VERY CAREFUL!!!!!!!
					//if i!=k && j!=m{
					
					//if !(i==k && j==m){
					if !(i==k || j==m){
						if !bp_efficient(&adj,&[i,j],&[k,m],0){
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
fn bp_efficient(adj:&Vec<Vec<[usize;2]>>,a:&[usize;2],b:&[usize;2],diff:usize)->bool{
//fn bp_efficient(adj:&Vec<Vec<[usize;2]>>,a:&[usize;2],b:&[usize;2],diff:usize) -> bool {
	if adj[a[0]][b[1]][0]+diff<adj[a[0]][a[1]][0]{
		if adj[a[0]][b[1]][1]+diff<adj[b[0]][b[1]][1]{
			return true;
		}
	}
	// "ELSE IF" IS NOT WORKING CORRECTLY!!!!!!!
	//else if adj[b[0]][a[1]][0]+diff<adj[b[0]][b[1]][0]{
	if adj[b[0]][a[1]][0]+diff<adj[b[0]][b[1]][0]{
		if adj[b[0]][a[1]][1]+diff<adj[a[0]][a[1]][1]{
			return true;
		}
	}	
	return false;
}
fn get_id_from_num(num:usize,bs:usize)->Vec<usize>{
	let mut new:Vec<usize>=vec![];
	//let num:usize=31;
	//let raw=BitVec::from_element(7_usize);
	let raw = num.view_bits::<Lsb0>();
	//println!("RAW {:?}",raw);
	let len:usize=raw.len();
	
	//println!("BIT VEC: {:?}",bp_matrix_2usize(&bp_matrix));
	//println!("BIT VEC BLOCK: {:?}",bp_matrix_2usize_block(&bp_matrix,5));
	
	
	//println!("TEST LARGE NUMBERS BIT: {}", 340282366920938463463374600121768211455 as u128 & 20000000000000000000000 as u128);
	
	//340282366920938463463374607431768211455
	
	
	
	//let bits=raw.iter_ones().position(|x| x==1).unwrap();
	let bits=raw.iter_ones();//.filter(|x| *x==1);
	//println!("BITS ITER {:?}",bits);
	
	//println!("TEST BIT ITER: {}",raw.len());
	//println!("TEST RAW ITER ONES: {:?}",raw.iter_ones().rev());
	for bit in raw.iter_ones().rev(){
		//print
		//println!("bit: {}, bs: {} ",bit,bs);
		//new.push(len-bit);
		
		new.push(bs-bit-1);
	}
	//println!();
	//println!("GET ID NEW: {:?}",new);
	new
}
fn bp_matrix_half(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let mut new:Vec<Vec<bool>>=bp.clone();
	for i in 0..new.len(){
		for j in 0..i{
			new[i][j]=false;
		}
	}
	new
}
fn stable_sync(bp:&Vec<usize>,mask:usize,tmp:Vec<usize>,i_pos:usize,bs:usize)->Vec<Vec<usize>>{
	//println!("STABLE SYNC !!!!!!!");
	let mut matches:Vec<Vec<usize>>=vec![];
	let n:usize=bp.len().isqrt();
	//if i_pos==n{
	if tmp.len()==n{
		println!("\nEND {:?}\n",tmp);
		//if bp[i_pos]&mask>0{
			matches.push(tmp);
		//}
		return matches;
	}
	else{
		println!("mask: {}",mask);
		let val:usize=bp[i_pos] & mask;
		println!("val: {}",val);
		if val>0{
		//let next:Vec<usize>=get_id_from_num(bp[i_pos],64);
		let next:Vec<usize>=get_id_from_num(val,n*n);
		println!("next {:?}",next);
		for i in 0..next.len(){
			
						
			let mut tmp_i:Vec<usize>=tmp.clone();
			
			
			tmp_i.push(next[i]);
			
			//matches.append(&mut stable_sync(&bp,val,tmp_i,i_pos+1,bs));
			//matches.append(&mut stable_sync(&bp,val,tmp_i,next[i],bs));
			let mut matches_i=stable_sync(&bp,val,tmp_i,next[i],bs);
			matches.append(&mut matches_i);
		
		}
		}
	
	}
	println!("matches: {:?}",matches);
	matches
}
fn position_N(chain:&Vec<[usize;N]>,b:&[usize;N])->usize{
	let mut pos:usize=0;
	for i in 0..chain.len(){
		if chain[i]==*b{
			pos=i;
			break;
		}
	}
	pos
}
fn vec_usize_2_f64(vec:&Vec<usize>)->Vec<f64>{
	let mut vecf64:Vec<f64>=vec![];
	for i in 0..vec.len(){
		vecf64.push(vec[i] as f64);
	}
	vecf64
}
fn vec_i8_2_f64(vec:&Vec<i8>)->Vec<f64>{
	let mut vecf64:Vec<f64>=vec![];
	for i in 0..vec.len(){
		vecf64.push(vec[i] as f64);
	}
	vecf64
}

fn mat_i8_2_f64(mat:&Vec<Vec<i8>>)->Vec<Vec<f64>>{
	let mut matf64:Vec<Vec<f64>>=vec![];
	for i in 0..mat.len(){
		let mut vecf64:Vec<f64>=vec![];
		for j in 0..mat[i].len(){
			vecf64.push(mat[i][j] as f64);
		}
		matf64.push(vecf64);
	}
	matf64
}
fn vec_switch_2_negativity(vec:&Vec<f64>)->Vec<f64>{
	let mut vec_neg:Vec<f64>=vec![];
	for i in 0..vec.len(){
		vec_neg.push((-1.0)*vec[i]);
	}
	vec_neg
}
fn vec_i8_switch_2_negativity(vec:&Vec<i8>)->Vec<i8>{
	let mut vec_neg:Vec<i8>=vec![];
	for i in 0..vec.len(){
		vec_neg.push((-1)*vec[i]);
	}
	vec_neg
}

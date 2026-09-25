#![allow(warnings)]
use super::helper::{rank_tree,IOClass};
use rand::seq::SliceRandom;
use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::{
	fs::{self, File},
	io::{self, BufRead},
	//path::Path,
};
use bitvec::prelude::BitVec;
use bitvec::prelude::*;
//use bitvector::*;

pub const N:usize=2;


pub fn test(){
	println!("BITSYNC TEST");
	println!("BITSYNC 2S");
	let mut bs2s:stablesync_2s=stablesync_2s::new();
	bs2s.init();
	//bs2s.run();
	let bp=IOClass::get_bp_automatically();
	let bp_half=IOClass::bp_matrix_half(&bp);
	println!("BP HALF:{:?}",bp_half);
	//let bp_usize=bitsync::bp_matrix_2usize_block2(&bp_half,16);
	//let mut bitsnc=bitsync::stable_sync_bs_general(&bp_usize,16);
	
	//let bp_usize=bitsync::bp_matrix_2usize(&bp_half);
	//let mut bitsnc=bitsync::stable_sync_general(&bp_usize,16);
	
	// LATTICE
	//let bp_usize=bitsync::bp_matrix_2usize(&bp_half);
	//let mut bitsnc=bitsync::stable_sync_lattice_general(&bp_usize,16);

	// LATTICE BS
	let bp_usize=bitsync::bp_matrix_2usize_block2(&bp_half,16);
	let mut bitsnc=bitsync::stable_sync_bs_lattice_general(&bp_usize,16);
}

// please insert adj function into struct!!!!!!!
struct stablesync_2s{
	//pref:Vec<Vec<Vec<[usize;N]>>>,
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
}
impl stablesync_2s{
	fn new()->Self{
		Self{
			//pref:vec![],
			m:vec![],
			w:vec![],
		}
	}
	fn init(&mut self){
		self.m=IOClass::read_txt("pref/m.txt".to_string());
		self.w=IOClass::read_txt("pref/w.txt".to_string());
	}
	fn create_adj(&self)->Vec<Vec<[usize;2]>>{
		let mut adj:Vec<Vec<[usize;2]>>=vec![];
		
		adj
	}
	fn get_id_from_num(num:usize,bs:usize)->Vec<usize>{
		let mut new:Vec<usize>=vec![];
		let raw = num.view_bits::<Lsb0>();
		let len:usize=raw.len();
		let bits=raw.iter_ones();//.filter(|x| *x==1);
		for bit in raw.iter_ones().rev(){
			new.push(bs-bit-1);
		}
		new
	}
	fn create_stabm_bool(&self,adj:&Vec<Vec<[usize;2]>>,n_vec:&[usize;2])->(Vec<Vec<bool>>,Vec<usize>){
		let mut del_vec:Vec<usize>=vec![];
		let mut stabm:Vec<Vec<bool>>=vec![];
		let n:usize=adj.len();
		let mut dummy_row:Vec<bool>=vec![];
		for i in 0..n*n_vec[1]{
			dummy_row.push(false);
		}
		for i in 0..n{
			for j in 0..n_vec[1]{
				let mut stabm_ij:Vec<bool>=vec![];
				let mut k_bool:bool=true;
				for k in 0..n{
					if k != i{
						let mut m_bool:bool=false;
						for m in 0..n_vec[1]{
							if m != j{
								if !Self::bp_efficient2(&adj,&[i,j],&[k,m],0){
									stabm_ij.push(true);
									m_bool=true;
								}
								else{
									stabm_ij.push(false);
								}
							}
							else{
								stabm_ij.push(false)
							}
						}
						if !m_bool{
							k_bool=false;
							del_vec.push(i*n+j);
							stabm_ij=dummy_row.clone();
							break;
						}
					}
					else{
						for m in 0..n_vec[1]{
							stabm_ij.push(false);
						}
					}
					if !k_bool{
						stabm_ij=dummy_row.clone();
						del_vec.push(i*n+j);
						break;
					}					
				}
				println!("k_bool:{:?}, stabm_ij:{:?}",k_bool,stabm_ij);
				stabm.push(stabm_ij);
			}
		}
		println!("STABM SYNC_2S:");
		for i in 0..stabm.len(){
			println!("{:?}",stabm[i]);
		}
		println!("del_vec:{:?}\ndel_vec.len():{}",del_vec,del_vec.len());
		(stabm,del_vec)
	}
	fn shrink_stabm(&self,stabm:&Vec<Vec<bool>>,del_vec:&Vec<usize>)->Vec<Vec<bool>>{
		let mut shrink:Vec<Vec<bool>>=vec![];
		for i in 0..stabm.len(){
			if !del_vec.contains(&i){
				let mut shrink_i:Vec<bool>=vec![];
				for j in 0..stabm[i].len(){
					if !del_vec.contains(&j){
						shrink_i.push(stabm[i][j]);
					}
				}
				shrink.push(shrink_i);
			}
		}
		shrink
	}
	fn create_bitvec_bool(bitlen:usize,tmp:&Vec<bool>)->Vec<Vec<bool>>{
		let mut bitvec:Vec<Vec<bool>>=vec![];
		
		bitvec
	}
	fn create_bp_vec_usize(bp:&Vec<Vec<usize>>,bitlen:usize,n_vec:&Vec<usize>)->Vec<Vec<Vec<Vec<usize>>>>{
		let n:usize=n_vec[0];
		let mut stabm:Vec<Vec<Vec<Vec<usize>>>>=vec![];
		for i in 0..bp.len(){
			let mut stabm_i:Vec<Vec<Vec<usize>>>=vec![];
			for j in 0..n{
				let mut stabm_ij:Vec<Vec<usize>>=vec![];
				
				stabm_i.push(stabm_ij);
			}
			stabm.push(stabm_i);
		}
		stabm
	}		
	fn run_bs(&self,bp:&Vec<Vec<usize>>,mask:Vec<usize>,tmp:Vec<usize>,i_pos:usize,bs_:usize,n:usize)->Vec<Vec<usize>>{
		//println!("STABLE SYNC !!!!!!!");
		let mut matches:Vec<Vec<usize>>=vec![];
		//let n:usize=bp.len().isqrt();
		//if i_pos==n{
		let mut bs:usize=bs_;
		/*
		if bs>n{
			bs=n;
		}
		*/
		
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
				let val:usize=bp[i_pos][i] & mask[i];
				val_vec.push(val);
			}
			for i in 0..val_vec.len(){
				let val:usize=val_vec[i];								
				if val>0{
					let next:Vec<usize>=Self::get_id_from_num(val,bs);					
					for j in 0..next.len(){						
						let j_pos:usize=i*bs+next[j];						
						let mut tmp_j:Vec<usize>=tmp.clone();																		
						tmp_j.push(j_pos);						
						let mut matches_j=self.run_bs(&bp,val_vec.clone(),tmp_j,j_pos,bs,n);
						matches.append(&mut matches_j);					
					}
				}
			}		
		}
		//println!("matches: {:?}",matches);
		matches
	}
	fn bp_efficient2(adj:&Vec<Vec<[usize;2]>>,a:&[usize;2],b:&[usize;2],diff:usize)->bool{
		if adj[a[0]][b[1]][0]+diff<adj[a[0]][a[1]][0]{
			println!("1. if");
			if adj[a[0]][b[1]][1]+diff<adj[b[0]][b[1]][1]{
				return true;
			}
		}
		if adj[b[0]][a[1]][0]+diff<adj[b[0]][b[1]][0]{
			println!("2. if");
			if adj[b[0]][a[1]][1]+diff<adj[a[0]][a[1]][1]{
				return true;
			}
		}
		println!("else");
		return false;
	}
	fn create_del_vec_mapper(del_vec:&Vec<usize>,n:usize)->Vec<usize>{
		let mut res:Vec<usize>=vec![];
		for i in 0..n{
			if !del_vec.contains(&i){
				res.push(i);
			}
		}
		res
	}
	fn transform_shrinked_matches(&self,matches:&Vec<Vec<usize>>,del_vec:&Vec<usize>,n:usize)->Vec<Vec<usize>>{
		let mut transform:Vec<Vec<usize>>=vec![];
		let mapper:Vec<usize>=Self::create_del_vec_mapper(&del_vec,n);
		for i in 0..matches.len(){
			let mut transform_i:Vec<usize>=vec![];
			for j in 0..matches[i].len(){
				transform_i.push(mapper[matches[i][j]]);
			}
			transform.push(transform_i);
		}
		transform
	}
	fn decompose(&self,val:usize)->[usize;2]{
		let n:usize=self.m.len();
		let mut rslt:[usize;2]=[n;2];
		let i:usize=val/n;
		let j:usize=val-i*n;
		rslt=[i,j];
		rslt
	}
	fn show_matches(&self,matches:&Vec<Vec<usize>>)->Vec<Vec<[usize;2]>>{
		let mut rslt:Vec<Vec<[usize;2]>>=vec![];
		for i in 0..matches.len(){
			let mut vec_i:Vec<[usize;2]>=vec![];
			for j in 0..matches[i].len(){
				vec_i.push(self.decompose(matches[i][j]));
			}
			println!("MATCH: {:?}",vec_i);
			rslt.push(vec_i);
		}
		rslt
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
		dbg!(&rank);
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
pub struct bitsync{
	
}
impl bitsync{

	pub fn stable_sync(bp:&Vec<usize>,mask:usize,tmp:Vec<usize>,i_pos:usize,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		if tmp.len()==n{
			println!("\nEND {:?}\n",tmp);
			matches.push(tmp);
			return matches;
		}
		else{
			println!("mask: {}",mask);
			let val:usize=bp[i_pos] & mask;
			println!("val: {}",val);
			if val>0{
			let next:Vec<usize>=Self::get_id_from_num(val,n*n);
			println!("next {:?}",next);
			for i in 0..next.len(){				
				let mut tmp_i:Vec<usize>=tmp.clone();								
				tmp_i.push(next[i]);
				let mut matches_i=Self::stable_sync(&bp,val,tmp_i,next[i],bs);
				matches.append(&mut matches_i);			
			}
			}		
		}
		println!("matches: {:?}",matches);
		matches
	}
	pub fn stable_sync_lattice(bp:&Vec<usize>,mask:usize,tmp:Vec<usize>,i_pos:usize,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		let (m_idx,w_idx)=Self::decompose_ij(i_pos,n);
		if tmp.len()==n{
			println!("\nEND {:?}\n",tmp);
			matches.push(tmp);
			return matches;
		}
		else{
			println!("mask: {}",mask);
			let val:usize=bp[i_pos] & mask;
			println!("val: {}",val);
			if val>0{
			let next:Vec<usize>=Self::get_id_from_num(val,n*n);
			println!("next {:?}",next);
			for i in 0..next.len(){				
				let (m_idx_i,w_idx_j)=Self::decompose_ij(next[i],n);
				if m_idx_i <= m_idx+1{
					let mut tmp_i:Vec<usize>=tmp.clone();										
					tmp_i.push(next[i]);
					let mut matches_i=Self::stable_sync(&bp,val,tmp_i,next[i],bs);
					matches.append(&mut matches_i);			
				}
			}
			}		
		}
		println!("matches: {:?}",matches);
		matches
	}
	pub fn stable_sync_bs_lattice(bp:&Vec<Vec<usize>>,mask:Vec<usize>,tmp:Vec<usize>,i_pos:usize,bs_:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		let mut bs:usize=bs_;
		let (m_idx,w_idx)=Self::decompose_ij(i_pos,n);
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
					// REWRITE FOR BLOCKSIZE !!!!!!!
					let next:Vec<usize>=Self::get_id_from_num(val,bs);
					for j in 0..next.len(){						
						let j_pos:usize=i*bs+next[j];						
						let (m_idx_i,w_idx_j)=Self::decompose_ij(j_pos,n);
						if m_idx_i <= m_idx+1{
							let mut tmp_j:Vec<usize>=tmp.clone();
							tmp_j.push(j_pos);	
							let mut matches_j=Self::stable_sync_bs(&bp,val_vec.clone(),tmp_j,j_pos,bs);
							matches.append(&mut matches_j);
						}
					}
				}
			}
		
		}
		matches
	}
	pub fn decompose_ij(len:usize,n:usize)->(usize,usize){
		let mut i:usize=len/n;
		let mut j:usize=len-i*n;
		println!("len: {}, n: {}, i: {}, j: {}",len,n,i,j);
		(i,j)
	}
	pub fn stable_sync_bs(bp:&Vec<Vec<usize>>,mask:Vec<usize>,tmp:Vec<usize>,i_pos:usize,bs_:usize)->Vec<Vec<usize>>{
		//println!("STABLE SYNC !!!!!!!");
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
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
					// REWRITE FOR BLOCKSIZE !!!!!!!
					let next:Vec<usize>=Self::get_id_from_num(val,bs);
					for j in 0..next.len(){						
						let j_pos:usize=i*bs+next[j];						
						let mut tmp_j:Vec<usize>=tmp.clone();
						tmp_j.push(j_pos);	
						let mut matches_j=Self::stable_sync_bs(&bp,val_vec.clone(),tmp_j,j_pos,bs);
						matches.append(&mut matches_j);					
					}
				}
			}
		}
		//println!("matches: {:?}",matches);
		matches
	}
	pub fn stable_sync_lattice_general(bp:&Vec<usize>,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		for i in 0..bp.len()-n{
			let tmp:Vec<usize>=vec![i];
			let mut matches_i=Self::stable_sync_lattice(&bp,bp[i],tmp,i,bs);
			matches.append(&mut matches_i);		
		}
		println!("MATCHES STABLE SYNC: {:?}",matches);
		matches
	}
	pub fn stable_sync_bs_lattice_general(bp:&Vec<Vec<usize>>,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		for i in 0..n{			
			let tmp:Vec<usize>=vec![i];
			println!("BS TEST {}",i);
			let mut matches_i=Self::stable_sync_bs_lattice(&bp,bp[i].clone(),tmp,i,bs);
			matches.append(&mut matches_i);		
		}
		//println!("MATCHES STABLE SYNC BLOCKSIZE: {:?}",matches);
		let dec_matches=Self::decompose_matches(&matches,n);
		Self::print_matches_decomposed(&dec_matches);
		matches
	}
	pub fn stable_sync_general(bp:&Vec<usize>,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		for i in 0..bp.len()-n{
			let tmp:Vec<usize>=vec![i];
			let mut matches_i=Self::stable_sync(&bp,bp[i],tmp,i,bs);
			matches.append(&mut matches_i);		
		}
		println!("MATCHES STABLE SYNC: {:?}",matches);
		matches
	}
	pub fn stable_sync_bs_general(bp:&Vec<Vec<usize>>,bs:usize)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		let n:usize=bp.len().isqrt();
		for i in 0..bp.len()-n{
			
			let tmp:Vec<usize>=vec![i];
			println!("BS TEST {}",i);
			let mut matches_i=Self::stable_sync_bs(&bp,bp[i].clone(),tmp,i,bs);
			matches.append(&mut matches_i);		
		}
		//println!("MATCHES STABLE SYNC BLOCKSIZE: {:?}",matches);
		let dec_matches=Self::decompose_matches(&matches,n);
		Self::print_matches_decomposed(&dec_matches);
		matches
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
	pub fn stable_sync_matches_ord(matches:&Vec<Vec<usize>>,resolve:Vec<usize>)->Vec<Vec<usize>>{
		let mut new:Vec<Vec<usize>>=vec![];
		let mut n:usize=0;
		if matches.len()>0{
			n=matches[0].len();
		}
		for i in 0..matches.len(){
			let mut new_i:Vec<usize>=vec![];
			for j in 0..matches[i].len(){
				new_i.push(n);
			}			
			for j in 0..matches[i].len(){
				let mut val:usize=resolve[matches[i][j]];
				let (i_pos,j_pos)=Self::decompose_ij(val,n);
				println!("val: {}, i_pos: {}, j_pos: {}",val,i_pos,j_pos);
				new_i[i_pos]=j_pos;
			}
			new.push(new_i);
		}
		println!("SYNC MATCHES ORDERED\n:{:?}",new);
		new
	}
	pub fn get_id_from_num(num:usize,bs:usize)->Vec<usize>{
		let mut new:Vec<usize>=vec![];
		//let num:usize=31;
		//let raw=BitVec::from_element(7_usize);
		let raw = num.view_bits::<Lsb0>();
		//println!("RAW {:?}",raw);
		let len:usize=raw.len();
		let bits=raw.iter_ones();//.filter(|x| *x==1);
		for bit in raw.iter_ones().rev(){
			new.push(bs-bit-1);
		}
		new
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
				if bp[i][j]{					
					// CHANGED i32 TO u32
					val+=(2_u32.pow((bs+i_b*bs-1-j) as u32)) as usize;
				}
				if j==bs-1 && i_b==0{
					i_b+=1;
					bits_i.push(val as usize);
					val=0;
				}
				else{
					if j<bs{
						continue;
					}
					// BE VERY VERY VERY VEY CAREFUL LATER !!!!!!!!!!!!
					if j==(i_b+1)*bs-1 || j==n-1{
						//println!("increase");
						i_b+=1;
						bits_i.push(val as usize);
						val=0;
					}
				}
			}
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
				if bp[i][j]{
					// CHANGED i32 TO u32
					// CHANGED FROM I32 TO U32 TO USIZE TO U128 !!!!!!! WORKS !!!!!!!
					val+=(2_u128.pow((bs+i_b*bs-1-j) as u32)) as usize;
				}				
				if j==bs-1 && i_b==0{
					i_b+=1;
					bits_i.push(val as usize);
					val=0;
				}
				else{					
					if j<bs{
						continue;
					}
					// BE VERY VERY VERY VEY CAREFUL LATER !!!!!!!!!!!!
					if (j==(i_b+1)*bs-1) || (j==n-1){
						i_b+=1;
						bits_i.push(val as usize);
						val=0;
					}
				}
			}
			bits.push(bits_i);
		}
		bits
	}		
}

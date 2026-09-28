#![allow(warnings)]
use super::helper::IOClass;
use image::{Rgb,RgbImage};
use std::cell::RefCell;
use std::rc::Rc;
use std::{
	fs::{self,File},
	io::{self,BufRead}
};
use std::path::Path;

const N:usize=2;
pub fn test_manyone_enumeration(){
	println!("#### ENUMERATION ####");
	let mut gf_mm:gusfield_manymany=gusfield_manymany::init();
	//gf_mm.update();
	gf_mm.show_lists();
	//gf_mm.enumeration();
	gf_mm.manyone_wrapper();
}

fn test_gusfield_manymany(){
	let mut gf_mo:gusfield_manymany=gusfield_manymany::init();
	let (is_ok,scxlen,stxlen)=gf_mo.check_list_consistency();
	gf_mo.show_lists();
	println!("consistent: {}, sc_xlen:{}, st_xlen:{}",is_ok,scxlen,stxlen);
	gf_mo.complete_lists_123();
	gf_mo.show_lists();
	let sc:Vec<Vec<usize>>=u16_2_usize(&gf_mo.sc);//.clone();
	let st:Vec<Vec<usize>>=u16_2_usize(&gf_mo.st);//.clone();
	let qu_sc:Vec<usize>=gf_mo.qu_sc.clone();
	let qu_st:Vec<usize>=gf_mo.qu_st.clone();
	let mtmp=gf_mo.create_full_mtmp();
	//let wtmp=gf_mo.create_empty_wtmp();
	let wtmp:Vec<Vec<usize>>=vec![vec![];st.len()];
	//gusfield_manyone::get_posets2(&sc,&st,&vec![],&vec![],&vec![0;st.len()],0);
	// student proposing works! take care of the files! swap if necessary!
	//gusfield_manymany::get_posets(&sc,&st,&qu_sc,&qu_st,&wtmp,&mtmp,&vec![vec![0];sc.len()],0);
	
	// iterate proposals until capacities of proposers are full!
	gusfield_manymany::get_posets_iter2(&sc,&st,&qu_sc,&qu_st,&wtmp,&mtmp,&vec![vec![0];sc.len()]);
}

fn test_gusfield_manyone(){
	let mut gf_mo:gusfield_manyone=gusfield_manyone::init();
	let (is_ok,scxlen,stxlen)=gf_mo.check_list_consistency();
	gf_mo.show_lists();
	println!("consistent: {}, sc_xlen:{}, st_xlen:{}",is_ok,scxlen,stxlen);
	gf_mo.complete_lists();
	gf_mo.show_lists();
	let sc:Vec<Vec<usize>>=u16_2_usize(&gf_mo.sc);//.clone();
	let st:Vec<Vec<usize>>=u16_2_usize(&gf_mo.st);//.clone();
	let qu_sc:Vec<usize>=gf_mo.qu_sc.clone();
	let qu_st:Vec<usize>=gf_mo.qu_st.clone();
	let mtmp=gf_mo.create_full_mtmp();
	let wtmp=gf_mo.create_empty_wtmp();
	let idx:Vec<Vec<usize>>=vec![vec![0];sc.len()];
	//gusfield_manyone::get_posets2(&sc,&st,&vec![],&vec![],&vec![0;st.len()],0);
	gusfield_manyone::get_posets2(&sc,&st,&qu_sc,&qu_st,&wtmp,&mtmp,&vec![0;st.len()],0);
	//gusfield_manyone::get_posets2(&sc,&st,&qu_sc,&qu_st,&wtmp,&mtmp,&vec![vec![0];sc.len()],0);
	//gusfield_manyone::get_posets2(&sc,&st,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,0);
}

fn test_gusfield(){
	let mut gf:gusfield=gusfield::new();
	gf.init();
	let match1:Vec<usize>=vec![7,2,4,5,6,0,1,3];
	let (m,w)=gf.reduce_list(&match1);
	println!("mpref\n{:?}",gf.m);
	println!("wpref\n{:?}",gf.w);
	println!("m: {:?}",m);
	println!("w: {:?}",w);
	println!("consistency?: {}",
		gusfield::check_consistency(
			&gusfield::u16_2_usize(&m),
			&gusfield::u16_2_usize(&w)
		)
	);
	let musize:Vec<Vec<usize>>=gusfield::u16_2_usize(&m);
	let wusize:Vec<Vec<usize>>=gusfield::u16_2_usize(&w);
	let mut idx:Vec<usize>=gf.create_zero_idx();
	// idx[7]=1 not working because idx[7]=0 is already the women-optimal stable matching, 
	// a worse woman is not possible!
	idx[0]=1; 
	let mut wtmp:Vec<Option<usize>>=gf.create_empty_wtmp();
	let mut mtmp:Vec<usize>=gf.create_full_mtmp();
	let rmatch:Vec<usize>=gusfield::get_posets(&musize,&wusize,&wtmp,&mtmp,&idx,0);
	println!("rmatch:{:?}",rmatch);
	let gsm:Vec<usize>=gf.gsm.clone();
	let gsw:Vec<usize>=gf.gsw.clone();
	println!("### GALE-SHAPLEY MEN-OPT: {:?} ###",gsm);
	println!("############ RUN ############");
	gf.next_level(&vec![gsm]);
	//gf.next_level(&vec![]);
	gf.print_matches();
	gf.print_edges();
}



// BIG TEST! IT WORKS PERFECT !!!!!!!
struct gusfield_manymany{
	sc:Vec<Vec<u16>>,
	st:Vec<Vec<u16>>,
	qu_sc:Vec<usize>,
	qu_st:Vec<usize>,
}
impl gusfield_manymany{
	fn init()->Self{
		Self{
			sc:read_txt("manyone_pref/schools.txt".to_string()),
			st:read_txt("manyone_pref/students.txt".to_string()),
			qu_sc:read_txt_usize("manyone_pref/quota_schools.txt".to_string())[0].clone(),
			qu_st:read_txt_usize("manyone_pref/quota_students.txt".to_string())[0].clone(), // later for many-to-many matchings!!!!!!!
		}
	}
	fn update(&mut self){
		//self.complete_lists_123();
		self.complete_lists_123_add1();
	}
	fn manyone_wrapper(&mut self){
		self.update();
		let sc:Vec<Vec<usize>>=u16_2_usize(&self.sc);
		let qu_sc:Vec<usize>=self.qu_sc.clone();
		let st:Vec<Vec<usize>>=u16_2_usize(&self.st);
		
		let qu_st:Vec<usize>=self.qu_st.clone();
		
		let adj:Vec<Vec<[usize;2]>>=Self::create_adj_school(&st,&sc);
		println!("SCHOOLS");
		for i in 0..sc.len(){
			println!("i:{}: {:?}",i,sc[i]);
		}
		println!("STUDENTS");
		for i in 0..st.len(){
			println!("i:{}: {:?}",i,st[i]);
		}
		//display_adj(&adj);
		let bp:Vec<Vec<bool>>=Self::bp_matrix_school(&adj);
		//display_bp(&bp);
		let bp_img=img_bp_matrix(&bp,"pic/bp.png".to_string());
		let raw_school_matches:Vec<Vec<usize>>=Self::manyone7(&adj,&bp,&qu_sc,&vec![],0,st.len(),sc.len());
		//println!("TRANSFORM MATCHES!");
		let school_matches:Vec<Vec<usize>>=Self::transform_deepsearch_matches(&raw_school_matches,sc.len());
		//println!("transformed matches:{:?}",school_matches);
		println!("\n\nSCHOOL MATCHES:\n");
		/*
		for i in 0..school_matches.len(){
			println!("{:?}",school_matches[i]);
		}
		*/
		println!("{}",self.transform_matches_2string(&school_matches));
	}
	fn transform_matches_2string(&self,matches:&Vec<Vec<usize>>)->String{
		let mut txt:String=String::new();
		let mut header:Vec<usize>=vec![];
		if matches.len()>0{
			for i in 0..matches[0].len(){
				header.push(i);
			}
			txt+=&self.transform_residents_headers_2string(&header);
			for i in 0..matches.len(){
				txt+=&self.transform_hospitals_2string(&matches[i]);
			}
		}
		txt
	}
	fn transform_residents_headers_2string(&self,residents:&Vec<usize>)->String{
		let mut txt:String=String::new();
		for i in 0..residents.len(){
			txt+="r";
			txt+=&(residents[i]+1).to_string();
			if i<residents.len()-1{
				txt+=", ";
			}
			else{
				txt+="\n";
			}
		}
		for i in 0..residents.len(){
			txt+="----";
			if i==residents.len()-1{
				txt+="\n";
			}
		}
		txt
	}
	fn transform_hospitals_2string(&self, hospitals:&Vec<usize>)->String{
		let mut txt:String=String::new();
		for i in 0..hospitals.len(){
			txt+="h";
			txt+=&(hospitals[i]+1).to_string();
			if i<hospitals.len()-1{
				txt+=", ";
				let mut i_cp:usize=i;
				while (i_cp+1)/10>0{
					txt+=" ";
					i_cp/=10;
				}
			}
			else{
				txt+="\n";
			}
		}
		txt
	}
	fn bp_matrix_school(adj:&Vec<Vec<[usize;2]>>)->Vec<Vec<bool>>{
		let mut mat:Vec<Vec<bool>>=vec![];
		let n:usize=adj.len();
		let mut ni:usize=0;
		if n>0{
			ni=adj[0].len();
		}
		//for i in 0..n*n{
		for i in 0..n{		
			//println!("bp_matrix i:{}, len:{}",i,adj.len());
			//for j in 0..i{
			for j in 0..ni{
				let mut row:Vec<bool>=vec![];
				//let mut idx:usize=i*n+j;
				for k in 0..n{
					for m in 0..ni{
						// BE VERY CAREFUL!!!!!!!
						//if i!=k && j!=m{
						
						//if !(i==k && j==m){
						if !(i==k || j==m){
							//if !bp_efficient(&adj,&[i,j],&[k,m],0){
							if !Self::bp_efficient(&adj,&[i,j],&[k,m],0){
								row.push(true);
							}
							else{
								row.push(false);
								//row.push(true);
							}
						}
						else{
							// for the assumption that vertical pairs are stable! no blocking pairs there!
							if (j==m)&&(i !=k){
								row.push(true);
								//row.push(false);
							}
							else{
								row.push(false);
								//row.push(true);
							}
						}					
					}
				}
				mat.push(row);			
			}
		}
		for i in 0..mat.len(){
			//println!("{:?}",mat[i]);
		}
		mat
	}

	fn bp_check_results_school(
		bp:&Vec<Vec<bool>>,
		adj:&Vec<Vec<[usize;2]>>,
		cap:&Vec<usize>,
		match_:&Vec<usize>,
		n:usize,
		ni:usize
	){
		let mut is_ok:bool=true;
		let mut blockpairs:Vec<[usize;2]>=vec![];
		for i in 0..match_.len()-1{
			let im:usize=match_[i];
			for j in i+1..match_.len(){
				let jm:usize=match_[j];
				//if bp_efficient(&adj,&[i,im],&[j,jm],0){
				if Self::bp_efficient(&adj,&[i,im],&[j,jm],0){
					is_ok=false;
					blockpairs.push([i,im]);
					blockpairs.push([j,jm]);
				}
			}
		}
		//println!("ok: {}, blockpairs: {:?}",is_ok,blockpairs);
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
	// not neccessary!!!
	/*
	// Header not working
	fn bp_efficient_school(adj:&Vec<Vec<[usize;2]>>,a:&[usize;2],b:&[usize;2],diff:usize)->bool{
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
	*/

	fn create_adj_school( m_: &Vec<Vec<usize>>, w_: &Vec<Vec<usize>> ) -> Vec<Vec<[usize; 2]>> {
		println!("m:{:?}",m_);
		let mut adj = vec![];
		let l = m_.len();
		let k = w_.len();
		for i in 0..m_.len() {
			adj.push( vec![] );
			//for j in 0..m_.len() {
			for j in 0..w_.len() {
				//adj[i].push([l,l]);
				adj[i].push([l,k]);
			}
		}
		for i in 0..l {
			//for j in 0..m_.len() {
			/*
			for j in 0..w_.len() {
				let mij = m_[i][j];
				println!("i: {}, j: {}",i,j);
				let wij = w_[i][j];
				let posmw = m_[i].iter().position(|&x| x == mij).unwrap();
				let poswm = w_[i].iter().position(|&x| x == wij).unwrap();
				adj[i][mij][0] = posmw;
				adj[wij][i][1] = poswm;
			}
			*/
			for j in 0..k{
				//println!("m[{}]:{:?}\nw[{}]:{:?}",i,m_[i],j,w_[j]);
				//println!("i:{}, l:{}, j:{}, k:{}",i,l,j,k);
				let mij=m_[i][j];
				let wij=w_[j][i];
				let posmw=m_[i].iter().position(|&x| x==mij).unwrap();
				let poswm=w_[j].iter().position(|&x| x==wij).unwrap();
				adj[i][mij][0]=posmw;
				adj[wij][j][1]=poswm;
			}
		}
		println!("adj:");
		for i in 0..l{
			println!("{:?}",adj[i]);
		}
		return adj;
	}

	fn manyone(
		adj:&Vec<Vec<[usize;2]>>,// adjacence matrix
		bp:&Vec<Vec<bool>>, 	// stability matrix
		cap:&Vec<usize>, 		// school capacity vector
		tmp:&Vec<usize>, 		// temporary match vector
		i:usize,				// student i / step i
		n:usize,				// total number of students
		ni:usize				// total number of schools
	)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		if tmp.len()==n{
			println!("MATCH: {:?}",tmp);
			return vec![tmp.to_vec()];
		}
		else{
			//for j in 0..cap.len(){
			for j in 0..ni{
				//if count_el_in_vec(&tmp,j)<cap[j]{
				if cap[j]>0{
					let num:usize=i*ni+j;
					let mut is_ok:bool=true;
					for k in 0..tmp.len(){
						//println!("i:{}, j:{}, k:{}",i,j,k);
						let numk:usize=k*ni+tmp[k];
						//println!("num: {}, numk: {}",num,numk);
						//println!("i:{}, j:{}, k:{}, n:{}, tmp:{:?}",i,j,k,n,tmp);
						if !bp[num][numk]{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						let mut cap_ok:bool=true;
						for k in 0..ni{
							if j !=k && cap[k]>0{
								if adj[i][k][0]<adj[i][j][0]{
									println!("cap_false! i:{}, k:{}, j:{}, tmp:{:?}",i,k,j,tmp);
									cap_ok=false;
									break;
								}
								else{
									
								}
							}
						}
						if cap_ok{
							
							let mut tmpk:Vec<usize>=tmp.clone();
							tmpk.push(j);
							let mut capk:Vec<usize>=cap.clone();
							capk[j]-=1;
							//matches.append(&mut school_run(&bp,&cap,&tmpk,i+1,n,ni));
							matches.append(&mut Self::manyone(&adj,&bp,&capk,&tmpk,i+1,n,ni));
							
						}
					}
				}
			}
		}
		matches
	}
	// PLEASE TAKE CARE OF THE RIGHT SET OF AGENTS (SCHOOLS & STUDENTS), SWAPPING IS NOT ALLOWED!
	fn manyone7(
		adj:&Vec<Vec<[usize;2]>>,// adjacence matrix
		bp:&Vec<Vec<bool>>, 	// stability matrix
		cap:&Vec<usize>, 		// school capacity vector
		tmp:&Vec<usize>, 		// temporary match vector
		i:usize,				// student i / step i
		n:usize,				// total number of students
		ni:usize				// total number of schools
	)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		//println!("FN MANYONE3 cap:{:?}, tmp:{:?}, i:{}, n:{}, ni:{}",cap,tmp,i,n,ni);
		if tmp.len()==n{
			println!("MATCH: {:?}",tmp);
			return vec![tmp.to_vec()];
		}
		else{
			if i==n{
				//println!("EMPTY VEC!");
				return vec![];
			}
			//for j in 0..cap.len(){
			for j in 0..ni{
				let num:usize=i*ni+j;
				//if count_el_in_vec(&tmp,j)<cap[j]{
				if cap[j]>0{
					//let num:usize=i*ni+j;
					let mut is_ok:bool=true;
					for k in 0..tmp.len(){
						//println!("i:{}, j:{}, k:{}",i,j,k);
						//let numk:usize=k*ni+tmp[k];
						let numk:usize=tmp[k];
						//println!("num: {}, numk: {}",num,numk);
						//println!("i:{}, j:{}, k:{}, n:{}, tmp:{:?}",i,j,k,n,tmp);
						if !bp[num][numk]{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						//else{
							let mut cap_to_add:Vec<usize>=cap.clone();
							cap_to_add[j]-=1;
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone7(&adj,&bp,&cap_to_add,&tmp_to_add,i+1,n,ni));							
						//}
					}	
				}
				//if cap[j]==0{ // the same, see above!
				else{
					let mut least_run:bool=false;
					let mut least_pos:usize=0;
					// temporary assignment of current student i
					let mut least_student:usize=i;
					
					let mut least_k:usize=0;
					for k in 0..tmp.len(){
						//let school:usize=tmp[k]/ni;
						let school:usize=tmp[k]%ni;
						//if j==k{
						if j==school{
							least_run=true;
							//println!("j==school! school:{}, j:{}, k:{}",school,j,k);
							//let student:usize=tmp[k]%ni;
							let student:usize=tmp[k]/ni;
							// 0 or 1 ??????? VERY IMPORTANT!!!!!!!
							//if adj[student][k][1]>least_pos{
							if adj[student][j][1]>least_pos{
								//println!("least_pos:{}, student:{}, k:{}, adj[{}][{}][0]:{}, adj[{}][{}][1]:{}",
								//	least_pos,student,k,student,k,adj[student][k][0],student,k,adj[student][k][1]);
								//least_pos=adj[student][k][1];
								least_pos=adj[student][j][1];
								least_student=student;
								least_k=k;
							}
						}
					}
					if least_run{
						if least_student !=i{
							//println!("i:{},ni:{},n:{},least_student:{},least_k:{},j:{},tmp:{:?}, cap:{:?}",i,ni,n,least_student,least_k,j,tmp,cap);
							//let delnode:usize=least_student*ni+least_k;
							let delnode:usize=least_student*ni+j;
							//println!("delnode:{}",delnode);
							let delpos:usize=tmp.iter().position(|x| *x==delnode).unwrap();
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.remove(delpos);
							//tmp_to_add.push(i*ni+j);
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone7(&adj,&bp,&cap,&tmp_to_add,i+1,n,ni));
						}
					}
				}				
				
			}
		}
		matches
	}
	// PLEASE TAKE CARE OF THE RIGHT SET OF AGENTS (SCHOOLS & STUDENTS), SWAPPING IS NOT ALLOWED!
	fn manyone4(
		adj:&Vec<Vec<[usize;2]>>,// adjacence matrix
		bp:&Vec<Vec<bool>>, 	// stability matrix
		cap:&Vec<usize>, 		// school capacity vector
		tmp:&Vec<usize>, 		// temporary match vector
		i:usize,				// student i / step i
		n:usize,				// total number of students
		ni:usize				// total number of schools
	)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		println!("FN MANYONE3 cap:{:?}, tmp:{:?}, i:{}, n:{}, ni:{}",cap,tmp,i,n,ni);
		if tmp.len()==n{
			println!("MATCH: {:?}",tmp);
			return vec![tmp.to_vec()];
		}
		else{
			if i==n{
				println!("EMPTY VEC!");
				return vec![];
			}
			//for j in 0..cap.len(){
			for j in 0..ni{
				let num:usize=i*ni+j;
				//if count_el_in_vec(&tmp,j)<cap[j]{
				if cap[j]>0{
					//let num:usize=i*ni+j;
					let mut is_ok:bool=true;
					for k in 0..tmp.len(){
						//println!("i:{}, j:{}, k:{}",i,j,k);
						//let numk:usize=k*ni+tmp[k];
						let numk:usize=tmp[k];
						//println!("num: {}, numk: {}",num,numk);
						//println!("i:{}, j:{}, k:{}, n:{}, tmp:{:?}",i,j,k,n,tmp);
						if !bp[num][numk]{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						//else{
							let mut cap_to_add:Vec<usize>=cap.clone();
							cap_to_add[j]-=1;
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone4(&adj,&bp,&cap_to_add,&tmp_to_add,i+1,n,ni));							
						//}
					}	
				}
				//if cap[j]==0{ // the same, see above!
				else{
					let mut least_run:bool=false;
					let mut least_pos:usize=0;
					// temporary assignment of current student i
					let mut least_student:usize=i;
					
					let mut least_k:usize=0;
					for k in 0..tmp.len(){
						//let school:usize=tmp[k]/ni;
						let school:usize=tmp[k]%ni;
						//if j==k{
						if j==school{
							least_run=true;
							println!("j==school! school:{}, j:{}, k:{}",school,j,k);
							//let student:usize=tmp[k]%ni;
							let student:usize=tmp[k]/ni;
							// 0 or 1 ??????? VERY IMPORTANT!!!!!!!
							//if adj[student][k][1]>least_pos{
							if adj[student][j][1]>least_pos{
								println!("least_pos:{}, student:{}, k:{}, adj[{}][{}][0]:{}, adj[{}][{}][1]:{}",
									least_pos,student,k,student,k,adj[student][k][0],student,k,adj[student][k][1]);
								//least_pos=adj[student][k][1];
								least_pos=adj[student][j][1];
								least_student=student;
								least_k=k;
							}
						}
					}
					if least_run{
						if least_student !=i{
							println!("i:{},ni:{},n:{},least_student:{},least_k:{},j:{},tmp:{:?}, cap:{:?}",i,ni,n,least_student,least_k,j,tmp,cap);
							//let delnode:usize=least_student*ni+least_k;
							let delnode:usize=least_student*ni+j;
							println!("delnode:{}",delnode);
							let delpos:usize=tmp.iter().position(|x| *x==delnode).unwrap();
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.remove(delpos);
							//tmp_to_add.push(i*ni+j);
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone4(&adj,&bp,&cap,&tmp_to_add,i+1,n,ni));
						}
					}
				}				
				
			}
		}
		matches
	}
	// PLEASE TAKE CARE OF THE RIGHT SET OF AGENTS (SCHOOLS & STUDENTS), SWAPPING IS NOT ALLOWED!
	fn manyone3(
		adj:&Vec<Vec<[usize;2]>>,// adjacence matrix
		bp:&Vec<Vec<bool>>, 	// stability matrix
		cap:&Vec<usize>, 		// school capacity vector
		tmp:&Vec<usize>, 		// temporary match vector
		i:usize,				// student i / step i
		n:usize,				// total number of students
		ni:usize				// total number of schools
	)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		println!("FN MANYONE3 cap:{:?}, tmp:{:?}, i:{}, n:{}, ni:{}",cap,tmp,i,n,ni);
		if tmp.len()==n{
			println!("MATCH: {:?}",tmp);
			return vec![tmp.to_vec()];
		}
		else{
			if i==n{
				println!("EMPTY VEC!");
				return vec![];
			}
			//for j in 0..cap.len(){
			for j in 0..ni{
				let num:usize=i*ni+j;
				//if count_el_in_vec(&tmp,j)<cap[j]{
				if cap[j]>0{
					//let num:usize=i*ni+j;
					let mut is_ok:bool=true;
					for k in 0..tmp.len(){
						//println!("i:{}, j:{}, k:{}",i,j,k);
						let numk:usize=k*ni+tmp[k];
						//println!("num: {}, numk: {}",num,numk);
						//println!("i:{}, j:{}, k:{}, n:{}, tmp:{:?}",i,j,k,n,tmp);
						if !bp[num][numk]{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						//else{
							let mut cap_to_add:Vec<usize>=cap.clone();
							cap_to_add[j]-=1;
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone3(&adj,&bp,&cap_to_add,&tmp_to_add,i+1,n,ni));							
						//}
					}	
				}
				//if cap[j]==0{ // the same, see above!
				else{
					let mut least_run:bool=false;
					let mut least_pos:usize=0;
					// temporary assignment of current student i
					let mut least_student:usize=i;
					
					let mut least_k:usize=0;
					for k in 0..tmp.len(){
						//let school:usize=tmp[k]/ni;
						let school:usize=tmp[k]%ni;
						//if j==k{
						if j==school{
							least_run=true;
							println!("j==school! school:{}, j:{}, k:{}",school,j,k);
							//let student:usize=tmp[k]%ni;
							let student:usize=tmp[k]/ni;
							// 0 or 1 ??????? VERY IMPORTANT!!!!!!!
							//if adj[student][k][1]>least_pos{
							if adj[student][j][1]>least_pos{
								println!("least_pos:{}, student:{}, k:{}, adj[{}][{}][0]:{}, adj[{}][{}][1]:{}",
									least_pos,student,k,student,k,adj[student][k][0],student,k,adj[student][k][1]);
								//least_pos=adj[student][k][1];
								least_pos=adj[student][j][1];
								least_student=student;
								least_k=k;
							}
						}
					}
					if least_run{
						if least_student !=i{
							println!("i:{},ni:{},n:{},least_student:{},least_k:{},j:{},tmp:{:?}, cap:{:?}",i,ni,n,least_student,least_k,j,tmp,cap);
							//let delnode:usize=least_student*ni+least_k;
							let delnode:usize=least_student*ni+j;
							println!("delnode:{}",delnode);
							let delpos:usize=tmp.iter().position(|x| *x==delnode).unwrap();
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.remove(delpos);
							//tmp_to_add.push(i*ni+j);
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone3(&adj,&bp,&cap,&tmp_to_add,i+1,n,ni));
						}
					}
				}				
				
			}
		}
		matches
	}
	// PLEASE TAKE CARE OF THE RIGHT SET OF AGENTS (SCHOOLS & STUDENTS), SWAPPING IS NOT ALLOWED!
	fn manyone2(
		adj:&Vec<Vec<[usize;2]>>,// adjacence matrix
		bp:&Vec<Vec<bool>>, 	// stability matrix
		cap:&Vec<usize>, 		// school capacity vector
		tmp:&Vec<usize>, 		// temporary match vector
		i:usize,				// student i / step i
		n:usize,				// total number of students
		ni:usize				// total number of schools
	)->Vec<Vec<usize>>{
		let mut matches:Vec<Vec<usize>>=vec![];
		println!("cap:{:?}, tmp:{:?}, i:{}, n:{}, ni:{}",cap,tmp,i,n,ni);
		if tmp.len()==n{
			println!("MATCH: {:?}",tmp);
			return vec![tmp.to_vec()];
		}
		else{
			if i==n{
				return vec![];
			}
			//for j in 0..cap.len(){
			for j in 0..ni{
				//if count_el_in_vec(&tmp,j)<cap[j]{
				if cap[j]>0{
					let num:usize=i*ni+j;
					let mut is_ok:bool=true;
					for k in 0..tmp.len(){
						//println!("i:{}, j:{}, k:{}",i,j,k);
						let numk:usize=k*ni+tmp[k];
						//println!("num: {}, numk: {}",num,numk);
						//println!("i:{}, j:{}, k:{}, n:{}, tmp:{:?}",i,j,k,n,tmp);
						if !bp[num][numk]{
							is_ok=false;
							break;
						}
					}
					if is_ok{
						//else{
							let mut cap_to_add:Vec<usize>=cap.clone();
							cap_to_add[j]-=1;
							let mut tmp_to_add:Vec<usize>=tmp.clone();
							tmp_to_add.push(num);
							matches.append(&mut Self::manyone2(&adj,&bp,&cap_to_add,&tmp_to_add,i+1,n,ni));							
						//}
					}	
				}
				//if cap[j]==0{ // the same, see above!
				else{
					let mut least_pos:usize=0;
					// temporary assignment of current student i
					let mut least_student:usize=i;
					let mut least_k:usize=0;
					for k in 0..tmp.len(){
						//let school:usize=tmp[k]/ni;
						let school:usize=tmp[k]%ni;
						if j==k{
							println!("j==k: j:{}, k:{}",j,k);
							//let student:usize=tmp[k]%ni;
							let student:usize=tmp[k]/ni;
							// 0 or 1 ??????? VERY IMPORTANT!!!!!!!
							if adj[student][k][0]>least_pos{
								println!("least_pos:{}, student:{}, k:{}, adj[{}][{}][0]:{}, adj[{}][{}][1]:{}",
									least_pos,student,k,student,k,adj[student][k][0],student,k,adj[student][k][1]);
								least_pos=adj[student][k][1];
								least_student=student;
								least_k=k;
							}
						}
					}
					if least_student !=i{
						println!("i:{},ni:{},n:{},least_student:{},least_k:{},j:{},tmp:{:?}, cap:{:?}",i,ni,n,least_student,least_k,j,tmp,cap);
						let delnode:usize=least_student*ni+least_k;
						println!("delnode:{}",delnode);
						let delpos:usize=tmp.iter().position(|x| *x==delnode).unwrap();
						let mut tmp_to_add:Vec<usize>=tmp.clone();
						tmp_to_add.remove(delpos);
						tmp_to_add.push(i*ni+j);
						matches.append(&mut Self::manyone2(&adj,&bp,&cap,&tmp_to_add,i+1,n,ni));
					}
				}				
				
			}
		}
		matches
	}
	fn transform_deepsearch_matches(matches:&Vec<Vec<usize>>,ni:usize)->Vec<Vec<usize>>{
		let mut transformed:Vec<Vec<usize>>=vec![];
		for i in 0..matches.len(){
			let mut transformed_i:Vec<usize>=vec![];
			for j in 0..matches[i].len(){
				//let school:usize=matches[i][j]%((j+1)*ni);
				let school:usize=matches[i][j]%ni;
				transformed_i.push(school);
			}
			transformed.push(transformed_i);
		}
		transformed
	}
	fn enumeration(&self){
		let mut matches:Vec<Vec<usize>>=vec![];
		//let mut m:Vec<Vec<usize>>=self.sc.clone();
		//let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut m:Vec<Vec<usize>>=u16_2_usize(&self.sc);
		let mut w:Vec<Vec<usize>>=u16_2_usize(&self.st);
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
		println!("RUN OPTIMAL");
		let opt:Vec<Vec<usize>>=Self::get_posets_iter2(&m,&w,&self.qu_sc,&self.qu_st,&wtmp,&mtmp,&idx);
		println!("RUN PESSIMAL");
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(w.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(m.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];w.len()];
		let pess_orig:Vec<Vec<usize>>=Self::get_posets_iter2(&w,&m,&self.qu_st,&self.qu_sc,&wtmp,&mtmp,&idx);
		let pess:Vec<Vec<usize>>=Self::traverse_wtmp(&pess_orig,m.len());
		println!("OPTIMAL:\n{:?}",opt);
		println!("PESSIMAL:\n{:?}",pess);
		//let (m_toggled,w_toggled)=Self::toggle_node((&m,&w),0);

		let (m_shorted_m,w_shorted_m)=Self::shorten_lists7(&m,&w,&opt);
		println!("SHORTEN LISTS OPTIMAL\nm_shorted_m:{:?}\nw_shorted_m:{:?}",m_shorted_m,w_shorted_m);
		println!("START SHORTEN LISTS FOR PESSIMAL!");
		let (w_shorted_w,m_shorted_w)=Self::shorten_lists7(&w_shorted_m,&m_shorted_m,&Self::traverse_wtmp(&pess,w.len()));
		//let (w_shorted_w,m_shorted_w)=Self::shorten_lists7(&w_shorted_m,&m_shorted_m,&pess);
		println!("SHORTEN LISTS OPTIMAL\nm_shorted_w:{:?}\nw_shorted_w:{:?}",m_shorted_w,w_shorted_w);

		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match3(&m,&w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&opt,&pess);
		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match7(&m,&w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&opt,&pess);
		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match77(&m,&w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&vec![opt.clone()],&pess);
		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match7712(&m,&w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&vec![opt.clone()],&pess);
		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match12(&m,&w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&vec![opt.clone()],&pess);
		let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match12(&m_shorted_w,&w_shorted_w,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&vec![opt.clone()],&pess);
		//let mut all_matches:Vec<Vec<Vec<usize>>>=Self::enumerate_from_match3(&m_toggled,&w_toggled,&self.qu_sc,&self.qu_st,&vec![opt.clone()],&opt,&pess);
		//println!("ALL MATCHES:\n{:?}",all_matches);
		println!("ALL MATCHES:");
		for i in 0..all_matches.len(){
			//println!("i:{}, {:?} ",i,all_matches[i]);
			println!("i:{}, {:?} ### STABLE: {}",i,all_matches[i],Self::check_chain_stability(&m,&w,&all_matches[i]));
		}
		
	}
	fn check_chain_stability(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,chain:&Vec<Vec<usize>>)->bool{
		let wchain:Vec<Vec<usize>>=Self::traverse_wtmp(&chain,w.len());
		//println!("CHECK CHAIN STABILITY\nm:{:?}\nw:{:?}\nchain: {:?}\nwchain:{:?}",m,w,chain,wchain);
		let mut stable:bool=true;
		for i in 0..chain.len(){
			if chain[i].len()>0{
				let least_preferred_pos:Option<usize>=Self::get_least_preferred_pos(&m,&chain,i);
				if !least_preferred_pos.is_none(){
					let least:usize=least_preferred_pos.unwrap();
					for j in 0..least{
						//let wij:usize=chain[i][j];
						let wij:usize=m[i][j];
						let wleast_package:Option<usize>=Self::get_least_preferred_pos(&w,&wchain,wij);
						if !wleast_package.is_none(){
							let wleast_pos:usize=wleast_package.unwrap();
							let mpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
							if mpos<wleast_pos{
								//println!("FALSE AT i:{}, wij:{}, wleast_pos:{}, mpos:{}",i,wij,wleast_pos,mpos);
								return false;
							}
						}
						else{
							println!("BIG PROBLEM IN CHECK CHAIN STABILITY! ACCEPTOR'S SIDE!");
						}
					}
				}
				else{
					println!("BIG PROBLEM IN CHECK CHAIN STABILITY! PROPOSER'S SIDE!");
				}
			}
			
		}
		true
	}
	fn get_least_preferred_pos(m:&Vec<Vec<usize>>,chain:&Vec<Vec<usize>>,pos:usize)->Option<usize>{
		if chain[pos].len()>0{
			let mut least_pos:usize=0;
			for j in 0..chain[pos].len(){
				let posj:usize=m[pos].iter().position(|x| *x==chain[pos][j]).unwrap();
				if least_pos<posj{
					least_pos=posj;
				}
			}
			Some(least_pos)
		}
		else{
			None
		}	
	}
	fn get_most_preferred_pos(m:&Vec<Vec<usize>>,chain:&Vec<Vec<usize>>,pos:usize)->Option<usize>{
		if chain[pos].len()>0{
			let mut best_pos:usize=chain[pos].len()-1;
			for j in 0..chain[pos].len(){
				let posj:usize=m[pos].iter().position(|x| *x==chain[pos][j]).unwrap();
				if best_pos>posj{
					best_pos=posj;
				}
			}
			Some(best_pos)
		}
		else{
			None
		}	
	}
	fn enumerate_from_match7712(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		matchpackage:&Vec<Vec<Vec<usize>>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		println!("ENUMERATE 7712");
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&m,&w,&match_);
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		// msc, wsc in shorten_lists?
		//let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&w_sc,&m_sc,&match_transformed);
		//println!("SHORTED LISTS:\nm:{:?}\nw:{:?}",m_sc1,w_sc1);
		//let mut big_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=matchstack.clone();
		for match_ in matchpackage{
			//let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
			println!("BEFORE SHORTEN LISTS 7:\nm:{:?}\nw:{:?}\nmmatch_:{:?}\nwmatch:{:?}",m,w,match_,Self::traverse_wtmp(&match_,w.len()));
			let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists7(&m,&w,&match_);
			println!("AFTER  SHORTEN LISTS 7:\nm:{:?}\nw:{:?}\nmmatch_:{:?}\nwmatch:{:?}",m_sc1,w_sc1,match_,Self::traverse_wtmp(&match_,w.len()));
			let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
			let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
			let last_pess:Vec<usize>=Self::get_last_pess(&m,&idx,&pess);
			let idx_coord:Vec<(usize,usize)>=Self::get_idx_len(&idx);
			println!("IDX_COORD: {:?}, IDX: {:?}",idx_coord,idx);
			//for i in 0..m_sc1.len(){
			//for i in 0..idx_len{
			for i in 0..idx_coord.len(){
				//let (idx,mtmp,wtmp)=Self::remove_idx_pos_iter2(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_);
				let datapackage=Self::remove_idx_pos_iter2(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_,&last_pess);
				if !datapackage.is_none(){
					println!("IDX COORD in {}: {:?}",i,idx_coord[i]);
					let (idx,mtmp,wtmp)=datapackage.unwrap();
					let match_i:Vec<Vec<usize>>=Self::get_posets_iter4(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
					if match_i.len()>0{
						//if Self::check_match_within_pess(&m_sc1,&w_sc1,&match_i,&pess){
						// CHECK WHETHER CHAIN IS STABLE. IT'S NOT THAT WHAT WE WANT!
						//if Self::check_chain_stability(&m_sc1,&w_sc1,&match_i){
							if !matches.contains(&match_i){
								if !matchstack.contains(&match_i){
									//println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
									if Self::check_consistency_mtmp_wtmp(&match_i){
										matches.push(match_i.clone());
									}
								}
								else{
									println!("MATCHSTACK CONTAINS! {:?}",match_i);
								}
							}
						//}
					}
				}
			}
		}
		big_matchstack.append(&mut matches.clone());
		println!("MATCHSTACK: {:?}\nMATCHES: {:?}",matchstack,matches);
		if matches.len()>0{
			//big_matchstack.append(&mut Self::enumerate_from_match7712(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess).clone());
			
			//COMMENTED!
			big_matchstack=Self::enumerate_from_match7712(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess);
		}
		big_matchstack
	}	
	fn enumerate_from_match12(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		matchpackage:&Vec<Vec<Vec<usize>>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		println!("ENUMERATE 7712");
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=matchstack.clone();
		for match_ in matchpackage{
			//let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
			println!("BEFORE SHORTEN LISTS 7:\nm:{:?}\nw:{:?}\nmmatch_:{:?}\nwmatch:{:?}",m,w,match_,Self::traverse_wtmp(&match_,w.len()));
			let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists7(&m,&w,&match_);
			println!("AFTER  SHORTEN LISTS 7:\nm:{:?}\nw:{:?}\nmmatch_:{:?}\nwmatch:{:?}",m_sc1,w_sc1,match_,Self::traverse_wtmp(&match_,w.len()));
			let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
			let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
			println!("IDX FOR MATCH:{:?}\nIDX:{:?}",match_,idx);
			let last_pess:Vec<usize>=Self::get_last_pess(&m,&idx,&pess);
			println!("LAST_PESS:{:?}",last_pess);
			let idx_coord:Vec<(usize,usize)>=Self::get_idx_len(&idx);
			println!("IDX_COORD: {:?}, IDX: {:?}",idx_coord,idx);
			//for i in 0..m_sc1.len(){
			//for i in 0..idx_len{
			for i in 0..idx_coord.len(){
				//let (idx,mtmp,wtmp)=Self::remove_idx_pos_iter2(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_);
				let datapackage=Self::remove_idx_pos_iter2(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_,&last_pess);
				if !datapackage.is_none(){
					println!("IDX COORD in {}: {:?}",i,idx_coord[i]);
					let (idx,mtmp,wtmp)=datapackage.unwrap();
					let match_i:Vec<Vec<usize>>=Self::get_posets_iter7(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx,&last_pess);
					if match_i.len()>0{
						//if Self::check_match_within_pess(&m_sc1,&w_sc1,&match_i,&pess){
						// CHECK WHETHER CHAIN IS STABLE. IT'S NOT THAT WHAT WE WANT!
						//if Self::check_chain_stability(&m_sc1,&w_sc1,&match_i){
							if !matches.contains(&match_i){
								if !matchstack.contains(&match_i){
									//println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
									if Self::check_consistency_mtmp_wtmp(&match_i){
										matches.push(match_i.clone());
									}
								}
								else{
									println!("MATCHSTACK CONTAINS! {:?}",match_i);
								}
							}
						//}
					}
				}
			}
		}
		big_matchstack.append(&mut matches.clone());
		println!("MATCHSTACK: {:?}\nMATCHES: {:?}",matchstack,matches);
		if matches.len()>0{
			//big_matchstack.append(&mut Self::enumerate_from_match7712(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess).clone());
			
			//COMMENTED!
			//big_matchstack=Self::enumerate_from_match12(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess);
		}
		//matches
		big_matchstack
	}		
	fn enumerate_from_match77(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		matchpackage:&Vec<Vec<Vec<usize>>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		println!("ENUMERATE 77");
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&m,&w,&match_);
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		// msc, wsc in shorten_lists?
		//let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&w_sc,&m_sc,&match_transformed);
		//println!("SHORTED LISTS:\nm:{:?}\nw:{:?}",m_sc1,w_sc1);
		//let mut big_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=matchstack.clone();
		for match_ in matchpackage{
			let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
			let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
			let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
			let last_pess:Vec<usize>=Self::get_last_pess(&m,&idx,&pess);
			let idx_coord:Vec<(usize,usize)>=Self::get_idx_len(&idx);
			println!("IDX_COORD: {:?}, IDX: {:?}",idx_coord,idx);
			//for i in 0..m_sc1.len(){
			//for i in 0..idx_len{
			for i in 0..idx_coord.len(){
				let (idx,mtmp,wtmp)=Self::remove_idx_pos_iter(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_);
				let match_i:Vec<Vec<usize>>=Self::get_posets_iter4(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
				if match_i.len()>0{
					if !matches.contains(&match_i){
						if !matchstack.contains(&match_i){
							//println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
							if Self::check_consistency_mtmp_wtmp(&match_i){
								matches.push(match_i.clone());
							}
						}
						else{
							println!("MATCHSTACK CONTAINS! {:?}",match_i);
						}
					}
				}
			}
		}
		big_matchstack.append(&mut matches.clone());
		println!("MATCHSTACK: {:?}\nMATCHES: {:?}",matchstack,matches);
		if matches.len()>0{
			//big_matchstack.append(&mut Self::enumerate_from_match77(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess).clone());
			
			//COMMENTED!
			//big_matchstack=Self::enumerate_from_match77(&m,&w,&qu_m,&qu_w,&big_matchstack,&matches,&pess);
		}
		//matches
		big_matchstack
	}				
	fn enumerate_from_match7(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		match_:&Vec<Vec<usize>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		if match_==pess{
			//return vec![];
		}
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&m,&w,&match_);
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
		// msc, wsc in shorten_lists?
		//let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&w_sc,&m_sc,&match_transformed);
		let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		//println!("SHORTED LISTS:\nm:{:?}\nw:{:?}",m_sc1,w_sc1);
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		
		
		/*
		let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),2);
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
		let match_i:Vec<Vec<usize>>=Self::get_posets_iter3(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
		println!("ENUMERATION 4 ### MATCH_I: {:?}",match_i);
		*/
		let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
		let idx_coord:Vec<(usize,usize)>=Self::get_idx_len(&idx);
		println!("IDX_COORD: {:?}, IDX: {:?}",idx_coord,idx);
		//for i in 0..m_sc1.len(){
		//for i in 0..idx_len{
		for i in 0..idx_coord.len(){
			//let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),i);
			//println!("i:{},\nm_toggled:{:?}\nw_toggled:{:?}\n",i,m_toggled,w_toggled);
			//let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
			//let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
			//let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
			//let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
			//println!("get idx:{:?}",idx);
			
			//let idx_len:usize=idx[i].len();
			//idx[i][idx_len-1]+=1;
			//let (idx,mtmp,wtmp)=Self::increment_idx_pos2(&m_sc1,&w_sc1,&idx,i,&match_);
			//let (idx,mtmp,wtmp)=Self::remove_idx_pos_iter(&m_sc1,&w_sc1,&idx,i,&match_);
			let (idx,mtmp,wtmp)=Self::remove_idx_pos_iter(&m_sc1,&w_sc1,&idx,idx_coord[i],&match_);
			//idx[i].remove(0);
			//let match_i:Vec<Vec<usize>>=Self::get_posets_iter2(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			//println!("#### ENUMERATION AT {} ####",i);
			//let match_i:Vec<Vec<usize>>=Self::get_posets_iter3(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			let match_i:Vec<Vec<usize>>=Self::get_posets_iter4(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			if match_i.len()>0{
				if !matches.contains(&match_i){
					if !matchstack.contains(&match_i){
						//println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
						if Self::check_consistency_mtmp_wtmp(&match_i){
							matches.push(match_i.clone());
						}
					}
					else{
						println!("MATCHSTACK CONTAINS! {:?}",match_i);
					}
				}
				big_matchstack.append(&mut matches.clone());
				//big_matchstack.append(&mut Self::enumerate_from_match4(&m_toggled,&w_toggled,&qu_m,&qu_w,&big_matchstack,&match_i,&pess).clone());
				big_matchstack.append(&mut Self::enumerate_from_match7(&m_sc1,&w_sc1,&qu_m,&qu_w,&big_matchstack,&match_i,&pess).clone());
			}
			let mut output:Vec<Vec<Vec<usize>>>=matchstack.clone();
			//let mut output:Vec<Vec<Vec<usize>>>=matches.clone();
			let mut matches_output:Vec<Vec<Vec<usize>>>=matches.clone();
			output.append(&mut matches_output);
			let mut new_output_matchstack:Vec<Vec<Vec<usize>>>=vec![];
			let mut traversed_matches:Vec<Vec<Vec<usize>>>=vec![];
			for match_i in &matches{
				//traversed_matches.push(Self::traverse_wtmp(match_i,w.len()));
				traversed_matches.push(match_i.to_vec());
			}
			println!("####### ENUMERATION MATCHES: #######");
			for match_i in &traversed_matches{
			//for match_i in &matches{
				//println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
				println!("{:?}",match_i);
			}
			println!("####### CLEAN MATCHES: #######");
			let mut clean_matches:Vec<Vec<Vec<usize>>>=vec![];
			for match_i in &matches{
				if Self::check_consistency_mtmp_wtmp(&match_i){
					clean_matches.push(match_i.to_vec());
					println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
				}
			}			
		}
		matches
	}				
	fn enumerate_from_match4(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		match_:&Vec<Vec<usize>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		if match_==pess{
			//return vec![];
		}
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&m,&w,&match_);
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
		// msc, wsc in shorten_lists?
		//let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&w_sc,&m_sc,&match_transformed);
		let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		//println!("SHORTED LISTS:\nm:{:?}\nw:{:?}",m_sc1,w_sc1);
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=vec![];	
		/*
		let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),2);
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
		let match_i:Vec<Vec<usize>>=Self::get_posets_iter3(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
		println!("ENUMERATION 4 ### MATCH_I: {:?}",match_i);
		*/
		for i in 0..m_sc1.len(){
			//let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),i);
			//println!("i:{},\nm_toggled:{:?}\nw_toggled:{:?}\n",i,m_toggled,w_toggled);
			let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
			let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
			//let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
			let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m_sc1,&w_sc1,&match_);
			//println!("get idx:{:?}",idx);
			
			//let idx_len:usize=idx[i].len();
			//idx[i][idx_len-1]+=1;
			let (idx,mtmp,wtmp)=Self::increment_idx_pos2(&m_sc1,&w_sc1,&idx,i,&match_);
			//idx[i].remove(0);
			//let match_i:Vec<Vec<usize>>=Self::get_posets_iter2(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			//println!("#### ENUMERATION AT {} ####",i);
			//let match_i:Vec<Vec<usize>>=Self::get_posets_iter3(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			let match_i:Vec<Vec<usize>>=Self::get_posets_iter4(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			if match_i.len()>0{
				if !matches.contains(&match_i){
					if !matchstack.contains(&match_i){
						//println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
						if Self::check_consistency_mtmp_wtmp(&match_i){
							matches.push(match_i.clone());
						}
					}
					else{
						println!("MATCHSTACK CONTAINS! {:?}",match_i);
					}
				}
				big_matchstack.append(&mut matches.clone());
				//big_matchstack.append(&mut Self::enumerate_from_match4(&m_toggled,&w_toggled,&qu_m,&qu_w,&big_matchstack,&match_i,&pess).clone());
				big_matchstack.append(&mut Self::enumerate_from_match4(&m_sc1,&w_sc1,&qu_m,&qu_w,&big_matchstack,&match_i,&pess).clone());
			}
			
			let mut output:Vec<Vec<Vec<usize>>>=matchstack.clone();
			//let mut output:Vec<Vec<Vec<usize>>>=matches.clone();
			let mut matches_output:Vec<Vec<Vec<usize>>>=matches.clone();
			output.append(&mut matches_output);
			let mut new_output_matchstack:Vec<Vec<Vec<usize>>>=vec![];
			let mut traversed_matches:Vec<Vec<Vec<usize>>>=vec![];
			for match_i in &matches{
				//traversed_matches.push(Self::traverse_wtmp(match_i,w.len()));
				traversed_matches.push(match_i.to_vec());
			}
			println!("####### ENUMERATION MATCHES: #######");
			for match_i in &traversed_matches{
			//for match_i in &matches{
				//println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
				println!("{:?}",match_i);
			}
			println!("####### CLEAN MATCHES: #######");
			let mut clean_matches:Vec<Vec<Vec<usize>>>=vec![];
			for match_i in &matches{
				if Self::check_consistency_mtmp_wtmp(&match_i){
					clean_matches.push(match_i.to_vec());
					println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
				}
			}			
		}
		matches
	}
	fn enumerate_from_match3(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		match_:&Vec<Vec<usize>>,pess:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		//println!("ENUMERATE FROM MATCH:\nm:{:?}\nw:{:?}\nmatchstack:{:?}\nmatch_:{:?},pess:{:?}",m,w,matchstack,match_,pess);
		if match_==pess{
			//return vec![];
		}
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&m,&w,&match_);
		//let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
		// msc, wsc in shorten_lists?
		//let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists2(&w_sc,&m_sc,&match_transformed);
		let (mut m_sc1, mut w_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists_wrapper(&m,&w,&match_);
		//println!("SHORTED LISTS:\nm:{:?}\nw:{:?}",m_sc1,w_sc1);
		let mut big_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		for i in 0..m_sc1.len(){
			let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),i);
			//println!("i:{},\nm_toggled:{:?}\nw_toggled:{:?}\n",i,m_toggled,w_toggled);
			let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
			let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
			let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
			//let match_i:Vec<Vec<usize>>=Self::get_posets_iter2(&m_sc1,&w_sc1,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			//println!("#### ENUMERATION AT {} ####",i);
			let match_i:Vec<Vec<usize>>=Self::get_posets_iter3(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			if match_i.len()>0{
				if !matches.contains(&match_i){
					if !matchstack.contains(&match_i){
						println!("match_i:{:?}",Self::traverse_wtmp(&match_i,w.len()));
						if Self::check_consistency_mtmp_wtmp(&match_i){
							matches.push(match_i.clone());
						}
					}
				}
				big_matchstack.append(&mut matches.clone());
				big_matchstack.append(&mut Self::enumerate_from_match3(&m_toggled,&w_toggled,&qu_m,&qu_w,&big_matchstack,&match_i,&pess).clone());
			}
		}
		let mut output:Vec<Vec<Vec<usize>>>=matchstack.clone();
		//let mut output:Vec<Vec<Vec<usize>>>=matches.clone();
		let mut matches_output:Vec<Vec<Vec<usize>>>=matches.clone();
		output.append(&mut matches_output);
		let mut new_output_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		let mut traversed_matches:Vec<Vec<Vec<usize>>>=vec![];
		for match_i in &matches{
			traversed_matches.push(Self::traverse_wtmp(match_i,w.len()));
		}
		println!("####### ENUMERATION MATCHES: #######");
		for match_i in &traversed_matches{
		//for match_i in &matches{
			//println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
			println!("{:?}",match_i);
		}
		println!("####### CLEAN MATCHES: #######");
		let mut clean_matches:Vec<Vec<Vec<usize>>>=vec![];
		for match_i in &matches{
			if Self::check_consistency_mtmp_wtmp(&match_i){
				clean_matches.push(match_i.to_vec());
				println!("{:?}",Self::traverse_wtmp(match_i,w.len()));
			}
		}
		
		for match_i in matches{
			if Self::check_consistency_mtmp_wtmp(&match_i){
				new_output_matchstack.append(&mut Self::enumerate_from_match3(&m_sc1,&w_sc1,&qu_m,&qu_w,&output,&match_i,&pess));
			}
			
		}
		let mut traversed_matchstack:Vec<Vec<Vec<usize>>>=vec![];
		for i in 0..big_matchstack.len(){
			traversed_matchstack.push(Self::traverse_wtmp(&big_matchstack[i],w.len()));
		}
		//output
		//new_output_matchstack
		//big_matchstack
		traversed_matchstack
	}
	
	fn show_lists(&self){
		println!("Schools:");
		for i in 0..self.sc.len(){
			println!("{:?}",self.sc[i]);
		}
		println!("Students:");
		for i in 0..self.st.len(){
			println!("{:?}",self.st[i]);
		}
		println!("Quota Schools: {:?}",self.qu_sc);
		println!("Quota Students: {:?}",self.qu_st);
	}
	fn complete_lists(&mut self){
		let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		if consistency{
			let st_last:u16=self.st.len() as u16;
			//let st_last:u16=self.st.len() as u16;
			let sc_last:u16=self.sc.len() as u16;
			for i in 0..self.sc.len(){				
				for j in self.sc[i].len()-1 as usize..(st_last) as usize{
					self.sc[i].push(st_last);
				}
			}
			let lastsc:Vec<u16>=vec![st_last;(st_last+1) as usize];
			self.sc.push(lastsc);
			for i in 0..self.st.len(){				
				for j in self.st[i].len()-1 as usize..(sc_last-1) as usize{
					self.st[i].push(sc_last);
				}
			}
			let lastst:Vec<u16>=vec![sc_last;sc_last as usize];
			self.st.push(lastst);
		}
		else{
			// ???????
		}
		self.qu_sc.push(1);
		self.qu_st.push(1);
	}

	fn complete_lists_123(&mut self){
		let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		if consistency{
			let st_last:u16=self.st.len() as u16;
			//let st_last:u16=self.st.len() as u16;
			let sc_last:u16=self.sc.len() as u16;
			for i in 0..self.sc.len(){				
				for j in self.sc[i].len()-1 as usize..(st_last) as usize{
					self.sc[i].push(st_last);
				}
			}
			//let lastsc:Vec<u16>=vec![st_last;(st_last+1) as usize];
			let mut lastsc:Vec<u16>=vec![st_last];
			for i in 0..self.st.len(){
				lastsc.push(i as u16)
			}
			//self.sc.push(lastsc);
			for i in 0..self.st.len(){				
				for j in self.st[i].len()-1 as usize..(sc_last-1) as usize{
					self.st[i].push(sc_last);
				}
			}
			//let lastst:Vec<u16>=vec![sc_last;sc_last as usize];
			let mut lastst:Vec<u16>=vec![sc_last];
			for i in 0..self.sc.len(){
				lastst.push(i as u16);
			}
		
			self.sc.push(lastsc);
			self.st.push(lastst);
		}
		else{
			// ???????
		}
		self.qu_sc.push(1);
		self.qu_st.push(1);
	}
	
	
	// behind the main pref lists, the dummy is also added!
	fn complete_lists_123_add1(&mut self){
		let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		if consistency{
			let st_last:u16=self.st.len() as u16;
			//let st_last:u16=self.st.len() as u16;
			let sc_last:u16=self.sc.len() as u16;
			for i in 0..self.sc.len(){				
				for j in self.sc[i].len()-1 as usize..(st_last) as usize{
					self.sc[i].push(st_last);
				}
			}
			//let lastsc:Vec<u16>=vec![st_last;(st_last+1) as usize];
			let mut lastsc:Vec<u16>=vec![st_last];
			for i in 0..self.st.len(){
				lastsc.push(i as u16)
			}
			//self.sc.push(lastsc);
			for i in 0..self.st.len(){				
				//for j in self.st[i].len()-1 as usize..(sc_last-1) as usize{
				for j in self.st[i].len()-1 as usize..(sc_last) as usize{
					self.st[i].push(sc_last);
				}
			}
			//let lastst:Vec<u16>=vec![sc_last;sc_last as usize];
			let mut lastst:Vec<u16>=vec![sc_last];
			for i in 0..self.sc.len(){
				lastst.push(i as u16);
			}
		
			self.sc.push(lastsc);
			self.st.push(lastst);
		}
		else{
			// ???????
		}
		self.qu_sc.push(1);
		self.qu_st.push(1);
	}
	fn get_idx_from_match(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,wtmp:&Vec<Vec<usize>>)->Vec<Vec<usize>>{
		//let mtmp:Vec<Vec<usize>>=Self::traverse_wtmp(&wtmp,m.len());
		//println!("GET IDX FROM MATCH\nm:{:?}\nw:{:?}\nwtmp:{:?}",m,w,wtmp);
		let mtmp:Vec<Vec<usize>>=wtmp.clone();
		let mut idx:Vec<Vec<usize>>=vec![vec![];m.len()];
		for i in 0..mtmp.len(){
			/*
			let mut highest_pos:usize=mtmp[i][0];
			if mtmp[i].len()>1{
				for j in 0..mtmp[i].len(){
					let wij:usize=mtmp[i][j];
					let wpos:usize=m[i].iter().position(|x| *x==wij).unwrap();
					if wpos>highest_pos{
						highest_pos=wpos;
					}
				}
			}
			idx[i]=highest_pos;
			*/
			for j in 0..mtmp[i].len(){
				let wij:usize=mtmp[i][j];
				let wpos:usize=m[i].iter().position(|x| *x==wij).unwrap();
				idx[i].push(wpos);
			}
			idx[i].sort();
		}
		for i in 0..idx.len(){
			let idx_len:usize=idx[i].len();
			let last:usize=idx[i][idx_len-1];
			idx[i].push(last+1);
		}
		idx
	}
	fn remove_idx_pos_iter2(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,idx_:&Vec<Vec<usize>>,
		idx_coord:(usize,usize),wtmp_:&Vec<Vec<usize>>,
		last_pess:&Vec<usize>)->Option<(Vec<Vec<usize>>,Vec<usize>,Vec<Vec<usize>>)>{
			
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		// PERFORMANCE???
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		//println!("REMOVE IDX POS ITER\nm:{:?}\nidx:{:?}\niter_pos:{}\nwtmp:{:?}\n",m,idx,iter_pos,wtmp);
		//println!("REMOVE IDX POS ITER\nm:{:?}\nidx:{:?}\nidx_coord:{:?}\nwtmp:{:?}\n",m,idx,idx_coord,wtmp);
		
		// get ipos & jpos (ipos -> replace pos, jpos -> replace 0)
		let (ipos,jpos):(usize,usize)=idx_coord;
		let idx_i_len:usize=idx[ipos].len();
		if idx[ipos][idx_i_len-1]<last_pess[ipos]{
			let mut mtmp:Vec<usize>=vec![ipos];
			let idx_len:usize=idx[ipos].len();
			let wij:usize=m[ipos][idx[ipos][jpos]];

			// BE VERY VERY CAREFUL !!!!!!!!!!!!
			// BE VERY VERY CAREFUL !!!!!!!!!!!!
			// BE VERY VERY CAREFUL !!!!!!!!!!!!
			
			//println!("ipos:{}, jpos:{}, wij:{}",ipos,jpos,wij);
			
			//WIJ OR POS OR SOMETHING ELSE ????????????
			// SEE THE DIFFERENCE BETWEEN WTMP (ALREADY TRANSLATED) AND WTMP THAT IS GIVEN INTO THE FUNCTION !!!!!!!

			let mpos:usize=wtmp[ipos].iter().position(|x| *x==wij).unwrap();

			wtmp[ipos].remove(mpos);

			idx[ipos].remove(jpos);
			wtmp=Self::traverse_wtmp(&wtmp,w.len());
			//println!("RESULT FROM INCREMENT IDX POS:\nidx:{:?}\nmtmp:{:?}\nwtmp:{:?}",idx,mtmp,wtmp);
			Some((idx,mtmp,wtmp))
		}
		else{
			None
		}
	}
	fn remove_idx_pos_iter(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,idx_:&Vec<Vec<usize>>,idx_coord:(usize,usize),wtmp_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<usize>,Vec<Vec<usize>>){
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		// PERFORMANCE???
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		//println!("REMOVE IDX POS ITER\nm:{:?}\nidx:{:?}\niter_pos:{}\nwtmp:{:?}\n",m,idx,iter_pos,wtmp);
		//println!("REMOVE IDX POS ITER\nm:{:?}\nidx:{:?}\nidx_coord:{:?}\nwtmp:{:?}\n",m,idx,idx_coord,wtmp);
		
		// get ipos & jpos (ipos -> replace pos, jpos -> replace 0)
		let (ipos,jpos):(usize,usize)=idx_coord;
		let mut mtmp:Vec<usize>=vec![ipos];
		//mtmp.remove(pos);
		//let idx_len:usize=idx[pos].len();
		let idx_len:usize=idx[ipos].len();
		//let wij:usize=m[pos][idx[pos][idx_len-1]];
		//let wij:usize=m[pos][idx[pos][0]];
		let wij:usize=m[ipos][idx[ipos][jpos]];
		/*
		for i in 0..wtmp.len(){
			if wtmp[i].contains(&wij)
		}
		*/
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		
		//println!("ipos:{}, jpos:{}, wij:{}",ipos,jpos,wij);
		
		//WIJ OR POS OR SOMETHING ELSE ????????????
		// SEE THE DIFFERENCE BETWEEN WTMP (ALREADY TRANSLATED) AND WTMP THAT IS GIVEN INTO THE FUNCTION !!!!!!!
		
		//let mpos:usize=wtmp[wij].iter().position(|x| *x==pos).unwrap();
		//let mpos:usize=wtmp[pos].iter().position(|x| *x==wij).unwrap();
		let mpos:usize=wtmp[ipos].iter().position(|x| *x==wij).unwrap();
		//wtmp[wij].remove(mpos);
		//wtmp[pos].remove(mpos);
		wtmp[ipos].remove(mpos);
		//idx[pos][idx_len-1]+=1;
		//idx[pos].remove(0);
		idx[ipos].remove(jpos);
		wtmp=Self::traverse_wtmp(&wtmp,w.len());
		//println!("RESULT FROM INCREMENT IDX POS:\nidx:{:?}\nmtmp:{:?}\nwtmp:{:?}",idx,mtmp,wtmp);
		(idx,mtmp,wtmp)
	}
	fn get_idx_len(idx:&Vec<Vec<usize>>)->Vec<(usize,usize)>{
		let mut vec_coord:Vec<(usize,usize)>=vec![];
		for i in 0..idx.len(){
			for j in 0..idx[i].len()-1{
				vec_coord.push((i,j));
			}
		}
		vec_coord
	}
	fn get_last_pess(m:&Vec<Vec<usize>>,idx:&Vec<Vec<usize>>,pess:&Vec<Vec<usize>>)->Vec<usize>{
		//let mut idx_before:Vec<Vec<usize>>=vec![];
		let mut last_pess:Vec<usize>=vec![];
		//println!("GET IDX BEFORE PESS: {:?}",pess);
		for i in 0..pess.len(){
			let mut last_pos:usize=0;
			for j in 0..pess[i].len(){
				let pos:usize=m[i].iter().position(|x| *x==pess[i][j]).unwrap();
				if pos>last_pos{
					last_pos=pos;
				}
			}
			last_pess.push(last_pos);
		}
		//println!("LAST PESS: {:?}",last_pess);
		//idx_before
		last_pess
	}
	fn check_match_within_pess(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>,pess:&Vec<Vec<usize>>)->bool{
		// ASSUMPTION: IDX ELEMENTS POSSIBLY NOT ORDERED, SO IT'S FAILSAFE!
		//let mut idx:Vec<Vec<usize>>=idx_.clone();
		let mut idx:Vec<Vec<usize>>=Self::get_idx_from_match(&m,&w,&match_);
		for i in 0..idx.len(){
			idx[i].sort();
		}
		let last_pess:Vec<usize>=Self::get_last_pess(&m,&idx,&pess);
		//println!("CHECK MATCH WITHIN PESS!\nm:{:?}\nw:{:?}\nmatch_:{:?}\npess:{:?}\nlast_pess:{:?}\nidx:{:?}",m,w,match_,pess,last_pess,idx);
		for i in 0..idx.len(){
			let idxi_len:usize=idx[i].len();
			// -2 is right because idx has additional values that do not belong to the match!
			if idx[i][idxi_len-2]>last_pess[i]{
				//println!("FALSE at i:{}, idx[{}]:{:?},last_pess[{}]:{}",i,i,idx[i],i,last_pess[i]);
				return false;
			}
		}
		true
	}
	// BE CAREFUL! WTMP NOT NEEDED (CAN BE CALCULATED BUT FOR PERFORMANCE REASON WE PUT IT AS A PARAMETER INTO THE FUNCTION!
	fn increment_idx_pos(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,idx_:&Vec<Vec<usize>>,pos:usize,wtmp_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<usize>,Vec<Vec<usize>>){
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		// PERFORMANCE???
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		println!("INCREMENT IDX POS\nm:{:?}\nidx:{:?}\npos:{}\nwtmp:{:?}\n",m,idx,pos,wtmp);
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(idx.len());
		mtmp.remove(pos);
		let idx_len:usize=idx[pos].len();
		let wij:usize=m[pos][idx[pos][idx_len-1]];
		//let wij:usize=m[pos][idx[pos][0]];
		
		/*
		for i in 0..wtmp.len(){
			if wtmp[i].contains(&wij)
		}
		*/
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		
		//WIJ OR POS OR SOMETHING ELSE ????????????
		// SEE THE DIFFERENCE BETWEEN WTMP (ALREADY TRANSLATED) AND WTMP THAT IS GIVEN INTO THE FUNCTION !!!!!!!
		
		//let mpos:usize=wtmp[wij].iter().position(|x| *x==pos).unwrap();
		let mpos:usize=wtmp[pos].iter().position(|x| *x==wij).unwrap();
		//wtmp[wij].remove(mpos);
		wtmp[pos].remove(mpos);
		//idx[pos][idx_len-1]+=1;
		wtmp=Self::traverse_wtmp(&wtmp,w.len());
		println!("RESULT FROM INCREMENT IDX POS:\nidx:{:?}\nmtmp:{:?}\nwtmp:{:?}",idx,mtmp,wtmp);
		(idx,mtmp,wtmp)
	}
	fn increment_idx_pos2(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,idx_:&Vec<Vec<usize>>,pos:usize,wtmp_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<usize>,Vec<Vec<usize>>){
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		// PERFORMANCE???
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		//println!("INCREMENT IDX POS\nm:{:?}\nidx:{:?}\npos:{}\nwtmp:{:?}\n",m,idx,pos,wtmp);
		//let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(idx.len());
		let mut mtmp:Vec<usize>=vec![pos];
		//mtmp.remove(pos);
		let idx_len:usize=idx[pos].len();
		//let wij:usize=m[pos][idx[pos][idx_len-1]];
		let wij:usize=m[pos][idx[pos][0]];
		/*
		for i in 0..wtmp.len(){
			if wtmp[i].contains(&wij)
		}
		*/
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		// BE VERY VERY CAREFUL !!!!!!!!!!!!
		
		//WIJ OR POS OR SOMETHING ELSE ????????????
		// SEE THE DIFFERENCE BETWEEN WTMP (ALREADY TRANSLATED) AND WTMP THAT IS GIVEN INTO THE FUNCTION !!!!!!!
		
		//let mpos:usize=wtmp[wij].iter().position(|x| *x==pos).unwrap();
		let mpos:usize=wtmp[pos].iter().position(|x| *x==wij).unwrap();
		//wtmp[wij].remove(mpos);
		wtmp[pos].remove(mpos);
		//idx[pos][idx_len-1]+=1;
		idx[pos].remove(0);
		wtmp=Self::traverse_wtmp(&wtmp,w.len());
		//println!("RESULT FROM INCREMENT IDX POS:\nidx:{:?}\nmtmp:{:?}\nwtmp:{:?}",idx,mtmp,wtmp);
		(idx,mtmp,wtmp)
	}
	// remove the first idx element at pos!
	fn remove_first_idx(idx_:&Vec<Vec<usize>>,pos:usize)->Vec<Vec<usize>>{
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		idx[pos].remove(0);
		idx
	}
	fn shorten_lists(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		//let mut m:Vec<Vec<usize>>=self.sc.clone();
		let mut m:Vec<Vec<usize>>=m_.clone();
		//let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut w:Vec<Vec<usize>>=w_.clone();
		println!("SHORTEN LISTS - match_:{:?}",match_);
		for i in 0..match_.len(){
			let match_len:usize=match_.len();
			//let match_len:usize=match_[i].len();
			let last:usize=match_[i][match_len-1];
			let last_pos:usize=m[i].iter().position(|x| *x==last).unwrap();
			// last or last_pos ??????????
			//for j in 0..last{
			for j in 0..last_pos{
				let wij:usize=m[i][j];
				//if !match_[i].contains(m[i][j]){
				if !match_[i].contains(&wij){
					let pos_wij_in_w:usize=w[wij].iter().position(|x| *x==i).unwrap();
					w[wij].remove(pos_wij_in_w);
					m[i].remove(j);
				}
			}
		}
		(m,w)
	}
	fn shorten_lists_wrapper(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		//let mut m:Vec<Vec<usize>>=m_.clone();
		//let mut w:Vec<Vec<usize>>=w_.clone();
		// detect what length match has. does it belong to m or to w?
		let mlen:usize=m_.len();
		let wlen:usize=w_.len();
		let mut match_short:Vec<Vec<usize>>=match_.clone();
		if match_.len()==wlen{
			//println!("SHORTEN LISTS WRAPPER - WLEN to MLEN!");
			match_short=Self::traverse_wtmp(&match_,mlen);
		}
		//println!("MATCH SHORT, SHORTEN LISTS WRAPPER:\nmatch_short:{:?}",match_short);
		//println!("m:\n{:?}\n\nw:\n{:?}",m_,w_);
		let (mut m, mut w):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists3(&m_,&w_,&match_short,"first".to_string());
		//println!("AFTER SHORTEN LISTS 3 IN WRAPPER");
		//println!("m:\n{:?}\n\nw:\n{:?}",m,w);
		match_short=Self::traverse_wtmp(&match_,wlen);
		//println!("NEW MATCH_SHORT:{:?}",match_short);
		let (mut w, mut m):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists3(&w,&m,&match_short,"last".to_string());
		//println!("AFTER WLEN MATCH_SHORT!");
		(m,w)
	}
	fn shorten_lists2(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		//let mut m:Vec<Vec<usize>>=self.sc.clone();
		let mut m:Vec<Vec<usize>>=m_.clone();
		//let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut w:Vec<Vec<usize>>=w_.clone();
		println!("SHORTEN LISTS 2 - match_:{:?}",match_);
		println!("m:{:?}\nw:{:?}\n",m,w);
		for i in 0..match_.len(){
			// assumption: all players will be matched!
			println!("i:{}, match_[{}][0]:{}, m[{}]: {:?}\n",i,i,match_[i][0],i,m[i]);
			let mut first_pos:usize=m[i].iter().position(|x| *x==match_[i][0]).unwrap();
			println!("first_pos:{}",first_pos);
			if match_[i].len()>1{
				for j in 1..match_[i].len(){
					println!("first_pos:{}, i:{}, j:{}",first_pos,i,j);
					let wpos:usize=m[i].iter().position(|x| *x==match_[i][j]).unwrap();
					if wpos<first_pos{
						first_pos=wpos;
					}
				}
			}
			println!("first_pos:{}",first_pos);
			let mut delvec:Vec<usize>=vec![];
			for j in 0..first_pos{
				delvec.insert(0,j);
				let wij=m[i][j];
				let delpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
				w[wij].remove(delpos);
			}
			for j in 0..delvec.len(){
				m[i].remove(delvec[j]);
			}
		}
		(m,w)
	}	
	
	fn shorten_lists3(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>,first_last:String)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		//let mut m:Vec<Vec<usize>>=self.sc.clone();
		let mut m:Vec<Vec<usize>>=m_.clone();
		//let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut w:Vec<Vec<usize>>=w_.clone();
		//println!("SHORTEN LISTS 3 - match_:{:?}",match_);
		//println!("m:{:?}\nw:{:?}\n",m,w);
		println!("SHORTEN LISTS 3 - {:?}\nm:{:?}\nw:{:?}\nmatch_:{:?}",first_last,m,w,match_);
		for i in 0..match_.len(){
			// assumption: all players will be matched!
			//println!("i:{}, match_[{}][0]:{}, m[{}]: {:?}\n",i,i,match_[i][0],i,m[i]);
			let mut distinct_pos:usize=m[i].iter().position(|x| *x==match_[i][0]).unwrap();
			//println!("first_pos:{}",distinct_pos);
			if match_[i].len()>1{
				for j in 1..match_[i].len(){
					println!("distinct_pos:{}, i:{}, j:{}",distinct_pos,i,j);
					let wpos:usize=m[i].iter().position(|x| *x==match_[i][j]).unwrap();
					if first_last=="first".to_string(){
						if wpos<distinct_pos{
							distinct_pos=wpos;
						}
					}
					else{
						if first_last=="last".to_string(){
							if wpos>distinct_pos{
								distinct_pos=wpos;
							}
						}
						else{
							println!("VERY BIG PROBLEM IN SHORTEN_LISTS3!");
						}
					}
				}
			}
			//println!("first_pos:{}, first_last:{:?}",distinct_pos,first_last);
			let mut delvec:Vec<usize>=vec![];
			if first_last=="first".to_string(){
				for j in 0..distinct_pos{
					delvec.insert(0,j);
					let wij=m[i][j];
					let delpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
					w[wij].remove(delpos);
				}
			}
			else{
				if first_last=="last".to_string(){
					if m[i].len()>distinct_pos+1{
						//for j in 0..distinct_pos{
						for j in distinct_pos+1..m[i].len(){
							delvec.insert(0,j);
							let wij=m[i][j];
							//println!("w[{}][{}]:{}",i,j,wij);
							let delpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
							w[wij].remove(delpos);
						}					
					}
				}
				else{
					println!("VERY BIG PROBLEM IN SHORTEN_LISTS3, SECOND");
				}
			}
			for j in 0..delvec.len(){
				m[i].remove(delvec[j]);
			}
			//println!("m[{}]:{:?}",i,m[i]);
		}

		(m,w)
	}	










	fn shorten_lists4(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>,first_last:String)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		//let mut m:Vec<Vec<usize>>=self.sc.clone();
		let mut m:Vec<Vec<usize>>=m_.clone();
		//let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut w:Vec<Vec<usize>>=w_.clone();
		//println!("SHORTEN LISTS 3 - match_:{:?}",match_);
		//println!("m:{:?}\nw:{:?}\n",m,w);
		println!("SHORTEN LISTS 3 - {:?}\nm:{:?}\nw:{:?}\nmatch_:{:?}",first_last,m,w,match_);
		for i in 0..match_.len(){
			// assumption: all players will be matched!
			//println!("i:{}, match_[{}][0]:{}, m[{}]: {:?}\n",i,i,match_[i][0],i,m[i]);
			let mut distinct_pos:usize=m[i].iter().position(|x| *x==match_[i][0]).unwrap();
			//println!("first_pos:{}",distinct_pos);
			if match_[i].len()>1{
				for j in 1..match_[i].len(){
					println!("distinct_pos:{}, i:{}, j:{}",distinct_pos,i,j);
					let wpos:usize=m[i].iter().position(|x| *x==match_[i][j]).unwrap();
					if first_last=="first".to_string(){
						if wpos<distinct_pos{
							distinct_pos=wpos;
						}
					}
					else{
						if first_last=="last".to_string(){
							if wpos>distinct_pos{
								distinct_pos=wpos;
							}
						}
						else{
							println!("VERY BIG PROBLEM IN SHORTEN_LISTS3!");
						}
					}
				}
			}
			//println!("first_pos:{}, first_last:{:?}",distinct_pos,first_last);
			
			let mut delvec:Vec<usize>=vec![];
			// IT'S THE OTHER WAY ROUND!
			/*
			if first_last=="first".to_string(){
				for j in 0..distinct_pos{
					delvec.insert(0,j);
					let wij=m[i][j];
					let delpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
					w[wij].remove(delpos);
				}
			}
			else{
				if first_last=="last".to_string(){
					if m[i].len()>distinct_pos+1{
						//for j in 0..distinct_pos{
						for j in distinct_pos+1..m[i].len(){
							delvec.insert(0,j);
							let wij=m[i][j];
							//println!("w[{}][{}]:{}",i,j,wij);
							let delpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
							w[wij].remove(delpos);
						}					
					}
				}
				else{
					println!("VERY BIG PROBLEM IN SHORTEN_LISTS3, SECOND");
				}
			}
			*/
			
			for j in 0..delvec.len(){
				m[i].remove(delvec[j]);
			}
			//println!("m[{}]:{:?}",i,m[i]);
		}

		(m,w)
	}	
	// SHORTLY LATER: COMPLETE ALL ELEM THAT ARE NOT IN THE PREF LIST OF THE OTHER PART!
	fn shorten_lists7(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>,match_:&Vec<Vec<usize>>)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		let mut m:Vec<Vec<usize>>=m_.clone();
		let mut w:Vec<Vec<usize>>=w_.clone();
		let mmatch:Vec<Vec<usize>>=match_.clone();
		let wmatch:Vec<Vec<usize>>=Self::traverse_wtmp(&mmatch,w.len());
		println!("SHORTEN LISTS 7:\nm:{:?}\nw:{:?}\nmmatch:{:?}\nwmatch:{:?}",m,w,mmatch,wmatch);
		// processing mmatch from 0 to last elem in mmatch!
		println!("### mmatch: {:?}",mmatch);
		for i in 0..mmatch.len(){
			let mut last_pos:usize=0;
			for j in 0..mmatch[i].len(){
				let wij:usize=mmatch[i][j];
				println!("i:{}, j:{}, last_pos:{}, wij:{}",i,j,last_pos,wij);
				let pos:usize=m[i].iter().position(|x| *x==wij).unwrap();
				if last_pos<pos{
					last_pos=pos;
				}
			}
			let mut delvec:Vec<usize>=vec![];
			for j in 0..last_pos{
				let wij:usize=m[i][j];
				if !mmatch[i].contains(&wij){
					let wpos:usize=w[wij].iter().position(|x| *x==i).unwrap();
					w[wij].remove(wpos);
					delvec.insert(0,j);
				}
			}
			println!("mmatch[{}]: last_pos:{}, delvec:{:?}",i,last_pos,delvec);
			for j in 0..delvec.len(){
				m[i].remove(delvec[j]);
			}
		}
		// processing wmatch from first elem in wmatch to end of pref-list!
		println!("### wmatch: {:?}",wmatch);
		for i in 0..wmatch.len(){
			//let mut first_pos:usize=w[i].len()-1;
			let mut first_pos:usize=0;
			for j in 0..wmatch[i].len(){
				let mij:usize=wmatch[i][j];
				let pos:usize=w[i].iter().position(|x| *x==mij).unwrap();
				//if first_pos>pos{
				if first_pos<pos{
					first_pos=pos;
				}
			}
			let mut delvec:Vec<usize>=vec![];
			if w[i].len()>first_pos{
				for j in first_pos+1..w[i].len(){
					let mij:usize=w[i][j];
					if !wmatch[i].contains(&mij){
						let mpos_option:Option<usize>=m[mij].iter().position(|x| *x==i);
						if mpos_option.is_some(){
							let mpos:usize=mpos_option.unwrap();
							m[mij].remove(mpos);
						}
						delvec.insert(0,j);
					}
				}
			}
			println!("wmatch[{}]: first_pos:{}, delvec:{:?}",i,first_pos,delvec);
			for j in 0..delvec.len(){
				w[i].remove(delvec[j]);
			}
		}
		println!("RESULT SHORTEN LISTS 7:\nm:{:?}\nw:{:?}",m,w);
		(m,w)
	}
	fn transform_tmp_matrix(a:&Vec<Vec<usize>>)->Vec<Vec<usize>>{
		let mut b:Vec<Vec<usize>>=vec![];
		let mut len_a:usize=0;
		for i in 0..a.len(){
			for j in 0..a[i].len(){
				if len_a<a[i][j]{
					len_a=a[i][j];
				}
			}
		}
		for i in 0..len_a+1{
			b.push(vec![]);
		}
		for i in 0..a.len(){
			for j in 0..a[i].len(){
				b[a[i][j]].push(i);
			}
		}
		b
	}
	fn toggle_node(shortlists:(&Vec<Vec<usize>>,&Vec<Vec<usize>>),toggle_idx:usize)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		let mut m:Vec<Vec<usize>>=shortlists.0.clone();
		let mut w:Vec<Vec<usize>>=shortlists.1.clone();
		let wi0:usize=m[toggle_idx][0];
		let pos_m:usize=w[wi0].iter().position(|x| *x==toggle_idx).unwrap();
		m[toggle_idx].remove(0);
		w[wi0].remove(pos_m);
		(m,w)
	}
	fn check_list_consistency(&self)->(bool,usize,usize){
		let mut sc_longest:usize=0; // horizontal length
		let mut st_longest:usize=0; // horizontal length
		for i in 0..self.sc.len(){
			if self.sc[i].len()>sc_longest{
				sc_longest=self.sc[i].len();
			}
		}
		for i in 0..self.st.len(){
			if self.st[i].len()>st_longest{
				st_longest=self.st[i].len();
			}
		}
		let mut is_consistent:bool=false;
		if sc_longest==self.st.len() && st_longest==self.sc.len(){
			is_consistent=true;
		}
		(is_consistent,sc_longest,st_longest)
	}
	
	fn create_full_mtmp_vec(len:usize)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..len{
			mtmp.push(i);
		}
		mtmp
	}
	fn create_full_mtmp(&self)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..self.sc.len(){
			mtmp.push(i);
			
		}
		mtmp
	}
	fn create_empty_wtmp_vec(len:usize)->Vec<Vec<usize>>{
		let mut wtmp:Vec<Vec<usize>>=vec![vec![];len];
		wtmp
	}
	fn create_empty_wtmp(&self)->Vec<Vec<Option<usize>>>{
		let mut tmp:Vec<Vec<Option<usize>>>=vec![];
		for i in 0..self.st.len(){
			//tmp.push(None);
			// using the quota???
			let mut tmp_i:Vec<Option<usize>>=vec![None;self.qu_st[i]];
			tmp.push(tmp_i);
			//for j in 0..self.st[i].len(){
			/*
			for j in 0..self.qu_st[i]{
				
			}
			*/
		}
		tmp
	}
	fn check_consistency_mtmp_wtmp(mtmp:&Vec<Vec<usize>>)->bool{
		let mut highest:usize=0;
		//println!("CHECK CONSISTENCY MTMP:{:?}",mtmp);
		for i in 0..mtmp.len(){
			if mtmp[i].len()==0{
				//println!("return false first, mtmp:{:?}",mtmp);
				return false;
			}
			for j in 0..mtmp[i].len(){
				if mtmp[i][j]>highest{
					highest=mtmp[i][j];
				}
			}
		}
		for i in 0..highest{
			let mut within:bool=false;
			for j in 0..mtmp.len(){
				if mtmp[j].contains(&i){
					within=true;
				}
			}
			if !within{
				//println!("return false second, mtmp:{:?}",mtmp);
				return false;
			}
		}
		//println!("RETURN TRUE, MTMP:{:?}",mtmp);
		true
	}
	//fn traverse_wtmp(wtmp_:&Vec<Vec<Option<usize>>>)->Vec<Vec<usize>>{
	fn traverse_wtmp(wtmp_:&Vec<Vec<usize>>,sc_len:usize)->Vec<Vec<usize>>{
		let mut mtmp:Vec<Vec<usize>>=vec![vec![];sc_len];
		let mut wtmp:Vec<Vec<usize>>=vec![];
		//println!("traverse wtmp -> wtmp:{:?}",wtmp_);
		/*
		for i in 0..wtmp_.len(){
			wtmp.push(*wtmp_[i].as_ref().expect(""));
		}
		*/
		// Adding elements of wtmp to mtmp for displaying mtmp correctly!
		// BE VERY VERY VERY CAREFUL!
		for i in 0..wtmp_.len(){
			for j in 0..wtmp_[i].len(){
				//let pos:usize=wtmp[i].iter().position(|x| *x==j).unwrap();
				let m:usize=wtmp_[i][j];
				mtmp[m].push(i);
				
			}
			//let pos:usize=wtmp.iter().position(|x| *x==i).unwrap();
			//mtmp.push(pos);
		}
		//println!("TRAVERSE WTMP:{:?}",mtmp);
		mtmp
		
	}
	// for sc!
	fn idx_check(idx:&Vec<Vec<usize>>,qu:&Vec<usize>)->bool{
		let mut ok:bool=true;
		for i in 0..idx.len(){
			/*
			for j in 0..idx[i].len(){
				if idx[i][j]>qu[i]{
					return false;
				}
			}
			*/
			if idx[i].len()>qu[i]{
				return false;
			}
		}
		true
	}
	// iterating until capacity of proposers are full!
	fn get_posets_iter7(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<Vec<usize>>,last_pess:&Vec<usize>,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		//let mut cur:usize=cur_;
		let mut cur:usize=mtmp[0];
		//println!("INPUT ITER2 DATA\ncur:{}, wtmp:{:?}, mtmp:{:?}, idx:{:?}",cur,wtmp,mtmp,idx);
		//println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		// PREF OVERFLOW
		// ADDED M[CUR].LEN() <= IDX... AS ADDITIONAL CONDITION!
		if idx[cur][idx_cur_len-1]>=w.len() || idx[cur].len()-1>=qu_sc[cur] || m[cur].len()<=idx[cur][idx_cur_len-1] || idx[cur][idx_cur_len-1]>last_pess[cur]{
			//println!("PROPOSALS AT THE END OR PROPOSER-QUOTA FULL!");
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				//cur=mtmp[0];
				rmatch=Self::get_posets_iter7(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,&last_pess);
				//println!("RMATCH EXIT 1: {:?}",rmatch);
				//return rmatch;
			}
			else{
				// FINISHED! MATCH!
				rmatch=Self::traverse_wtmp(&wtmp,m.len());
				//println!("RMATCH EXIT 2: {:?}",rmatch);
				//return rmatch;
			}
		}
		else{			
			//println!("cur:{}, idx:{:?}, idx_cur_len:{}",cur,idx,idx_cur_len);
			//println!("m:{:?}",m);
			let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
					
			if !w[w_mi].contains(&cur){
				let last_idx:usize=idx[cur][idx_cur_len-1];
				idx[cur].push(last_idx+1);
				rmatch=Self::get_posets_iter7(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,&last_pess);
				//println!("RMATCH EXIT 3: {:?}",rmatch);
				//return rmatch;
			}
			
			else{
				// STILL IN CAPACITY OF ACCEPTORS
				if wtmp[w_mi].len()<qu_st[w_mi]{
					//idx[cur]+=1;
					wtmp[w_mi].push(cur);
					let last_idx:usize=idx[cur][idx_cur_len-1];
					idx[cur].push(last_idx+1);
					rmatch=Self::get_posets_iter7(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,&last_pess);
					//println!("RMATCH EXIT 4: {:?}",rmatch);
					//return rmatch;
				}
				// CAPACITY OVERFLOW OF ACCEPTORS
				else{
					//println!("QUOTA OF ACCEPTOR IS FULL -> SELECT LAST ONE!");
					if wtmp[w_mi].len()>qu_st[w_mi]{
						println!("BIG PROBLEM! QUOTA OVERFLOW!");
					}
					let mut go_next:bool=true;
					let mut pos_least_preferred:usize=0;
					let mut m_least_preferred:usize=wtmp[w_mi][0];
					for i in 0..wtmp[w_mi].len(){
						let wpartner:usize=wtmp[w_mi][i];
						let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
						let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
						//if pos_wprt>pos_cur{
						if pos_cur<pos_wprt{
							go_next=false;
							if pos_least_preferred<pos_wprt{
								pos_least_preferred=pos_wprt;
								m_least_preferred=wpartner;
							}
						}
					}
					let wpartner:usize=m_least_preferred;
					let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
					let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
					//if pos_wprt<pos_cur{
					
					// STILL UNPREFERRED BY ACCEPTOR
					if go_next{
						let idx_cur_len:usize=idx[cur].len();
						idx[cur][idx_cur_len-1]+=1;
						//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
						rmatch=Self::get_posets_iter7(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,&last_pess);
						//println!("RMATCH EXIT 5: {:?}",rmatch);
						//return rmatch;
					}
					// ACCEPTOR KICKS LAST ONE OUT AND TAKES THE CURRENT PROPOSER!
					else{
						if pos_wprt==pos_cur{
							println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
						}						
							// COPY !!!!!!!
							// BE CAREFUL !									
							let idx_cur_len:usize=idx[cur].len();
							let last_idx:usize=idx[cur][idx_cur_len-1];
							idx[cur].push(last_idx+1);
							// REARRANGE WTMP !!!!!!!
							let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
							wtmp[w_mi].remove(wpos_cur);
							//println!("BEFORE PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
							wtmp[w_mi].push(cur);
							//println!("AFTER  PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
						// BE VEY CAREFUL !!!!!!!
							//let idx_cur_len:usize=idx[cur].len();
							let delpos_idx_wprt:usize=m[wpartner].iter().position(|x| *x==w_mi).unwrap();
							if !idx[wpartner].contains(&delpos_idx_wprt){
								println!("BIG PROBLEM! NOT IN IDX[WPARTNER]!!!!");
								println!("idx:{:?}, wpartner:{}, delpos_idx_wprt:{}, w_mi:{}",idx,wpartner,delpos_idx_wprt,w_mi);
							}
							
							// ####### IMPORTANT ####### 
							let del_idx_wprt_final:usize=idx[wpartner].iter().position(|x| *x==delpos_idx_wprt).unwrap();
							idx[wpartner].remove(del_idx_wprt_final);
							let idx_wprt_len:usize=idx[wpartner].len();
							mtmp.insert(0,wpartner);													
						rmatch=Self::get_posets_iter7(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,&last_pess);
					}
				}
			}
		}
		//println!("####### END REACHED !!!!!!! #######\n{:?}",rmatch);
		if !Self::check_consistency_mtmp_wtmp(&rmatch){
			return vec![];
		}
		//println!("RESULT POSETS:{:?}",rmatch);
		rmatch
	}
	// iterating until capacity of proposers are full!
	fn get_posets_iter4(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<Vec<usize>>,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		//let mut cur:usize=cur_;
		let mut cur:usize=mtmp[0];
		//println!("INPUT ITER2 DATA\ncur:{}, wtmp:{:?}, mtmp:{:?}, idx:{:?}",cur,wtmp,mtmp,idx);
		//println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		// PREF OVERFLOW
		// ADDED M[CUR].LEN() <= IDX... AS ADDITIONAL CONDITION!
		if idx[cur][idx_cur_len-1]>=w.len() || idx[cur].len()-1>=qu_sc[cur] || m[cur].len()<=idx[cur][idx_cur_len-1]{
			//println!("PROPOSALS AT THE END OR PROPOSER-QUOTA FULL!");
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				//cur=mtmp[0];
				rmatch=Self::get_posets_iter4(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 1: {:?}",rmatch);
				//return rmatch;
			}
			else{
				// FINISHED! MATCH!
				rmatch=Self::traverse_wtmp(&wtmp,m.len());
				//println!("RMATCH EXIT 2: {:?}",rmatch);
				//return rmatch;
			}
		}
		else{			
			//println!("cur:{}, idx:{:?}, idx_cur_len:{}",cur,idx,idx_cur_len);
			//println!("m:{:?}",m);
			let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
					
			if !w[w_mi].contains(&cur){
				let last_idx:usize=idx[cur][idx_cur_len-1];
				idx[cur].push(last_idx+1);
				rmatch=Self::get_posets_iter4(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 3: {:?}",rmatch);
				//return rmatch;
			}
			
			else{
				// STILL IN CAPACITY OF ACCEPTORS
				if wtmp[w_mi].len()<qu_st[w_mi]{
					//idx[cur]+=1;
					wtmp[w_mi].push(cur);
					let last_idx:usize=idx[cur][idx_cur_len-1];
					idx[cur].push(last_idx+1);
					rmatch=Self::get_posets_iter4(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
					//println!("RMATCH EXIT 4: {:?}",rmatch);
					//return rmatch;
				}
				// CAPACITY OVERFLOW OF ACCEPTORS
				else{
					//println!("QUOTA OF ACCEPTOR IS FULL -> SELECT LAST ONE!");
					if wtmp[w_mi].len()>qu_st[w_mi]{
						println!("BIG PROBLEM! QUOTA OVERFLOW!");
					}
					let mut go_next:bool=true;
					let mut pos_least_preferred:usize=0;
					let mut m_least_preferred:usize=wtmp[w_mi][0];
					for i in 0..wtmp[w_mi].len(){
						let wpartner:usize=wtmp[w_mi][i];
						let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
						let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
						//if pos_wprt>pos_cur{
						if pos_cur<pos_wprt{
							go_next=false;
							if pos_least_preferred<pos_wprt{
								pos_least_preferred=pos_wprt;
								m_least_preferred=wpartner;
							}
						}
					}
					//let wpartner:usize=wtmp[w_mi].expect("");
					let wpartner:usize=m_least_preferred;
					let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
					let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
					//if pos_wprt<pos_cur{
					
					// STILL UNPREFERRED BY ACCEPTOR
					if go_next{
						let idx_cur_len:usize=idx[cur].len();
						idx[cur][idx_cur_len-1]+=1;
						//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
						rmatch=Self::get_posets_iter4(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 5: {:?}",rmatch);
						//return rmatch;
					}
					// ACCEPTOR KICKS LAST ONE OUT AND TAKES THE CURRENT PROPOSER!
					else{
						if pos_wprt==pos_cur{
							println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
						}						
							// COPY !!!!!!!
							// BE CAREFUL !									
							//println!("LAST wpartner in iter2:{}",wpartner);
							//mtmp.push(wpartner);
							//mtmp.insert(0,wpartner);
							//wtmp[w_mi]=Some(cur);
							let idx_cur_len:usize=idx[cur].len();
							let last_idx:usize=idx[cur][idx_cur_len-1];
							idx[cur].push(last_idx+1);
							// REARRANGE WTMP !!!!!!!
							let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
							wtmp[w_mi].remove(wpos_cur);
							//println!("BEFORE PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
							wtmp[w_mi].push(cur);
							//println!("AFTER  PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
						// BE VEY CAREFUL !!!!!!!
							//let idx_cur_len:usize=idx[cur].len();
							let delpos_idx_wprt:usize=m[wpartner].iter().position(|x| *x==w_mi).unwrap();
							if !idx[wpartner].contains(&delpos_idx_wprt){
								println!("BIG PROBLEM! NOT IN IDX[WPARTNER]!!!!");
								println!("idx:{:?}, wpartner:{}, delpos_idx_wprt:{}, w_mi:{}",idx,wpartner,delpos_idx_wprt,w_mi);
							}
							//else{
							//println!("cur(new):{}, IDX:{:?}, w_mi:{}, wpartner:{}, del_idxpos_wprt:{}",cur,idx,w_mi,wpartner,delpos_idx_wprt);
							
							//println!("REMOVE IDX_WPRT: {}",delpos_idx_wprt);
							
							// ####### IMPORTANT ####### 
							let del_idx_wprt_final:usize=idx[wpartner].iter().position(|x| *x==delpos_idx_wprt).unwrap();
							
							//idx[wpartner].remove(delpos_idx_wprt);
							
							//println!("REMOVE IDX! wpartner:{},del_idx_wprt_final:{},idx:{:?}",wpartner,del_idx_wprt_final,idx);
							
							
							// COMMENTED! IMPORTANT CASE!!!!!!!
							idx[wpartner].remove(del_idx_wprt_final);
							
							
							// TROUBLESHOOTING !!!!!!! JUST TESTING IT !!!!!!!
							/*
							if idx[wpartner].len()>1{
								idx[wpartner].remove(del_idx_wprt_final);
							}
							else{
								idx[wpartner][0]+=1;
							}
							*/
							//} //from else
							let idx_wprt_len:usize=idx[wpartner].len();
							// BE VERY CAREFUL!!! IF SUCCESS -> PUSH! IF REJECTED -> +=1 !!!!!!!
							//idx[wpartner].push(last_idx+1);
							//idx[wpartner][idx_wprt_len-1]+=1;// .push(last_idx+1);
							mtmp.insert(0,wpartner);

							//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);	
						//}														
						rmatch=Self::get_posets_iter4(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 6: {:?}",rmatch);
						//return rmatch;
					}
					//println!("LAST!");
				}
			}
		}
		//println!("####### END REACHED !!!!!!! #######\n{:?}",rmatch);
		if !Self::check_consistency_mtmp_wtmp(&rmatch){
			return vec![];
		}
		//println!("RESULT POSETS:{:?}",rmatch);
		rmatch
	}		
	// iterating until capacity of proposers are full!
	fn get_posets_iter3(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<Vec<usize>>,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		//let mut cur:usize=cur_;
		let mut cur:usize=mtmp[0];
		//println!("INPUT ITER2 DATA\ncur:{}, wtmp:{:?}, mtmp:{:?}, idx:{:?}",cur,wtmp,mtmp,idx);
		println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		// PREF OVERFLOW
		// ADDED M[CUR].LEN() <= IDX... AS ADDITIONAL CONDITION!
		if idx[cur][idx_cur_len-1]>=w.len() || idx[cur].len()-1>=qu_sc[cur] || m[cur].len()<=idx[cur][idx_cur_len-1]{
			//println!("PROPOSALS AT THE END OR PROPOSER-QUOTA FULL!");
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				//cur=mtmp[0];
				rmatch=Self::get_posets_iter3(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 1: {:?}",rmatch);
				//return rmatch;
			}
			else{
				// FINISHED! MATCH!
				rmatch=Self::traverse_wtmp(&wtmp,m.len());
				//println!("RMATCH EXIT 2: {:?}",rmatch);
				//return rmatch;
			}
		}
		else{			
			//println!("cur:{}, idx:{:?}, idx_cur_len:{}",cur,idx,idx_cur_len);
			//println!("m:{:?}",m);
			let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
					
			if !w[w_mi].contains(&cur){
				let last_idx:usize=idx[cur][idx_cur_len-1];
				idx[cur].push(last_idx+1);
				rmatch=Self::get_posets_iter3(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 3: {:?}",rmatch);
				//return rmatch;
			}
			
			else{
				// STILL IN CAPACITY OF ACCEPTORS
				if wtmp[w_mi].len()<qu_st[w_mi]{
					//idx[cur]+=1;
					wtmp[w_mi].push(cur);
					let last_idx:usize=idx[cur][idx_cur_len-1];
					idx[cur].push(last_idx+1);
					rmatch=Self::get_posets_iter3(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
					//println!("RMATCH EXIT 4: {:?}",rmatch);
					//return rmatch;
				}
				// CAPACITY OVERFLOW OF ACCEPTORS
				else{
					//println!("QUOTA OF ACCEPTOR IS FULL -> SELECT LAST ONE!");
					if wtmp[w_mi].len()>qu_st[w_mi]{
						println!("BIG PROBLEM! QUOTA OVERFLOW!");
					}
					let mut go_next:bool=true;
					let mut pos_least_preferred:usize=0;
					let mut m_least_preferred:usize=wtmp[w_mi][0];
					for i in 0..wtmp[w_mi].len(){
						let wpartner:usize=wtmp[w_mi][i];
						let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
						let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
						//if pos_wprt>pos_cur{
						if pos_cur<pos_wprt{
							go_next=false;
							if pos_least_preferred<pos_wprt{
								pos_least_preferred=pos_wprt;
								m_least_preferred=wpartner;
							}
						}
					}
					//let wpartner:usize=wtmp[w_mi].expect("");
					let wpartner:usize=m_least_preferred;
					let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
					let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
					//if pos_wprt<pos_cur{
					
					// STILL UNPREFERRED BY ACCEPTOR
					if go_next{
						let idx_cur_len:usize=idx[cur].len();
						idx[cur][idx_cur_len-1]+=1;
						//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
						rmatch=Self::get_posets_iter3(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 5: {:?}",rmatch);
						//return rmatch;
					}
					// ACCEPTOR KICKS LAST ONE OUT AND TAKES THE CURRENT PROPOSER!
					else{
						if pos_wprt==pos_cur{
							println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
						}						
							// COPY !!!!!!!
							// BE CAREFUL !									
							//println!("LAST wpartner in iter2:{}",wpartner);
							//mtmp.push(wpartner);
							//mtmp.insert(0,wpartner);
							//wtmp[w_mi]=Some(cur);
							let idx_cur_len:usize=idx[cur].len();
							let last_idx:usize=idx[cur][idx_cur_len-1];
							idx[cur].push(last_idx+1);
							// REARRANGE WTMP !!!!!!!
							let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
							wtmp[w_mi].remove(wpos_cur);
							//println!("BEFORE PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
							wtmp[w_mi].push(cur);
							//println!("AFTER  PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
						// BE VEY CAREFUL !!!!!!!
							//let idx_cur_len:usize=idx[cur].len();
							let delpos_idx_wprt:usize=m[wpartner].iter().position(|x| *x==w_mi).unwrap();
							if !idx[wpartner].contains(&delpos_idx_wprt){
								println!("BIG PROBLEM! NOT IN IDX[WPARTNER]!!!!");
							}
							//println!("cur(new):{}, IDX:{:?}, w_mi:{}, wpartner:{}, del_idxpos_wprt:{}",cur,idx,w_mi,wpartner,delpos_idx_wprt);
							
							//println!("REMOVE IDX_WPRT: {}",delpos_idx_wprt);
							
							// ####### IMPORTANT ####### 
							let del_idx_wprt_final:usize=idx[wpartner].iter().position(|x| *x==delpos_idx_wprt).unwrap();
							
							//idx[wpartner].remove(delpos_idx_wprt);
							
							println!("REMOVE IDX! wpartner:{},del_idx_wprt_final:{},idx:{:?}",wpartner,del_idx_wprt_final,idx);
							idx[wpartner].remove(del_idx_wprt_final);
							
							
							let idx_wprt_len:usize=idx[wpartner].len();
							// BE VERY CAREFUL!!! IF SUCCESS -> PUSH! IF REJECTED -> +=1 !!!!!!!
							//idx[wpartner].push(last_idx+1);
							//idx[wpartner][idx_wprt_len-1]+=1;// .push(last_idx+1);
							mtmp.insert(0,wpartner);

							//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);	
						//}														
						rmatch=Self::get_posets_iter3(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 6: {:?}",rmatch);
						//return rmatch;
					}
					//println!("LAST!");
				}
			}
		}
		//println!("####### END REACHED !!!!!!! #######\n{:?}",rmatch);
		if !Self::check_consistency_mtmp_wtmp(&rmatch){
			return vec![];
		}
		rmatch
	}		
	fn get_posets(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		//wtmp_:&Vec<Vec<Option<usize>>>,mtmp_:&Vec<Vec<usize>>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		//idx_:&Vec<usize>,cur_:usize,
		idx_:&Vec<Vec<usize>>,cur_:usize,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		println!("INPUT DATA\nwtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		//let mut wtmp:Vec<Vec<Option<usize>>>=wtmp_.clone();
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		let mut cur:usize=cur_;
		println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		//let w_mi:usize=m[cur][idx[cur].len()-1];
		println!("cur:{}, idx_cur_len:{}",cur,idx_cur_len);
		let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
		println!("wtmp:{:?}",wtmp);
		//if wtmp[w_mi].is_none(){
		println!("w_mi:{}",w_mi);
		if !gusfield_manymany::idx_check(&idx_,&qu_sc){
			println!("ERROR !!!!!!!");
			return vec![];
		}
		
		if wtmp[w_mi].len()<qu_st[w_mi]{
			//idx[cur]+=1;
			//wtmp[w_mi]=Some(cur);
			wtmp[w_mi].push(cur);
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				cur=mtmp[0];
				println!("FIRST");
				rmatch=Self::get_posets(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
			}
			else{
				rmatch=Self::traverse_wtmp(&wtmp,m.len());
			}
		}
		else{
			if wtmp[w_mi].len()>qu_st[w_mi]{
				println!("BIG PROBLEM! QUOTA OVERFLOW!");
			}
			let mut go_next:bool=true;
			let mut pos_least_preferred:usize=0;
			let mut m_least_preferred:usize=wtmp[w_mi][0];
			for i in 0..wtmp[w_mi].len(){
				let wpartner:usize=wtmp[w_mi][i];
				let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
				let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
				//if pos_wprt>pos_cur{
				if pos_cur<pos_wprt{
					go_next=false;
					if pos_least_preferred<pos_wprt{
						pos_least_preferred=pos_wprt;
						m_least_preferred=wpartner;
					}
				}
			}
			//let wpartner:usize=wtmp[w_mi].expect("");
			let wpartner:usize=m_least_preferred;
			let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
			let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
			//if pos_wprt<pos_cur{
			if go_next{
				let idx_cur_len:usize=idx[cur].len();
				idx[cur][idx_cur_len-1]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			else{
				if pos_wprt==pos_cur{
					println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
				}
				let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
				mtmp.remove(delpos);
				mtmp.push(wpartner);
				//wtmp[w_mi]=Some(cur);
				let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
				wtmp[w_mi].remove(wpos_cur);
				wtmp[w_mi].push(cur);
				cur=wpartner;
				//idx[wpartner]+=1;
				let idx_wprt_len:usize=idx[wpartner].len();
				idx[wpartner][idx_wprt_len-1]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			println!("LAST!");
			rmatch=Self::get_posets(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
		}
		rmatch
	}
	// iterating until capacity of proposers are full!
	fn get_posets_iter(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		//wtmp_:&Vec<Vec<Option<usize>>>,mtmp_:&Vec<Vec<usize>>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		//idx_:&Vec<usize>,cur_:usize,
		idx_:&Vec<Vec<usize>>,cur_:usize,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		println!("INPUT DATA\nwtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		//let mut wtmp:Vec<Vec<Option<usize>>>=wtmp_.clone();
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		let mut cur:usize=cur_;
		println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		//let w_mi:usize=m[cur][idx[cur].len()-1];
		//println!("cur:{}, idx_cur_len:{}",cur,idx_cur_len);
		
		
		//let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
		//println!("wtmp:{:?}",wtmp);
		//if wtmp[w_mi].is_none(){
		//println!("w_mi:{}",w_mi);
		if !gusfield_manymany::idx_check(&idx_,&qu_sc){
			println!("ERROR !!!!!!!");
			//return vec![];

		}
		//if idx[cur][idx_cur_len-1]==qu_sc[cur]{
		if idx[cur][idx_cur_len-1]>=w.len(){
			println!("PROPOSALS AT THE END!");
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				cur=mtmp[0];
				//rmatch=
				rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
			}
			else{
				// FINISHED! MATCH!
				rmatch=Self::traverse_wtmp(&wtmp,m.len());

			}
		}
		else{
			
			println!("cur:{}, idx:{:?}, idx_cur_len:{}",cur,idx,idx_cur_len);
			let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
			println!("w_mi:{}",w_mi);
			
		
			if wtmp[w_mi].len()<qu_st[w_mi]{
				//idx[cur]+=1;
				wtmp[w_mi].push(cur);
				if idx[cur].len()<qu_sc[cur]{
					let last_idx:usize=idx[cur][idx_cur_len-1];
					//idx[cur].push(idx[cur][idx_cur_len]);
					if last_idx<w.len()-1{
						idx[cur].push(last_idx+1);
						println!("FIRST");
						rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
					}
					else{
						println!("FIRST ELSE!");
						let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
						mtmp.remove(delpos);
						if mtmp.len()>0{
							cur=mtmp[0];
							println!("SECOND");
							rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
						}
						else{
							rmatch=Self::traverse_wtmp(&wtmp,m.len());
						}						
					}
					//rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
				}
				else{
					let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
					mtmp.remove(delpos);
					if mtmp.len()>0{
						cur=mtmp[0];
						println!("SECOND");
						rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
					}
					else{
						rmatch=Self::traverse_wtmp(&wtmp,m.len());
					}
				}
			}
			else{
				if wtmp[w_mi].len()>qu_st[w_mi]{
					println!("BIG PROBLEM! QUOTA OVERFLOW!");
				}
				let mut go_next:bool=true;
				let mut pos_least_preferred:usize=0;
				let mut m_least_preferred:usize=wtmp[w_mi][0];
				for i in 0..wtmp[w_mi].len(){
					let wpartner:usize=wtmp[w_mi][i];
					let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
					let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
					//if pos_wprt>pos_cur{
					if pos_cur<pos_wprt{
						go_next=false;
						if pos_least_preferred<pos_wprt{
							pos_least_preferred=pos_wprt;
							m_least_preferred=wpartner;
						}
					}
				}
				//let wpartner:usize=wtmp[w_mi].expect("");
				let wpartner:usize=m_least_preferred;
				let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
				let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
				//if pos_wprt<pos_cur{
				if go_next{
					let idx_cur_len:usize=idx[cur].len();
					idx[cur][idx_cur_len-1]+=1;
					//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
				}
				else{
					if pos_wprt==pos_cur{
						println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
					}

					/*
					if idx[cur].len()<qu_sc[cur]{
						let idx_cur_len:usize=idx[cur].len();
						let last_idx:usize=idx[cur][idx_cur_len-1];
						//idx[cur].push(idx[cur][idx_cur_len]);
						idx[cur].push(last_idx+1);
						println!("FIRST");
						rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
					}
					else{
					*/
						
						// COPY !!!!!!!
						// BE CAREFUL !
						//let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
						//mtmp.remove(delpos);
						
						/*
						let last_idx:usize=idx[cur][idx_cur_len-1];
						//idx[cur].push(idx[cur][idx_cur_len]);
						idx[cur].push(last_idx+1);
						*/
											
						println!("LAST wpartner:{}",wpartner);
						//mtmp.push(wpartner);
						mtmp.insert(0,wpartner);
						//wtmp[w_mi]=Some(cur);
						let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
						wtmp[w_mi].remove(wpos_cur);
						println!("BEFORE PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
						wtmp[w_mi].push(cur);
						println!("AFTER  PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
					// BE VEY CAREFUL !!!!!!!
						let idx_cur_len:usize=idx[cur].len();

						/*
						let last_idx:usize=idx[cur][idx_cur_len-1];
						//idx[cur].push(idx[cur][idx_cur_len]);
						
						if last_idx<w.len()-1{
							idx[cur].push(last_idx+1);
						}
						else{
							
						}
						*/
					if idx[cur].len()<qu_sc[cur]{
							
							
						let last_idx:usize=idx[cur][idx_cur_len-1];
						//idx[cur].push(idx[cur][idx_cur_len]);
						if last_idx<w.len()-1{
							idx[cur].push(last_idx+1);
							println!("FIRST");
							rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
						}
						else{
							println!("FIRST ELSE!");
							let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
							mtmp.remove(delpos);
							if mtmp.len()>0{
								cur=mtmp[0];
								println!("SECOND");
								rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
							}
							else{
								rmatch=Self::traverse_wtmp(&wtmp,m.len());
							}						
						}
						//rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
					}
					else{
						let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
						mtmp.remove(delpos);
						if mtmp.len()>0{
							cur=mtmp[0];
							println!("SECOND");
							rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
						}
						else{
							rmatch=Self::traverse_wtmp(&wtmp,m.len());
						}
					}
							
						
						/*
						
					// BE VERY CAREFUL !!!!!!! THIS REMAINS THAT WAY !!!!!!! qu_sc[cur] + 1 is IMPORTANT, ESPECIALLY "+ 1" !!!!!!!
					if idx[cur].len()==qu_sc[cur]+1{
						println!("QUOTA FULL for #### {} ####, quota:{:?}, idx:{:?}",cur,qu_sc,idx);
						let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
						mtmp.remove(delpos);
					}
					
						*/
						cur=wpartner;
						//idx[wpartner]+=1;
						let mut del_idxpos_wprt:usize=idx[wpartner][0];
						for i in 0..idx[wpartner].len(){
							if m[wpartner][idx[wpartner][i]]==w_mi{
								del_idxpos_wprt=i;
								break;
							}
						}
						
						println!("cur(new):{}, IDX:{:?}, w_mi:{}, wpartner:{}, del_idxpos_wprt:{}",cur,idx,w_mi,wpartner,del_idxpos_wprt);
						let idx_wprt_len:usize=idx[wpartner].len();
						let last_idx:usize=idx[wpartner][idx_wprt_len-1];
						
						if last_idx<m[0].len()-1{
							idx[wpartner].push(last_idx+1);
						}
						idx[wpartner].remove(del_idxpos_wprt);
						/*
						idx[wpartner][idx_wprt_len-1]+=1;
						*/
						//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);	
					//}		
					
							
				}
				println!("LAST!");
				rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
			}
		}
		rmatch
	}
		


	// iterating until capacity of proposers are full!
	fn get_posets_iter2(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc_:&Vec<usize>,qu_st_:&Vec<usize>,
		wtmp_:&Vec<Vec<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<Vec<usize>>,
	)->Vec<Vec<usize>>{
		//println!("get_poset2!");
		//println!("INPUT DATA\nsc:{:?}\nst:{:?},wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",m,w,wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<Vec<usize>>=vec![];
		//let mut wtmp:Vec<Vec<Option<usize>>>=wtmp_.clone();
		let mut wtmp:Vec<Vec<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut qu_sc:Vec<usize>=qu_sc_.clone();
		let mut qu_st:Vec<usize>=qu_st_.clone();
		let mut idx:Vec<Vec<usize>>=idx_.clone();
		//let mut cur:usize=cur_;
		let mut cur:usize=mtmp[0];
		//println!("INPUT ITER2 DATA\ncur:{}, wtmp:{:?}, mtmp:{:?}, idx:{:?}",cur,wtmp,mtmp,idx);
		//println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let idx_cur_len:usize=idx[cur].len();
		//let w_mi:usize=m[cur][idx[cur].len()-1];1
		//println!("cur:{}, idx_cur_len:{}",cur,idx_cur_len);
				
		//let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
		//println!("wtmp:{:?}",wtmp);
		//if wtmp[w_mi].is_none(){
		//println!("w_mi:{}",w_mi);
		
		/*
		if !gusfield_manymany::idx_check(&idx_,&qu_sc){
			println!("ERROR !!!!!!!");
			//return vec![];
		}
		*/
		//if idx[cur][idx_cur_len-1]==qu_sc[cur]{
		
		// PREF OVERFLOW
		//if idx[cur][idx_cur_len-1]>=w.len(){
		//if idx[cur][idx_cur_len-1]>=w.len() || idx[cur].len()-1>=qu_sc[cur]{
		// ADDED M[CUR].LEN() <= IDX... AS ADDITIONAL CONDITION!
		if idx[cur][idx_cur_len-1]>=w.len() || idx[cur].len()-1>=qu_sc[cur] || m[cur].len()<=idx[cur][idx_cur_len-1]{
			//println!("PROPOSALS AT THE END OR PROPOSER-QUOTA FULL!");
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				//cur=mtmp[0];
				//rmatch=
				//rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
				rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 1: {:?}",rmatch);
				//return rmatch;
			}
			else{
				// FINISHED! MATCH!
				rmatch=Self::traverse_wtmp(&wtmp,m.len());
				//println!("RMATCH EXIT 2: {:?}",rmatch);
				//return rmatch;
			}
		}
		else{			
			println!("cur:{}, idx:{:?}, idx_cur_len:{}",cur,idx,idx_cur_len);
			println!("m:{:?}",m);
			/*
			if m[cur].len()<=idx[cur][idx_cur_len-1]{
				return vec![];
			}
			*/
			let w_mi:usize=m[cur][idx[cur][idx_cur_len-1]];
			//println!("w_mi:{}",w_mi);			
					
			if !w[w_mi].contains(&cur){
				let last_idx:usize=idx[cur][idx_cur_len-1];
				idx[cur].push(last_idx+1);
				rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				//println!("RMATCH EXIT 3: {:?}",rmatch);
				//return rmatch;
			}
			
			else{
				// STILL IN CAPACITY OF ACCEPTORS
				if wtmp[w_mi].len()<qu_st[w_mi]{
					//idx[cur]+=1;
					wtmp[w_mi].push(cur);
					let last_idx:usize=idx[cur][idx_cur_len-1];
					idx[cur].push(last_idx+1);
					rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
					//println!("RMATCH EXIT 4: {:?}",rmatch);
					//return rmatch;
					/*
					if idx[cur].len()<qu_sc[cur]{
						let last_idx:usize=idx[cur][idx_cur_len-1];
						//idx[cur].push(idx[cur][idx_cur_len]);
						if last_idx<w.len()-1{
							idx[cur].push(last_idx+1);
							println!("FIRST");
							rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
						}
						else{
							println!("FIRST ELSE!");
							let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
							mtmp.remove(delpos);
							if mtmp.len()>0{
								cur=mtmp[0];
								println!("SECOND");
								rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
							}
							else{
								rmatch=Self::traverse_wtmp(&wtmp,m.len());
							}						
						}
						//rmatch=Self::get_posets_iter(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
					}
					else{
						let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
						mtmp.remove(delpos);
						if mtmp.len()>0{
							cur=mtmp[0];
							println!("SECOND");
							rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
						}
						else{
							rmatch=Self::traverse_wtmp(&wtmp,m.len());
						}
					}
					*/
				}
				// CAPACITY OVERFLOW OF ACCEPTORS
				else{
					//println!("QUOTA OF ACCEPTOR IS FULL -> SELECT LAST ONE!");
					if wtmp[w_mi].len()>qu_st[w_mi]{
						println!("BIG PROBLEM! QUOTA OVERFLOW!");
					}
					let mut go_next:bool=true;
					let mut pos_least_preferred:usize=0;
					let mut m_least_preferred:usize=wtmp[w_mi][0];
					for i in 0..wtmp[w_mi].len(){
						let wpartner:usize=wtmp[w_mi][i];
						let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
						let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
						//if pos_wprt>pos_cur{
						if pos_cur<pos_wprt{
							go_next=false;
							if pos_least_preferred<pos_wprt{
								pos_least_preferred=pos_wprt;
								m_least_preferred=wpartner;
							}
						}
					}
					//let wpartner:usize=wtmp[w_mi].expect("");
					let wpartner:usize=m_least_preferred;
					let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
					let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
					//if pos_wprt<pos_cur{
					
					// STILL UNPREFERRED BY ACCEPTOR
					if go_next{
						let idx_cur_len:usize=idx[cur].len();
						idx[cur][idx_cur_len-1]+=1;
						//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
						rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 5: {:?}",rmatch);
						//return rmatch;
					}
					// ACCEPTOR KICKS LAST ONE OUT AND TAKES THE CURRENT PROPOSER!
					else{
						if pos_wprt==pos_cur{
							println!("A BIG BIG PROBLEM !!!! SAME GUY!!!");
						}
						
							// COPY !!!!!!!
							// BE CAREFUL !
							//let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
							//mtmp.remove(delpos);						
							/*
							let last_idx:usize=idx[cur][idx_cur_len-1];
							//idx[cur].push(idx[cur][idx_cur_len]);
							idx[cur].push(last_idx+1);
							*/											
							println!("LAST wpartner in iter2:{}",wpartner);
							//mtmp.push(wpartner);
							//mtmp.insert(0,wpartner);
							//wtmp[w_mi]=Some(cur);
							let idx_cur_len:usize=idx[cur].len();
							let last_idx:usize=idx[cur][idx_cur_len-1];
							idx[cur].push(last_idx+1);
							// REARRANGE WTMP !!!!!!!
							let wpos_cur:usize=wtmp[w_mi].iter().position(|x| *x==wpartner).unwrap();
							wtmp[w_mi].remove(wpos_cur);
							//println!("BEFORE PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
							wtmp[w_mi].push(cur);
							//println!("AFTER  PUSH CUR: cur:{}, w_mi:{}, wtmp:{:?}, idx:{:?}",cur,w_mi,wtmp,idx);
						// BE VEY CAREFUL !!!!!!!
							//let idx_cur_len:usize=idx[cur].len();


							let delpos_idx_wprt:usize=m[wpartner].iter().position(|x| *x==w_mi).unwrap();
							if !idx[wpartner].contains(&delpos_idx_wprt){
								println!("BIG PROBLEM! NOT IN IDX[WPARTNER]!!!!");
							}
							
							println!("cur(new):{}, IDX:{:?}, w_mi:{}, wpartner:{}, del_idxpos_wprt:{}",cur,idx,w_mi,wpartner,delpos_idx_wprt);
							//let idx_wprt_len:usize=idx[wpartner].len();
							//let last_idx:usize=idx[wpartner][idx_wprt_len-1];
							
							//if last_idx<m[0].len()-1{
							//	idx[wpartner].push(last_idx+1);
							//}
							//idx[wpartner].remove(del_idxpos_wprt);
							println!("REMOVE IDX_WPRT: {}",delpos_idx_wprt);
							
							// ####### IMPORTANT ####### 
							let del_idx_wprt_final:usize=idx[wpartner].iter().position(|x| *x==delpos_idx_wprt).unwrap();
							
							//idx[wpartner].remove(delpos_idx_wprt);
							idx[wpartner].remove(del_idx_wprt_final);
							
							
							let idx_wprt_len:usize=idx[wpartner].len();
							// BE VERY CAREFUL!!! IF SUCCESS -> PUSH! IF REJECTED -> +=1 !!!!!!!
							//idx[wpartner].push(last_idx+1);
							//idx[wpartner][idx_wprt_len-1]+=1;// .push(last_idx+1);
							mtmp.insert(0,wpartner);
							/*
							idx[wpartner][idx_wprt_len-1]+=1;
							*/
							//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);	
						//}														
						rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
						//println!("RMATCH EXIT 6: {:?}",rmatch);
						//return rmatch;
					}
					println!("LAST!");
					//rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
					//rmatch=Self::get_posets_iter2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx);
				}
			}
		}
		//println!("####### END REACHED !!!!!!! #######\n{:?}",rmatch);
		rmatch
	}		
	fn enumerate_from_match2(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_m:&Vec<usize>,qu_w:&Vec<usize>,
		matchstack:&Vec<Vec<Vec<usize>>>,
		match_:&Vec<Vec<usize>>)->Vec<Vec<Vec<usize>>>{
		let mut matches:Vec<Vec<Vec<usize>>>=vec![];
		let (mut m_sc, mut w_sc):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists(&m,&w,&match_);
		let match_transformed:Vec<Vec<usize>>=Self::transform_tmp_matrix(&match_);
		let (mut w_sc1, mut m_sc1):(Vec<Vec<usize>>,Vec<Vec<usize>>)=Self::shorten_lists(&w,&m,&match_transformed);
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
		for i in 0..match_.len(){
			let (m_toggled,w_toggled)=Self::toggle_node((&m_sc1,&w_sc1),i);
			let matchresult:Vec<Vec<usize>>=Self::get_posets_iter2(&m_toggled,&w_toggled,&qu_m,&qu_w,&wtmp,&mtmp,&idx);
			if !matchstack.contains(&matchresult){
				if !matches.contains(&matchresult){
					matches.push(matchresult);
				}
			}
		}
		let mut output:Vec<Vec<Vec<usize>>>=matchstack.clone();
		output.append(&mut matches);
		output
	}
	/*
	fn enumeration2(&self){
		let mut matches:Vec<Vec<usize>>=vec![];
		let mut m:Vec<Vec<usize>>=self.sc.clone();
		let mut w:Vec<Vec<usize>>=self.st.clone();
		let mut mtmp:Vec<usize>=Self::create_full_mtmp_vec(m.len());
		let mut wtmp:Vec<Vec<usize>>=Self::create_empty_wtmp_vec(w.len());
		let mut idx:Vec<Vec<usize>>=vec![vec![0];m.len()];
		let opt:Vec<Vec<usize>>=Self::get_posets_iter2(&m,&w,&self.qu_sc,&self.qu_st,&mtmp,&wtmp,&idx);
		let pess:Vec<Vec<usize>>=Self::get_posets_iter2(&w,&m,&self.qu_st,&self.qu_sc,&mtmp,&wtmp,&idx);
		
	}
	*/
}

struct gusfield{
	m:Vec<Vec<u16>>,
	w:Vec<Vec<u16>>,
	gsm:Vec<
	usize>,
	gsw:Vec<usize>,
	posets:Vec<Vec<usize>>,
	edges:Vec<Vec<usize>>,
}
impl gusfield{
	fn new()->Self{
		println!("GUSFIELD::NEW()");
		Self{
			//m:vec![],
			//w:vec![],
			m:read_txt("pref/m.txt".to_string()),
			w:read_txt("pref/w.txt".to_string()),
			gsm:vec![],
			gsw:vec![],
			posets:vec![],
			edges:vec![],
		}
	}
	fn u16_2_usize(matrix:&Vec<Vec<u16>>)->Vec<Vec<usize>>{
		let mut mat:Vec<Vec<usize>>=vec![];
		for i in 0..matrix.len(){
			let mut mat_i:Vec<usize>=vec![];
			for j in 0..matrix[i].len(){
				mat_i.push(matrix[i][j] as usize);
			}
			mat.push(mat_i);
		}
		mat
	}
	fn init(&mut self){
		let mut gsm:gale_shapley=gale_shapley::new();
		gsm.init(&Self::u16_2_usize(&self.m),&Self::u16_2_usize(&self.w));
		gsm.run();
		self.gsm=gsm.matchres.as_ref().expect("").clone();
		let mut gsw:gale_shapley=gale_shapley::new();
		gsw.init(&Self::u16_2_usize(&self.w),&Self::u16_2_usize(&self.m));
		gsw.run();
		// THINK OF TRAVERSE!
		//self.gsw=gsw.matchres.as_ref().expect("").clone();
		
		self.gsw=gsw.matchres.as_ref().expect("").clone();
		self.posets.push(self.gsm.clone());
	}
	fn get_all_stable_matches(&mut self){
		
	}
	fn next_level(&mut self,posets:&Vec<Vec<usize>>){
		//println!("next_level!\nposets:{:?}",posets);
		let idx_:Vec<usize>=self.create_zero_idx();
		
		// Later it has to be adapted to posets!!!!!!!
		let mtmp:Vec<usize>=self.create_full_mtmp();
		
		// Later it has to be adapted to posets!!!!!!!
		let wtmp:Vec<Option<usize>>=self.create_empty_wtmp();
		for i in 0..posets.len(){
			let pos_poset:usize=self.posets.iter().position(|x| *x==posets[i]).unwrap();
			let (m,w)=self.reduce_list(&posets[i]);
			let musize:Vec<Vec<usize>>=Self::u16_2_usize(&m);
			let wusize:Vec<Vec<usize>>=Self::u16_2_usize(&w);
			let mut next_posets:Vec<Vec<usize>>=vec![];
			//println!("reduced lists:\nm:{:?}\nw:{:?}",m,w);
			for j in 0..m.len(){
				//if m[j].len()>1{
				let w_mj:usize=musize[j][0];
				// IS J OF M AN W-OPT ASSIGNMENT TO W_MJ?
				if self.gsw[w_mj]!=j{
				//if m[i].len()>idx_[i]{
					let mut idx:Vec<usize>=idx_.clone();
					idx[j]=1;
					//println!("m[{}]:{:?}",j,m[j]);
					//println!("poset[{}] @{}:{:?}",i,j,posets[i]);
					let poset_j:Vec<usize>=Self::get_posets2(&musize,&wusize,&wtmp,&mtmp,&idx,j);
					//println!("### MATCH! poset_i @{}: {:?}",j,poset_j);
					if !self.posets.contains(&poset_j){
						self.edges.push(vec![pos_poset,self.posets.len()]);
						self.posets.push(poset_j.clone());
						next_posets.push(poset_j);
						
					}
				}
			}
			self.next_level(&next_posets);
		}
	}
	// not needed, we made it shorter! :)
	fn is_mj_last_in_gsw(&self,mj:usize,w_mj:usize)->bool{
		let gsw:Vec<usize>=self.gsw.clone();
		if gsw[w_mj]==mj{
			return true;
		}
		false
	}
	fn create_full_mtmp(&self)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..self.m.len(){
			mtmp.push(i);
		}
		mtmp
	}
	fn create_empty_wtmp(&self)->Vec<Option<usize>>{
		let mut tmp:Vec<Option<usize>>=vec![];
		for i in 0..self.m.len(){
			tmp.push(None);
		}
		tmp
	}
	fn create_zero_idx(&self)->Vec<usize>{
		let mut idx:Vec<usize>=vec![];
		for i in 0..self.m.len(){
			idx.push(0);
		}
		idx
	}
	fn traverse_wtmp(wtmp_:&Vec<Option<usize>>)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		let mut wtmp:Vec<usize>=vec![];
		for i in 0..wtmp_.len(){
			wtmp.push(*wtmp_[i].as_ref().expect(""));
		}
		for i in 0..wtmp.len(){
			let pos:usize=wtmp.iter().position(|x| *x==i).unwrap();
			mtmp.push(pos);
		}
		mtmp
	}
	fn get_posets(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		wtmp_:&Vec<Option<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<usize>,cur_:usize,
	)->Vec<usize>{
		println!("wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<usize>=vec![];
		let mut wtmp:Vec<Option<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut idx:Vec<usize>=idx_.clone();
		let mut cur:usize=cur_;
		println!("cur:{}, idx:{:?}, m[]:{:?}",cur,idx,m);
		let w_mi:usize=m[cur][idx[cur]];
		if wtmp[w_mi].is_none(){
			//idx[cur]+=1;
			wtmp[w_mi]=Some(cur);
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				cur=mtmp[0];
				rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);
			}
			else{
				rmatch=Self::traverse_wtmp(&wtmp);
			}
		}
		else{
			let wpartner:usize=wtmp[w_mi].expect("");
			let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
			let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
			if pos_wprt<pos_cur{
				idx[cur]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			else{
				if pos_wprt==pos_cur{
					println!(" A BIG BIG PROBLEM !!!! SAME GUY!!!");
				}
				let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
				mtmp.remove(delpos);
				mtmp.push(wpartner);
				wtmp[w_mi]=Some(cur);
				cur=wpartner;
				idx[wpartner]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
		}
		rmatch
	}
	fn get_posets2(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		wtmp_:&Vec<Option<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<usize>,cur_:usize,
	)->Vec<usize>{
		//println!("get_poset2!");
		//println!("wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<usize>=vec![];
		let mut wtmp:Vec<Option<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		let mut idx:Vec<usize>=idx_.clone();
		let mut cur:usize=cur_;
		//println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let w_mi:usize=m[cur][idx[cur]];
		if wtmp[w_mi].is_none(){
			//idx[cur]+=1;
			wtmp[w_mi]=Some(cur);
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				cur=mtmp[0];
				rmatch=Self::get_posets2(&m,&w,&wtmp,&mtmp,&idx,cur);
			}
			else{
				rmatch=Self::traverse_wtmp(&wtmp);
			}
		}
		else{
			let wpartner:usize=wtmp[w_mi].expect("");
			let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
			let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
			if pos_wprt<pos_cur{
				idx[cur]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			else{
				if pos_wprt==pos_cur{
					println!(" A BIG BIG PROBLEM !!!! SAME GUY!!!");
				}
				let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
				mtmp.remove(delpos);
				mtmp.push(wpartner);
				wtmp[w_mi]=Some(cur);
				cur=wpartner;
				idx[wpartner]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			rmatch=Self::get_posets2(&m,&w,&wtmp,&mtmp,&idx,cur);				
		}
		rmatch
	}
	fn reduce_list(&mut self,match_:&Vec<usize>)->(Vec<Vec<u16>>,Vec<Vec<u16>>){
		//let mut m:Vec<Vec<u16>>=vec![];
		let mut m:Vec<Vec<u16>>=self.m.clone();
		//let mut w:Vec<Vec<u16>>=vec![];
		let mut w:Vec<Vec<u16>>=self.w.clone();
		for i in 0..match_.len(){
			// men's side for deleting all w who are better than match
			let elem:usize=match_[i];
			let pos:usize=m[i].iter().position(|x| *x==elem as u16).unwrap();
			let mut del:Vec<usize>=vec![];
			for j in 0..pos{
				del.insert(0,j);
				let posw:usize=w[m[i][j] as usize].iter().position(|x| *x==i as u16).unwrap();
				w[m[i][j] as usize].remove(posw);
			}
			for j in 0..del.len(){
				m[i].remove(del[j]);
			}
			// women's side for deleting all men who are worse than match
			let pos:usize=w[elem].iter().position(|x| *x==i as u16).unwrap();
			if pos<w[elem].len()-1{
				del=vec![];
				for j in pos+1..w[elem].len(){
					del.insert(0,j);
					let mj:usize=w[elem][j] as usize;
					let posm:usize=m[mj].iter().position(|x| *x==elem as u16).unwrap();
					m[mj].remove(posm);
				}
				for j in 0..del.len(){
					w[elem].remove(del[j]);
				}
			}
		}
		(m,w)
	}
	fn check_consistency(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>)->bool{
		for i in 0..m.len(){
			for j in 0..m[i].len(){
				if !w[m[i][j]].contains(&i){
					return false;
				}
			}
		}
		for i in 0..w.len(){
			for j in 0..w[i].len(){
				if !m[w[i][j]].contains(&i){
					return false;
				}
			}
		}
		true
	}
	fn print_matches(&self){
		println!("#### MATCHES: #### TOTAL LEN: {}",self.posets.len());
		for i in 0..self.posets.len(){
			println!("{:?}",self.posets[i]);
		}
	}
	fn print_edges(&self){
		println!("#### EDGES: TOTAL LEN: {}",self.edges.len());
		for i in 0..self.edges.len(){
			println!("{:?}",self.edges[i]);
		}
	}
}



struct gusfield_manyone{
	sc:Vec<Vec<u16>>,
	st:Vec<Vec<u16>>,
	qu_sc:Vec<usize>,
	qu_st:Vec<usize>,
}
impl gusfield_manyone{
	fn init()->Self{
		Self{
			sc:read_txt("manyone_pref/schools.txt".to_string()),
			st:read_txt("manyone_pref/students.txt".to_string()),
			qu_sc:read_txt_usize("manyone_pref/quota_schools.txt".to_string())[0].clone(),
			qu_st:read_txt_usize("manyone_pref/quota_students.txt".to_string())[0].clone(), // later for many-to-many matchings!!!!!!!
		}
	}
	fn show_lists(&self){
		println!("Schools:");
		for i in 0..self.sc.len(){
			println!("{:?}",self.sc[i]);
		}
		println!("Students:");
		for i in 0..self.st.len(){
			println!("{:?}",self.st[i]);
		}
		println!("Quota Schools: {:?}",self.qu_sc);
		println!("Quota Students: {:?}",self.qu_st);
	}
	fn complete_lists(&mut self){
		let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		if consistency{
			let st_last:u16=self.st.len() as u16;
			//let st_last:u16=self.st.len() as u16;
			let sc_last:u16=self.sc.len() as u16;
			for i in 0..self.sc.len(){				
				for j in self.sc[i].len()-1 as usize..(st_last-1) as usize{
					self.sc[i].push(st_last);
				}
			}
			let lastsc:Vec<u16>=vec![st_last;st_last as usize];
			self.sc.push(lastsc);
			for i in 0..self.st.len(){				
				for j in self.st[i].len()-1 as usize..(sc_last-1) as usize{
					self.st[i].push(sc_last);
				}
			}
			let lastst:Vec<u16>=vec![sc_last;sc_last as usize];
			self.st.push(lastst);
		}
		else{
			// ???????
		}
	}
	fn check_list_consistency(&self)->(bool,usize,usize){
		let mut sc_longest:usize=0; // horizontal length
		let mut st_longest:usize=0; // horizontal length
		for i in 0..self.sc.len(){
			if self.sc[i].len()>sc_longest{
				sc_longest=self.sc[i].len();
			}
		}
		for i in 0..self.st.len(){
			if self.st[i].len()>st_longest{
				st_longest=self.st[i].len();
			}
		}
		let mut is_consistent:bool=false;
		if sc_longest==self.st.len() && st_longest==self.sc.len(){
			is_consistent=true;
		}
		(is_consistent,sc_longest,st_longest)
	}
	fn create_full_mtmp(&self)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..self.sc.len(){
			mtmp.push(i);
		}
		mtmp
	}
	fn create_empty_wtmp(&self)->Vec<Option<usize>>{
		let mut tmp:Vec<Option<usize>>=vec![];
		for i in 0..self.st.len(){
			tmp.push(None);
		}
		tmp
	}
	fn traverse_wtmp(wtmp_:&Vec<Option<usize>>)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		let mut wtmp:Vec<usize>=vec![];
		println!("traverse wtmp -> wtmp:{:?}",wtmp_);
		for i in 0..wtmp_.len(){
			wtmp.push(*wtmp_[i].as_ref().expect(""));
		}
		for i in 0..wtmp.len(){
			let pos:usize=wtmp.iter().position(|x| *x==i).unwrap();
			mtmp.push(pos);
		}
		mtmp
	}
	fn get_posets2(
		m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,
		qu_sc:&Vec<usize>,qu_st:&Vec<usize>,
		wtmp_:&Vec<Option<usize>>,mtmp_:&Vec<usize>,
		idx_:&Vec<usize>,cur_:usize,
	)->Vec<usize>{
		//println!("get_poset2!");
		//println!("wtmp:{:?}, mtmp:{:?}, idx:{:?}, cur:{}",wtmp_,mtmp_,idx_,cur_);
		let mut rmatch:Vec<usize>=vec![];
		let mut wtmp:Vec<Option<usize>>=wtmp_.clone();
		let mut mtmp:Vec<usize>=mtmp_.clone();
		
		let mut idx:Vec<usize>=idx_.clone();
		let mut cur:usize=cur_;
		println!("cur:{}, idx:{:?}, m:{:?}",cur,idx,m);
		let w_mi:usize=m[cur][idx[cur]];
		println!("wtmp:{:?}",wtmp);
		if wtmp[w_mi].is_none(){
			//idx[cur]+=1;
			wtmp[w_mi]=Some(cur);
			let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
			mtmp.remove(delpos);
			if mtmp.len()>0{
				cur=mtmp[0];
				rmatch=Self::get_posets2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);
			}
			else{
				rmatch=Self::traverse_wtmp(&wtmp);
			}
		}
		else{
			let wpartner:usize=wtmp[w_mi].expect("");
			let pos_cur:usize=w[w_mi].iter().position(|x| *x==cur).unwrap();
			let pos_wprt:usize=w[w_mi].iter().position(|x| *x==wpartner).unwrap();
			if pos_wprt<pos_cur{
				idx[cur]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			else{
				if pos_wprt==pos_cur{
					println!(" A BIG BIG PROBLEM !!!! SAME GUY!!!");
				}
				let delpos:usize=mtmp.iter().position(|x| *x==cur).unwrap();
				mtmp.remove(delpos);
				mtmp.push(wpartner);
				wtmp[w_mi]=Some(cur);
				cur=wpartner;
				idx[wpartner]+=1;
				//rmatch=Self::get_posets(&m,&w,&wtmp,&mtmp,&idx,cur);				
			}
			rmatch=Self::get_posets2(&m,&w,&qu_sc,&qu_st,&wtmp,&mtmp,&idx,cur);				
		}
		rmatch
	}
		
}


struct gale_shapley{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	proposals:Vec<usize>,
	unmatched:Vec<usize>,
	wtmp:Vec<usize>,
	// when the algorithm finishes, the vale of wtmp will be copied to matchres
	matchres:Option<Vec<usize>>
}
impl gale_shapley{
	fn new()->Self{
		Self{
			m:vec![],
			w:vec![],
			// proposal vector that shows the current choice (1st,2nd,3rd,4th...) of male person i
			proposals:vec![],
			// this vector refers to the unmatched male persons
			unmatched:vec![],
			//the female vector that stores the current male partners/proposals (m(w_i))
			wtmp:vec![],
			matchres:None
		}
	}
	fn init(&mut self,mpref:&Vec<Vec<usize>>,wpref:&Vec<Vec<usize>>){
		self.m=mpref.clone();
		self.w=wpref.clone();
		let n:usize=self.m.len();
		for i in 0..n{
			self.proposals.push(0);
			self.unmatched.push(i);
			self.wtmp.push(n);
		}
	}
	fn run(&mut self){
		let n:usize=self.m.len();
		while self.unmatched.len()>0{
			let m:usize=self.unmatched[0];
			let wi:usize=self.m[m][self.proposals[m]];
			if self.wtmp[wi]==n{
				self.wtmp[wi]=m;
				self.unmatched.remove(0);
			}
			else{
				let wi_mold:usize=self.wtmp[wi];
				let pos_old:usize=self.w[wi].iter().position(|&x| x==wi_mold).unwrap();
				let pos_new:usize=self.w[wi].iter().position(|&x| x==m).unwrap();
				if pos_new<pos_old{
					self.wtmp[wi]=m;
					self.unmatched.remove(0);
					self.unmatched.push(wi_mold);
					self.proposals[wi_mold]+=1;
				}
				else{
					self.proposals[m]+=1;
				}
			}
		}
		//self.matchres=Some(self.wtmp.clone());
		self.matchres=Some(Self::traverse_wtmp(&self.wtmp.clone()));
	}
	// copy it into SCHOOL CHOICE!
	fn traverse_wtmp(wtmp:&Vec<usize>)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..wtmp.len(){
			let pos:usize=wtmp.iter().position(|x| *x==i).unwrap();
			mtmp.push(pos);
		}
		mtmp
	}
	// this function will display the wtmp vector as mtmp vector where male i shows the match (w(m_i))!
	fn print_match(&self){
		println!("self.wtmp:\n{:?}",self.wtmp);
		println!("self.matchres:\n{:?}",self.matchres);
		let n:usize=self.m.len();
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..n{
			let pos:usize=self.wtmp.iter().position(|&x| x==i).unwrap();
			mtmp.push(pos);
		}
		println!("mtmp/match:\n{:?}",mtmp);		
	}
}


fn test_option(){
	let a:Vec<u16>=vec![0,1,2,3,4,5,6,12];
	let mut a_ref:Vec<Rc<RefCell<u16>>>=vec![];
	let mut b_opt:Vec<Option<Rc<RefCell<u16>>>>=vec![];
	for i in 0..a.len(){
		a_ref.push(Rc::new(RefCell::new(a[i])));
		b_opt.push(Some(a_ref[i].clone()));
	}
	//let b=aref[7].clone();
	if b_opt.contains(&Some(a_ref[7].clone())){
		let pos:usize=b_opt.iter().position(|x| *x==Some(a_ref[7].clone())).unwrap();
		println!("it works! {}",b_opt[pos].clone().expect("").borrow());
	}
}

fn test_borrow(){
	struct atest{
		a:u16,
		b:u16,
		really:bool,
		selfref:Option<Rc<RefCell<atest>>>,
	}
	impl atest{
		fn init()->Self{
			Self{
				a:1,
				b:2,
				really:true,
				selfref:None,
			}
		}
		fn add_selfref(&mut self,selfref:Rc<RefCell<atest>>){
			self.selfref=Some(selfref.clone());
		}
		fn print_something(&self){
			//self.b=4;
			println!("does it really work??? {}",self.really);
		}
	}
	let aref:Rc<RefCell<atest>>=Rc::new(RefCell::new(atest::init()));
	aref.borrow_mut().add_selfref(aref.clone());
	if aref.borrow().really{
		aref.borrow().print_something();
		// trying with borrow & borrow_mut OR with borrow_mut & borrow leads to error! (RefCell already borrowed).
		aref.borrow_mut().selfref.as_ref().expect("").borrow().print_something();
	}
}

fn test_delnodes(){
	let mut prefs:n_side=n_side::init();
	prefs.add_pref();
	prefs.add_block_bits(2);
	prefs.nvec=vec![5,5];
	let nvec:Vec<u16>=prefs.nvec.clone();
	prefs.add_pref_pattern(&nvec);
	prefs.add_nodes();
	//prefs.add_vmatrices();
	prefs.add_vmatrices2();
	prefs.add_vmatrices_to_nodes();
	prefs.add_refmatrices_to_vmatrices();
	prefs.add_delmem();
	let mut vm_m:vmatrix=vmatrix::init(5);
	println!("len of vm pref:{}",prefs.vmatrices.len());
	let prefs_clone:n_side=prefs.clone();
	//prefs.vmatrices[0].borrow_mut().run(&prefs_clone);
	prefs.vmatrices[0].borrow_mut().run2(&prefs_clone);
	//prefs.vmatrices[0].borrow().print_nodes();
	prefs.vmatrices[1].borrow().print_nodes();
	
	/*
	let pref_pattern:Vec<[u16;N]>=n_side::get_pref_pattern_arr(&vec![2,3,4,2],0,&[0;N]);
	println!("pref_pattern:");
	for i in 0..pref_pattern.len(){
		println!("{:?}",pref_pattern[i]);
	}
	let sel_idx:Vec<Vec<usize>>=n_side::select_pref_pattern(2,&vec![2,3,4,2]);
	*/
}

#[derive(Debug,PartialEq,Eq,Clone)]
struct n_side{
	pref:Vec<Vec<Vec<[u16;N]>>>,
	nvec:Vec<u16>,
	//pref_pattern:Vec<Vec<u16>>,
	pref_pattern:Vec<[u16;N]>,
	nodes:Vec<Rc<RefCell<vertice>>>,
	block:Vec<Vec<Vec<u16>>>,
	vmatrices:Vec<Rc<RefCell<vmatrix>>>,
	delmem:Option<Rc<RefCell<delstack>>>,
}
impl n_side{
	fn init()->Self{
		Self{
			pref:vec![],
			nvec:vec![],
			pref_pattern:vec![],
			nodes:vec![],
			block:vec![], // bit patterns for tmp.len={2,3,4,..,n} for checking blocking groups/pairs!
			vmatrices:vec![],
			delmem:None,
		}
	}
	fn add_pref(&mut self){
		let folder:String="3dpref/pref_".to_string();
		for i in 0..N{
			let mut filepath:String=folder.clone();
			filepath+=&i.to_string();
			filepath+=".txt";
			self.pref.push(read_pref(filepath));
		}
	}
	fn add_pref_pattern(&mut self,nvec:&Vec<u16>){
		self.pref_pattern=Self::get_pref_pattern_arr(&self.nvec,0,&[0;N]);
	}
	fn add_nodes(&mut self){
		for i in 0..self.pref_pattern.len(){
			let mut v:vertice=vertice::init();
			v.nodeval=self.pref_pattern[i];
			let vref:Rc<RefCell<vertice>>=Rc::new(RefCell::new(v));
			vref.borrow_mut().add_selfref(vref.clone());
			self.nodes.push(vref);
		}
	}
	// please take care of the idx! self.block[0] refers to n=2, self.block[1] refers to n=3 and so on...!
	fn add_block_bits(&mut self,idx:u16){
		if idx>1{
			for i in 0..idx-1{
				let allbits:Vec<Vec<u16>>=get_all_N_bits(idx,N as u16,&vec![]);
				let del_bits:Vec<Vec<u16>>=delete_imcompatibility(&allbits,idx,N as u16);
				self.block.push(del_bits);
			}
		}
	}
	fn add_delmem(&mut self){
		let mut delmem:delstack=delstack::init();
		let delmem_ref:Rc<RefCell<delstack>>=Rc::new(RefCell::new(delmem));
		for i in 0..self.vmatrices.len(){
			self.vmatrices[i].borrow_mut().add_delmem(delmem_ref.clone());
		}
		for i in 0..self.nodes.len(){
			self.nodes[i].borrow_mut().add_delmem(delmem_ref.clone());
		}
	}
	// this holds for n=2 (2 groups). 
	// if we want to add more groups, we have to implement a new function or alter the current one!
	fn is_stable(&self,a:Rc<RefCell<vertice>>,b:Rc<RefCell<vertice>>)->bool{
		let avalue:[u16;N]=a.borrow().nodeval;
		let bvalue:[u16;N]=b.borrow().nodeval;

		let groups:Vec<[u16;N]>=vec![avalue,bvalue];
		let mut is_ok:bool=true;
		let grouplen:usize=groups.len();
		let blocksize:usize=self.block[grouplen-2].len()/N;
		let mut posgroup:Vec<Vec<usize>>=vec![];
		for i in 0..groups.len(){
			// groups[i].len() must be equal to N !!!!!!!
			let mut posgroup_i:Vec<usize>=vec![];
			for j in 0..N{
				// be careful!
				let pos:usize=self.pref[j][groups[i][j] as usize].iter().position(|x| *x==groups[i]).unwrap();
				posgroup_i.push(pos);
			}
			posgroup.push(posgroup_i);
		}
		for i in 0..self.block[grouplen-2].len(){
			let bl:&Vec<u16>=&self.block[grouplen-2][i];
			
			/*
			for j in 0..N{
				for k in 0..blocksize{
					
				}
			}
			*/
			
			// total numbers of 1 must be equal to N !!!!!!!
			let mut blockgroup:[u16;N]=[0;N];
			let mut idx:Vec<usize>=vec![];
			for j in 0..bl.len(){
				if bl[j]==1{
					idx.push(j);
					blockgroup[j%N]=groups[j/N][j%N];
				}
			}
			if idx.len()!=N{
				println!("BIG PROBLEM IN fn n_side::is_stable() !!!!!!!");
			}
			for j in 0..idx.len(){
				let group:usize=idx[j]/N;
				let player_order:usize=idx[j]%N;
				let player:usize=groups[group][player_order] as usize;
				let rank_block:usize=self.pref[player_order][player].iter().position(|x| *x==groups[group]).unwrap();
				println!("group:{}, player_order:{}, player:{}, rank_block:{}, posgroup[{}][{}]:{}",
					group,player_order,player,rank_block,group,player_order,posgroup[group][player_order]);
				if rank_block<posgroup[group][player_order]{
					is_ok=false;
					break;
				}
			}
		}
		//true
		is_ok
	}
	fn is_stable2(&self,a:Rc<RefCell<vertice>>,b:Rc<RefCell<vertice>>)->bool{
		let avalue:[u16;N]=a.borrow().nodeval;
		let bvalue:[u16;N]=b.borrow().nodeval;

		let groups:Vec<[u16;N]>=vec![avalue,bvalue];
		let mut is_ok:bool=true;
		let grouplen:usize=groups.len();
		let blocksize:usize=self.block[grouplen-2].len()/N;
		let mut posgroup:Vec<Vec<usize>>=vec![];
		for i in 0..groups.len(){
			// groups[i].len() must be equal to N !!!!!!!
			let mut posgroup_i:Vec<usize>=vec![];
			for j in 0..N{
				// be careful!
				let pos:usize=self.pref[j][groups[i][j] as usize].iter().position(|x| *x==groups[i]).unwrap();
				posgroup_i.push(pos);
			}
			posgroup.push(posgroup_i);
		}
		for i in 0..posgroup.len(){
			println!("posgroup[{}]:{:?}",i,posgroup[i]);
		}
		for i in 0..self.block[grouplen-2].len(){
			let bl:&Vec<u16>=&self.block[grouplen-2][i];			
			// total numbers of 1 must be equal to N !!!!!!!
			let mut blockgroup:[u16;N]=[0;N];
			let mut idx:Vec<usize>=vec![];
			for j in 0..bl.len(){
				if bl[j]==1{
					idx.push(j);
					blockgroup[j%N]=groups[j/N][j%N];
				}
			}
			if idx.len()!=N{
				println!("BIG PROBLEM IN fn n_side::is_stable() !!!!!!!");
			}
			//println!("blockgroup:{:?}, idx:{:?}",blockgroup,idx);
			//println!("self.block:{:?}",self.block);
			let mut cn_block:usize=0;
			for j in 0..idx.len(){
				//println!("self.block[{}]:{:?}",idx[j],self.block[idx[j]]);
				let group:usize=idx[j]/N;
				let player_order:usize=idx[j]%N;
				let player:usize=groups[group][player_order] as usize;
				//let rank_block:usize=self.pref[player_order][player].iter().position(|x| *x==groups[group]).unwrap();
				let rank_block:usize=self.pref[player_order][player].iter().position(|x| *x==blockgroup).unwrap();
				println!("group:{}, player_order:{}, player:{}, rank_block:{}, posgroup[{}][{}]:{}, aval:{:?}, bval:{:?}",
					group,player_order,player,rank_block,group,player_order,posgroup[group][player_order],avalue,bvalue);
				if rank_block<posgroup[group][player_order]{
					//is_ok=false;
					//break;
					cn_block+=1;
				}
				if cn_block==N{
					is_ok=false;
					break;
				}
			}
		}
		is_ok
	}
	fn check_compatibility(a:Rc<RefCell<vertice>>,b:Rc<RefCell<vertice>>)->bool{
		let mut is_ok:bool=true;
		let aval:[u16;N]=a.borrow().nodeval.clone();
		let bval:[u16;N]=b.borrow().nodeval.clone();
		for i in 0..N{
			if aval[i]==bval[i]{
				is_ok=false;
				break;
			}
		}
		is_ok
	}
	fn get_pref_pattern(nvec:&Vec<u16>,idx:usize,tmp:&Vec<u16>)->Vec<Vec<u16>>{
		let mut pattern:Vec<Vec<u16>>=vec![];
		if tmp.len()==nvec.len(){
			return vec![tmp.to_vec()];
		}
		else{
			for i in 0..nvec[idx]{
				let mut tmp_i:Vec<u16>=tmp.clone();
				tmp_i.push(i);
				pattern.append(&mut Self::get_pref_pattern(&nvec,idx+1,&tmp_i));
			}
		}
		pattern
	}
	fn get_pref_pattern_arr(nvec:&Vec<u16>,idx:usize,tmp:&[u16;N])->Vec<[u16;N]>{
		let mut pattern:Vec<[u16;N]>=vec![];
		if idx==nvec.len(){
			return vec![*tmp];
		}
		else{
			for i in 0..nvec[idx]{
				let mut tmp_i:[u16;N]=tmp.clone();
				//tmp_i.push(i);
				tmp_i[idx]=i;
				pattern.append(&mut Self::get_pref_pattern_arr(&nvec,idx+1,&tmp_i));
			}
		}
		pattern
	}

	// returns a list of indices for the access to pref_pattern/nodes!
	fn select_pref_pattern(pref_order:usize,nvec:&Vec<u16>)->Vec<Vec<usize>>{
		let mut prod:u16=1;
		for i in 0..nvec.len(){
			prod*=nvec[i];
		}
		//let nvec:Vec<u16>=self.nvec.clone();
		let mut idx:Vec<Vec<usize>>=vec![];
		let mut block:usize=1;
		let mut iterator:usize=nvec[pref_order] as usize*block;
		if pref_order<nvec.len()-1{
			for i in pref_order+1..nvec.len(){
				block*=nvec[i] as usize;
			}
			iterator*=block;
		}
		println!("nvec:{:?}, block:{}, iterator:{}",nvec,block,iterator);
		//for i in 0..nvec[pref_order]{
		for i in 0..nvec[pref_order]{
			let mut idx_i:Vec<usize>=vec![];
			/*
			let pos:usize=iterator*i as usize;
			//println!(
			for j in 0..block{
			//for j in 0..nvec[pref_order] as usize{
				//if pos+j<prod as usize{
					idx_i.push(pos+j);
				//}
			}
			println!("idx_{}:{:?}",i,idx_i);
			idx.push(idx_i);
			*/
			
			let pos:usize=block*i as usize;
			for j in 0..prod as usize/iterator{
				for k in 0..block{
					idx_i.push(pos+j*iterator+k);
				}
			}
			println!("idx_{}:{:?}",i,idx_i);
			idx.push(idx_i);
			
		}		
		idx
	}
	fn add_vmatrices(&mut self){
		for i in 0..self.pref.len(){
			let mut vm_i:vmatrix=vmatrix::init(i);
			//vmi.order=i;
			println!("self.nvec: {:?}",self.nvec);
			let idx:Vec<Vec<usize>>=Self::select_pref_pattern(i,&self.nvec);
			println!("idx:\n{:?}",idx);
			for j in 0..idx.len(){
				let mut vm_ij:Vec<Option<Rc<RefCell<vertice>>>>=vec![];
				for k in 0..idx[j].len(){
					//println!("idx[{}][{}]:{}, self.nodes.len():{}",j,k,idx[j][k],self.nodes.len());
					vm_ij.push(Some(self.nodes[idx[j][k]].clone()));
				}
				vm_i.nodes.push(vm_ij);
			}
			let vmi_ref:Rc<RefCell<vmatrix>>=Rc::new(RefCell::new(vm_i));
			vmi_ref.borrow_mut().add_selfref(vmi_ref.clone());
			self.vmatrices.push(vmi_ref.clone());
		}
	}
	fn add_vmatrices2(&mut self){
		for i in 0..self.pref.len(){
			let mut vm_i:vmatrix=vmatrix::init(i);
			//vmi.order=i;
			println!("self.nvec: {:?}",self.nvec);
			let idx:Vec<Vec<usize>>=Self::select_pref_pattern(i,&self.nvec);
			println!("idx:\n{:?}",idx);
			let vmi_ref:Rc<RefCell<vmatrix>>=Rc::new(RefCell::new(vm_i));
			vmi_ref.borrow_mut().add_selfref(vmi_ref.clone());
			for j in 0..idx.len(){
				let mut vm_ij:Vec<Option<Rc<RefCell<vertice>>>>=vec![];
				for k in 0..idx[j].len(){
					//println!("idx[{}][{}]:{}, self.nodes.len():{}",j,k,idx[j][k],self.nodes.len());
					//self.nodes[idx[j][k]].borrow_mut().add_matrix(vmi_ref.clone());
					vm_ij.push(Some(self.nodes[idx[j][k]].clone()));
				}
				//vm_i.nodes.push(vm_ij);
				vmi_ref.borrow_mut().nodes.push(vm_ij);
			}
			//let vmi_ref:Rc<RefCell<vmatrix>>=Rc::new(RefCell::new(vm_i));
			self.vmatrices.push(vmi_ref.clone());
		}
	}
	fn add_vmatrices_to_nodes(&mut self){
		for i in 0..self.nodes.len(){
			for j in 0..self.vmatrices.len(){
				self.nodes[i].borrow_mut().matrix.push(self.vmatrices[j].clone());
			}
		}
	}
	fn add_refmatrices_to_vmatrices(&self){
		for i in 0..self.vmatrices.len(){
			for j in 0..self.vmatrices.len(){
				//if i != j{
				if self.vmatrices[i] != self.vmatrices[j]{
					self.vmatrices[i].borrow_mut().add_refmatrix(self.vmatrices[j].clone());
				}
			}
		}
	}
}

// for 2-sided pref
fn read_txt(to_pref: String) -> Vec<Vec<u16>> {
	let path = Path::new(&to_pref);
	println!("path: {:?}", path);
	let file = File::open(&path).expect("file not found");
	let reader = io::BufReader::new(file);
	let mut idx:usize = 0;
	let mut matrix: Vec<Vec<u16>> = vec![];
	let base10: u16 = 10;
	let pattern1: [char;9] = ['1','2','3','4','5','6','7','8','9'];
	let pattern2: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];
	let non_num: [char;5] = ['[',',',' ',']','.'];
	for line in reader.lines() {
		let line = line.expect("Could not read file");
		let mut arr: Vec<u16> = vec![];
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
						let mut num: u16 = 0;
						for k in i..j {
							let digit = (&ychars[k].to_string()).parse::<u16>().unwrap();
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

// for 2-sided pref usize
fn read_txt_usize(to_pref: String) -> Vec<Vec<usize>> {
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

// for N-sided pref
fn read_pref(txtfile:String)->Vec<Vec<[u16;N]>>{
	let mut pref:Vec<Vec<[u16;N]>>=vec![];		
	let path = Path::new(&txtfile);
	//println!("path: {:?}", path);
	let file = File::open(&path).expect("file not found");
	let reader = io::BufReader::new(file);
	for line in reader.lines(){
		let mut pref_i:Vec<[u16;N]>=vec![];
		let mut vecchar:Vec<char>=line.expect("").chars().collect();
		vecchar.remove(0);
		vecchar.remove(vecchar.len()-1);
		let tmp_string:String=vecchar.clone().into_iter().collect();
		for arr in tmp_string.split(']'){
			let mut word:String=arr.to_string();
			let mut word_char:Vec<char>=word.chars().collect();
			if arr.len()>0{
				if word_char[0]==','{
					word=word[1..word.len()].to_string();
					word_char.remove(0);
				}
				if word_char[0]=='['{
					word=word[1..word.len()].to_string();
					word_char.remove(0);
				}
				// for 4-sided matching we have [0,0,0]. for 5-sided matching we have [0,0,0,0].
				let mut el:[u16;N]=[0;N];
				let mut idx:usize=0;
				for part in word.split(','){
					let mut word_part:String=part.to_string();
					if part.contains(&" "){
						word_part=(&word_part[1..word_part.len()]).to_string();
					}
					let digit = word_part.parse::<u16>().unwrap();
					el[idx]=digit;
					idx+=1;
					let part_chars:Vec<char>=word_part.chars().collect();
				}
				pref_i.push(el);
			}
		}
		pref.push(pref_i);			
	}
	println!("PREF:\n{:?}",pref);
	pref
}





#[derive(PartialEq,Eq,Debug,Clone)]
struct delstack{
	nodes:Vec<Rc<RefCell<vertice>>>,
}
impl delstack{
	fn init()->Self{
		Self{
			nodes:vec![],
		}
	}
	fn contains(&self,node:Rc<RefCell<vertice>>)->bool{
		if self.nodes.contains(&node){
			return true;
		}
		false
	}
	fn add_node(&mut self,node:Rc<RefCell<vertice>>){
		self.nodes.push(node);
	}
	fn get_nodes(&self)->Vec<Rc<RefCell<vertice>>>{
		self.print_nodes();
		return self.nodes.clone();
	}
	fn clear_nodes(&mut self){
		self.nodes=vec![];
	}
	fn print_nodes(&self){
		let mut vec:Vec<[u16;N]>=vec![];
		for i in 0..self.nodes.len(){
			vec.push(self.nodes[i].borrow().nodeval);
		}
		println!(" ################################# DELSTACK:{:?}",vec);
	}
}

#[derive(Debug,Eq,PartialEq,Clone)]
struct vmatrix{
	order:usize,
	current_run:bool,
	nodes:Vec<Vec<Option<Rc<RefCell<vertice>>>>>,
	selfref:Option<Rc<RefCell<vmatrix>>>,
	delmem:Option<Rc<RefCell<delstack>>>,
	refmatrices:Vec<Rc<RefCell<vmatrix>>>,
}
impl vmatrix{
	fn init(ord:usize)->Self{
		Self{
			order:ord,
			current_run:false,
			nodes:vec![],
			selfref:None,
			delmem:None,
			refmatrices:vec![],
		}
	}
	fn add_selfref(&mut self,vm_ref:Rc<RefCell<vmatrix>>){
		self.selfref=Some(vm_ref.clone());
	}
	fn add_delmem(&mut self,mem:Rc<RefCell<delstack>>){
		self.delmem=Some(mem);
	}
	fn add_refmatrix(&mut self,refmatrix:Rc<RefCell<vmatrix>>){
		self.refmatrices.push(refmatrix);
	}
	fn del_node(&mut self,delnode:Rc<RefCell<vertice>>){
		println!("### VM DELNODE order: {}, ### node: {:?}, ####",self.order,delnode.borrow().nodeval);
		let id:usize=delnode.borrow().nodeval[self.order] as usize;
		if self.nodes[id].contains(&Some(delnode.clone())){
			// better would be a fixed value
			let pos:usize=self.nodes[id].iter().position(|x| *x==Some(delnode.clone())).unwrap();
			self.nodes[id][pos]=None;
		}
	}
	/*
	fn run(&mut self,nside:&n_side){
		self.current_run=true;
		let d:usize=1;
		for d in 1..self.nodes.len(){
		for i in 0..self.nodes.len(){
			println!("################# d : {} #################",d);
			let nextcol:usize=(i+d)%self.nodes.len();
			for j in 0..self.nodes[i].len(){
				if !self.nodes[i][j].is_none(){
					for k in 0..self.nodes[nextcol].len(){
						if !self.nodes[nextcol][k].is_none(){
							//if n_side::is_stable(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
							if n_side::check_compatibility(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
								if nside.is_stable2(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
									println!("stable!");
									self.nodes[i][j].as_ref().expect("").borrow_mut().add_node(self.order,d-1,self.nodes[nextcol][k].as_ref().expect("").clone());
									self.nodes[nextcol][k].as_ref().expect("").borrow_mut().add_node(self.order,d-1,self.nodes[i][j].as_ref().expect("").clone());
								}
								else{
									println!("not stable!");
								}
							}
							else{
								println!("not compatible!");
							}
						}
					// BE VERY VERY CAREFUL !!!!!!!
					/*
					println!("self.nodes[{}][{}]....nodes[{}-1].len():{} (==?)",i,j,d,
						self.nodes[i][j].as_ref().expect("").borrow().nodes.len());
					}
					if self.nodes[i][j].as_ref().expect("").borrow().nodes[d-1].len()==0{
						self.nodes[i][j].as_ref().expect("").borrow_mut().selfdestroy();
					}
					*/
					}
					let delnodes:Vec<Rc<RefCell<vertice>>>=self.delmem.as_ref().expect("").borrow().get_nodes();
					for k in 0..delnodes.len(){
						self.del_node(delnodes[k].clone());
						for vm in 0..self.refmatrices.len(){
							self.refmatrices[vm].borrow_mut().del_node(delnodes[k].clone());
						}
					}
					self.delmem.as_ref().expect("").borrow_mut().clear_nodes();
					
				}
			}
			for j in 0..self.nodes[nextcol].len(){
				if !self.nodes[nextcol][j].is_none(){
					//if self.nodes[nextcol][j].as_ref().expect("").borrow().nodes.len()==0{
					if self.nodes[nextcol][j].as_ref().expect("").borrow().is_layer_empty(self.order,d-1){
						self.nodes[nextcol][j].as_ref().expect("").borrow_mut().selfdestroy();
					}
				}
			}
			
			let delnodes:Vec<Rc<RefCell<vertice>>>=self.delmem.as_ref().expect("").borrow().get_nodes();
			for k in 0..delnodes.len(){
				self.del_node(delnodes[k].clone());				
				for vm in 0..self.refmatrices.len(){
					self.refmatrices[vm].borrow_mut().del_node(delnodes[k].clone());
				}
			}
			self.delmem.as_ref().expect("").borrow_mut().clear_nodes();
			
		}
		self.print_nodes();
		}
		self.current_run=false;
	}
	*/

	fn run2(&mut self,nside:&n_side){
		self.current_run=true;
		let d:usize=1;
		for d in 1..self.nodes.len(){
		for i in 0..self.nodes.len(){
			println!("################# d : {} #################",d);
			let nextcol:usize=(i+d)%self.nodes.len();
			for j in 0..self.nodes[i].len(){
				if !self.nodes[i][j].is_none(){
					for k in 0..self.nodes[nextcol].len(){
						if !self.nodes[nextcol][k].is_none(){
							//if n_side::is_stable(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
							if n_side::check_compatibility(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
								if nside.is_stable2(self.nodes[i][j].as_ref().expect("").clone(),self.nodes[nextcol][k].as_ref().expect("").clone()){
									println!("stable!");
									self.nodes[i][j].as_ref().expect("").borrow_mut().add_node(self.order,d-1,self.nodes[nextcol][k].as_ref().expect("").clone());
									self.nodes[nextcol][k].as_ref().expect("").borrow_mut().add_node(self.order,d-1,self.nodes[i][j].as_ref().expect("").clone());
								}
								else{
									println!("not stable!");
								}
							}
							else{
								println!("not compatible!");
							}
						}
					// BE VERY VERY CAREFUL !!!!!!!
					/*
					println!("self.nodes[{}][{}]....nodes[{}-1].len():{} (==?)",i,j,d,
						self.nodes[i][j].as_ref().expect("").borrow().nodes.len());
					}
					if self.nodes[i][j].as_ref().expect("").borrow().nodes[d-1].len()==0{
						self.nodes[i][j].as_ref().expect("").borrow_mut().selfdestroy();
					}
					*/
					}
					
					if !self.nodes[i][j].is_none(){
						//if self.nodes[nextcol][j].as_ref().expect("").borrow().nodes.len()==0{
						if self.nodes[i][j].as_ref().expect("").borrow().is_layer_empty(self.order,d-1){
							//self.nodes[i][j].as_ref().expect("").borrow_mut().selfdestroy();
							self.nodes[i][j].as_ref().expect("").borrow_mut().selfdestroy(self.order,d-1);
						}
					}

					
					let delnodes:Vec<Rc<RefCell<vertice>>>=self.delmem.as_ref().expect("").borrow().get_nodes();
					for k in 0..delnodes.len(){
						self.del_node(delnodes[k].clone());
						for vm in 0..self.refmatrices.len(){
							self.refmatrices[vm].borrow_mut().del_node(delnodes[k].clone());
						}
					}
					self.delmem.as_ref().expect("").borrow_mut().clear_nodes();
					
				}
			}
			for j in 0..self.nodes[nextcol].len(){
				if !self.nodes[nextcol][j].is_none(){
					//if self.nodes[nextcol][j].as_ref().expect("").borrow().nodes.len()==0{
					if self.nodes[nextcol][j].as_ref().expect("").borrow().is_layer_empty(self.order,d-1){
						//self.nodes[nextcol][j].as_ref().expect("").borrow_mut().selfdestroy();
						self.nodes[nextcol][j].as_ref().expect("").borrow_mut().selfdestroy(self.order,d-1);
					}
				}
			}
			
			let delnodes:Vec<Rc<RefCell<vertice>>>=self.delmem.as_ref().expect("").borrow().get_nodes();
			for k in 0..delnodes.len(){
				self.del_node(delnodes[k].clone());				
				for vm in 0..self.refmatrices.len(){
					self.refmatrices[vm].borrow_mut().del_node(delnodes[k].clone());
				}
			}
			self.delmem.as_ref().expect("").borrow_mut().clear_nodes();
			
		}
		self.print_nodes();
		}
		self.current_run=false;
	}
	
	fn print_nodes(&self){
		println!("PRINT NODES (VMATRIX)");
		for i in 0..self.nodes.len(){
			print!("{}:  ",i);
			for j in 0..self.nodes[i].len(){
				if self.nodes[i][j].is_none(){
					print!("   ");
				}
				else{
					print!("{}, ",j);
				}
			}
			println!();
		}
	}
}

#[derive(Debug,Eq,PartialEq)]
struct vertice{
	nodes:Vec<Vec<Vec<Rc<RefCell<vertice>>>>>,
	selfref:Option<Rc<RefCell<vertice>>>,
	marked:bool,
	nodeval:[u16;N],
	matrix:Vec<Rc<RefCell<vmatrix>>>,
	delmem:Option<Rc<RefCell<delstack>>>,
}
impl vertice{
	fn init()->Self{
		Self{
			nodes:vec![],
			selfref:None,
			marked:false,
			nodeval:[0;N],
			matrix:vec![],
			delmem:None,
		}
	}
	fn init_nodes(&mut self){
		
	}
	
	fn add_node(&mut self,vm:usize,layer:usize,addnode:Rc<RefCell<vertice>>){
		//println!("vm:{}, layer:{}, self.nodes.len:{}",vm,layer,self.nodes.len());
		// be careful here!
		println!("add_node->vm:{},layer:{},self.nodes.len():{}",vm,layer,self.nodes.len());
		if vm==0 && layer==3{
			println!("add_node->vm:{},layer:{},self.nodes[{}].len():{}",vm,layer,vm,self.nodes[vm].len());
			//println!("add_node->vm:{},layer:{},self.nodes[{}][{}].len():{}",vm,layer,vm,layer,self.nodes[vm][layer].len());
		}
		if self.nodes.len()==vm{
			self.nodes.push(vec![vec![addnode.clone()]]);
		}
		else{
			if self.nodes[vm].len()==layer{
				self.nodes[vm].push(vec![addnode.clone()]);
			}
			else{
				println!("vm:{}, layer:{}, self.nodes[{}].len:{}",vm,layer,vm,self.nodes[vm].len());
				self.nodes[vm][layer].push(addnode.clone());
			}
		}
	}
	
	fn add_selfref(&mut self,vref:Rc<RefCell<vertice>>){
		self.selfref=Some(vref.clone());
	}
	fn add_matrix(&mut self,vm_ref:Rc<RefCell<vmatrix>>){
		self.matrix.push(vm_ref.clone());
	}
	fn add_delmem(&mut self,mem:Rc<RefCell<delstack>>){
		self.delmem=Some(mem);
	}
	fn is_layer_empty(&self,vm:usize,layer:usize)->bool{
		println!("is_layer_empty-> vm:{}, layer:{}",vm,layer);
		self.print_nodes();
		let mut empty:bool=false;
		for i in 0..vm{
			for j in 0..self.nodes[vm].len(){
				if self.nodes[i][j].len()==0{
					return true;					
				}
			}
		}
		if self.nodes.len()==0{
			return true;
		}
		if self.nodes[vm].len()>0{
			if self.nodes[vm].len()==layer{
				return true;
			}
			for i in 0..layer+1{
				println!("vm:{}, layer:{}, self.nodes[{}].len()={}",vm,layer,vm,self.nodes[vm].len());
				if self.nodes[vm][i].len()==0{
					return true;
				}
			}
		}
		else{
			return true;
		}
		false
	}
	fn print_nodes(&self){
		println!("### PRINT NODES of {:?}: ############",self.nodeval);
		let mut nodes:Vec<Vec<Vec<[u16;N]>>>=vec![];
		for vm in 0..self.nodes.len(){
			println!("vm:{}",vm);
			let mut nodes_vm:Vec<Vec<[u16;N]>>=vec![];
			for i in 0..self.nodes[vm].len(){
				let mut nodes_layer:Vec<[u16;N]>=vec![];
				for j in 0..self.nodes[vm][i].len(){
					nodes_layer.push(self.nodes[vm][i][j].borrow().nodeval);
				}
				nodes_vm.push(nodes_layer);
			}
			nodes.push(nodes_vm);
		}
		for vm in 0..nodes.len(){
			println!("### NODES VM:{} ####",vm);
			for i in 0..nodes[vm].len(){
				println!("layer: {} -> {:?}",i,nodes[vm][i]);
			}
			println!();
		}
	}
	/*
	fn delete_node1(&mut self,delnode:Rc<RefCell<vertice>>){
		println!("#### DELETE NODE self: {:?}, #### node: {:?} ######",self.nodeval,delnode.borrow().nodeval);
		for i in 0..self.nodes.len(){
			for j in 0..self.nodes[i].len(){
				if self.nodes[i][j].contains(&delnode){
					let pos:usize=self.nodes[i][j].iter().position(|x| *x==delnode).unwrap();
					self.nodes[i][j].remove(pos);
					if self.nodes[i][j].len()==0{
						self.selfdestroy();
					}
				}
			}
		}
	}
	*/
	//fn delete_node(&mut self,delnode:Rc<RefCell<vertice>>){
	fn delete_node(&mut self,delnode:Rc<RefCell<vertice>>,vm:usize,layer:usize){
		if !self.delmem.as_ref().expect("").borrow().contains(delnode.clone()){
		//if !self.delmem.as_ref().expect("").borrow().contains(self.selfref.as_ref().expect("").clone()){
			println!("#### DELETE NODE self: {:?}, #### node: {:?} ######",self.nodeval,delnode.borrow().nodeval);
			for i in 0..self.nodes.len(){
				for j in 0..self.nodes[i].len(){
					if self.nodes[i][j].contains(&delnode){
						let pos:usize=self.nodes[i][j].iter().position(|x| *x==delnode).unwrap();
						self.nodes[i][j].remove(pos);
						//if self.nodes[i][j].len()==0{
						if self.is_layer_empty(vm,layer){
							self.selfdestroy(vm,layer);
						}
					}
				}
			}
		}
	}

	//fn selfdestroy(&mut self){
	fn selfdestroy(&mut self,vm:usize,layer:usize){
		println!("#########################################");
		println!("############ SELF DESTROY !!!!!!! #######");
		println!("############ NODE:    {:?} ############",self.nodeval);
		println!("#########################################");
		let mut included:bool=false;
		if !self.delmem.as_ref().expect("").borrow().contains(self.selfref.as_ref().expect("").clone()){
			included=true;
		}
		self.delmem.as_ref().expect("").borrow_mut().add_node(self.selfref.as_ref().expect("").clone());
		self.marked=true;
		
		for i in 0..self.nodes.len(){
			for j in 0..self.nodes[i].len(){
				for k in 0..self.nodes[i][j].len(){
					if !self.nodes[i][j][k].borrow().marked{
						println!("del next node!");
						self.nodes[i][j][k].borrow_mut().delete_node(self.selfref.clone().expect(""),vm,layer);
					}
				}
			}
		}
		// tell matrices to delete oneself!
		println!("vertice -> VM len: {}",self.matrix.len());
		for i in 0..self.matrix.len(){
			println!("tell matrix i:{}",i);
			/*
			println!("current run:{}",self.matrix[i].borrow().current_run);
			if !self.matrix[i].borrow().current_run{
				
				self.matrix[i].borrow_mut().del_node(self.selfref.clone().expect(""));
			}
			*/
		}
	}
}


// NOT FINISHED !!!!!!!
fn test_get_bits(){
	let n:u16=3;
	let bigN:u16=4;
	let bits:Vec<Vec<u16>>=get_all_N_bits(n,bigN,&vec![]);
	print_matrix(&bits,bigN);
	println!("size of bits: {}",bits.len());
	let bits_del:Vec<Vec<u16>>=delete_imcompatibility(&bits,n,bigN);
	print_matrix(&bits_del,bigN);
	println!("size of bits_del: {}",bits_del.len());	
}

//fn get_all_N_bits(n:usize,bigN:usize,tmp:&Vec<usize>){
fn get_all_N_bits(n:u16,bigN:u16,tmp:&Vec<u16>)->Vec<Vec<u16>>{
	let mut cn:u16=0;
	//println!("{:?}",tmp);
	for i in 0..tmp.len(){
		if tmp[i]==1{
			cn+=1;
		}
		if cn==bigN{
			let mut new_tmp:Vec<u16>=tmp.clone();
			// same output?
			//if (tmp.len() as u16)<n.pow(bigN.into())-1{
			if (tmp.len() as u16)<n.pow(bigN.into()){
				for i in tmp.len()..(n*bigN) as usize{
					new_tmp.push(0);
				}
			}
			return vec![new_tmp];
		}
	}
	let mut res:Vec<Vec<u16>>=vec![];
	let mut tmp0:Vec<u16>=tmp.clone();
	tmp0.push(0);
	let mut tmp1:Vec<u16>=tmp.clone();
	tmp1.push(1);
	//if n*bigN>=(bigN-cn)+tmp.len() as u16{
	if n*bigN>(bigN-cn)+tmp.len() as u16{
	//if tmp.len()<(n*bigN) as usize{
		res.append(&mut get_all_N_bits(n,bigN,&tmp0));
	}
	res.append(&mut get_all_N_bits(n,bigN,&tmp1));
	res
}
fn delete_imcompatibility(input:&Vec<Vec<u16>>,n:u16,bigN:u16)->Vec<Vec<u16>>{
	let mut output:Vec<Vec<u16>>=input.clone();
	let mut del:Vec<usize>=vec![];
	let mut del_row:Vec<usize>=vec![];
	for i in 0..input.len(){
		let mut idx:Vec<usize>=vec![];
		for j in 0..input[i].len(){
			if input[i][j]==1{
				idx.push(j);
			}
		}
		//println!("i:{}, idx:{:?}",i,idx);
		for j in 0..idx.len()-1{
			let mut not_ok:bool=false;
			for k in j+1..idx.len(){
				//if idx[j]+n==idx[k]{
				
				// VERY IMPORTANT CODE !!!!!!!
				// FOR SAW PATTERN IN STABM (REMOVING NODES) !!!!!!!
				
				//if (idx[k]-idx[j]) as u16%n==0{
				if (idx[k]-idx[j]) as u16%bigN==0{
					//println!("del first i:{}",i);
					del.insert(0,i);
					not_ok=true;
					break;
				}
			}
			if not_ok{
				break;
			}
			// please take care of variable not_ok!
			else{
				//println!("not_ok:{:?}, i{}",not_ok,i);
				if (idx[j])%bigN as usize==0{
					let mut cn_row:u16=0;
					for k in 0..bigN{
						if input[i][idx[j]+k as usize]==1{
							cn_row+=1;
						}
						else{
							break;
						}
					}
					if cn_row==bigN{
						//println!("delete i:{}",i);
						del.insert(0,i);
						//not_ok=true;
						break;
					}
				}
			}
		}
	}
	//println!("{:?}",del);
	for i in 0..del.len(){
		output.remove(del[i]);
	}
	output
}
// a little contradiction between bigN and N because N is already defined at the top of the code!
/*
fn get_all_nodes_pattern(n:u16,bigN:u16)->Vec<Vec<[u16;N]>>{
	
}
*/
//fn print_matrix<T: std::fmt::Debug>(mat:&Vec<Vec<T>>){
fn print_matrix(mat:&Vec<Vec<u16>>,bigN:u16){
	for i in 0..mat.len(){
		//println!("{:?}",mat[i]);
		//println!("{}\t{:?}",i,print_row_without_zero(&mat[i],bigN));
		println!("{:?}",print_row_without_zero(&mat[i],bigN));
	}
}
fn print_row_without_zero(vec:&Vec<u16>,bigN:u16)->String{
	let mut txt:String="".to_string();
	for i in 0..vec.len(){
		if i as u16%bigN==0{
			txt+="|";
		}
		if vec[i]==1{
			txt+="1";
		}
		else{
			txt+=" ";
		}
	}
	txt+="|";
	txt
}
// 3x3: -> 3x 6 (2 fields) + 6 (3 fields)


// transform u16 matrix to usize matrix
fn u16_2_usize(matrix:&Vec<Vec<u16>>)->Vec<Vec<usize>>{
	let mut mat:Vec<Vec<usize>>=vec![];
	for i in 0..matrix.len(){
		let mut mat_i:Vec<usize>=vec![];
		for j in 0..matrix[i].len(){
			mat_i.push(matrix[i][j] as usize);
		}
		mat.push(mat_i);
	}
	mat
}
fn display_adj(adj:&Vec<Vec<[usize;2]>>){
	println!("DISPLAY ADJACENT MATRIX!");
	for i in 0..adj.len(){
		for j in 0..adj[i].len(){
			print!("{:?}\t",adj[i][j]);
		}
		println!();
	}
}
fn display_pref(pref:&Vec<Vec<usize>>){
	println!("DISPLAY PREF LISTS!");
	for i in 0..pref.len(){
		for j in 0..pref[i].len(){
			print!("{:?}\t",pref[i][j]);
		}
		println!();
	}	
}
fn display_bp(bp:&Vec<Vec<bool>>){
	println!("DISPLAY STABILITY MATRIX");
	for i in 0..bp.len(){
		for j in 0..bp[i].len(){
			if bp[i][j]{
				print!("1");
			}
			else{
				print!("0");
			}
			//print!("{:?}\t",bp[i][j]);
		}
		println!();
	}	
}
fn img_bp_matrix(bp_matrix:&Vec<Vec<bool>>,path:String)->RgbImage{
	let w_:u32=777;
	let h_:u32=777;
	let n:usize=bp_matrix.len();
	let pw:usize=(w_ as f64/n as f64) as usize;
	let ph:usize=(h_ as f64/n as f64) as usize;
	let w:u32=(pw*n) as u32;
	let h:u32=(ph*n) as u32;
	//let (highest,h_coord)=highest(&adj);
	//let fc_col:f64=255 as f64/highest;
		
	let mut img:RgbImage=RgbImage::new(w,h);
	img=fill_pic(&img);
	for i in 0..bp_matrix.len(){
		let y_i:usize=ph*i;
		for j in 0..bp_matrix[i].len(){
			let x_j:usize=pw*j;
			//let val:f64=adj[i][j];
			let mut val:u8=0;
			if bp_matrix[i][j]{
				val=1;
			}
			let col_ij=Rgb([0,(120)*val,0]);
			
			
			//let col_ij=Rgb([0,(val*fc_col) as u8,0]);
			for k in 0..ph{
				let y:u32=(y_i+k) as u32;
				for m in 0..pw{
					let x:u32=(x_j+m) as u32;
					img.put_pixel(x,y,col_ij);
				}
			}
		}
	}
	//img.save("pic/adj_val.png");
	//img.save("pic/bp_matrix.png");
	img.save(path);
	img
}
fn fill_pic(rgb:&RgbImage)->RgbImage{
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

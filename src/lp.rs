#![allow(warnings)]
use highs::{RowProblem,Col,Sense};
use super::helper::{IOClass};



pub struct many2one{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	
	quota:Vec<usize>,
	matches:Vec<Vec<usize>>,
	c:Vec<i8>,
	A_eq:Vec<Vec<usize>>,
	b_eq:Vec<usize>,
	A_ub:Vec<Vec<i8>>,
	b_ub:Vec<usize>,
}
impl many2one{
	pub fn new()->Self{
		Self{
			//m:vec![],
			//m:read_txt_usize("many_pref/s.txt".to_string()),
			m:IOClass::read_txt("manyone_pref/schools.txt".to_string()),
			//w:vec![],
			//w:read_txt_usize("many_pref/i.txt".to_string()),
			w:IOClass::read_txt("manyone_pref/students.txt".to_string()),
			quota:IOClass::read_txt("manyone_pref/quota_schools.txt".to_string())[0].clone(),
			//q:vec![2,2,3,3],
			matches:vec![],
			c:vec![],
			A_eq:vec![],
			b_eq:vec![],
			A_ub:vec![],
			b_ub:vec![],
		}
	}
	pub fn update(&mut self){
		//self.complete_lists_123_add1_mw();
		self.print_pref();
	}
	pub fn print_pref(&self){
		println!("MEN / STUDENTS");
		for i in 0..self.m.len(){
			println!("{:?}",self.m[i]);
		}
		println!("WOMEN / SCHOOLS");
		for i in 0..self.w.len(){
			println!("{:?}",self.w[i]);
		}

	}
	/*
	// behind the main pref lists, the dummy is also added!
	fn complete_lists_123_add1(&mut self){
		//let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		//if consistency{
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
		//}
		//else{
			// ???????
		//}
		self.qu_sc.push(1);
		self.qu_st.push(1);
	}
	*/
	// behind the main pref lists, the dummy is also added!
	fn complete_lists_123_add1_mw(&mut self){
		//let (consistency,sc_xlen,st_xlen)=self.check_list_consistency();
		//if self.check_list_consistency().0{
		//if consistency{
			let st_last:usize=self.m.len() as usize;
			//let st_last:u16=self.st.len() as u16;
			let sc_last:usize=self.w.len() as usize;
			for i in 0..self.w.len(){				
				for j in self.w[i].len()-1 as usize..(st_last) as usize{
					self.w[i].push(st_last);
				}
			}
			//let lastsc:Vec<u16>=vec![st_last;(st_last+1) as usize];
			let mut lastsc:Vec<usize>=vec![st_last];
			for i in 0..self.m.len(){
				lastsc.push(i as usize)
			}
			//self.sc.push(lastsc);
			for i in 0..self.m.len(){				
				//for j in self.st[i].len()-1 as usize..(sc_last-1) as usize{
				for j in self.m[i].len()-1 as usize..(sc_last) as usize{
					self.m[i].push(sc_last);
				}
			}
			//let lastst:Vec<u16>=vec![sc_last;sc_last as usize];
			let mut lastst:Vec<usize>=vec![sc_last];
			for i in 0..self.w.len(){
				lastst.push(i as usize);
			}
		
			self.w.push(lastsc);
			self.m.push(lastst);
		//}
		//else{
			// ???????
		//}
		//self.qu_sc.push(1);
		self.quota.push(1);
		//self.qu_st.push(1);
	}
	fn get_vertices(&self)->Vec<Vec<usize>>{
		let mut vertices:Vec<Vec<usize>>=vec![];
		//let lenm:usize=self.m.len();
		//let lenw:usize=self.w.len();
		for i in 0..self.m.len(){
			//for j in 0..self.w.len(){
			for j in 0..self.m[i].len(){
				//vertices.push(vec![i,j]);
				vertices.push(vec![i,self.m[i][j]]);
				//vertices.push(vec![j,i]);
			}
		}
		vertices
	}
	fn get_id_vertices(mi:usize,wj:usize,mlen:usize,wlen:usize)->usize{
		//return mi*mlen+wj;
		return mi*wlen+wj;
	}
	fn stability_rothblum_A_le(&self)->Vec<Vec<i8>>{
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		let mut empty:Vec<i8>=vec![0;mlen*wlen];
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			println!("mi:{}, wi:{}, vi:{:?}",mi,wi,vi);
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*wlen+wj]=1;
			}
			for j in 0..posm{
				let mj:usize=self.w[wi][j];
				empty_vi[mj*wlen+wi]=1;
			}
			A_el.push(empty_vi);						
		}
		A_el
	}
	pub fn all_poss_idx_for_comb(pos:usize,q:usize,idx:usize,tmp:&Vec<usize>)->Vec<Vec<usize>>{
		let mut all_poss:Vec<Vec<usize>>=vec![];
		if idx==pos || tmp.len()==q{
			if tmp.len()==q{
				return vec![tmp.to_vec()];
			}
			else{
				return vec![];
			}
		}
		else{
			for i in idx..pos{
				let mut new_tmp:Vec<usize>=tmp.clone();
				new_tmp.push(i);
				let result:Vec<Vec<usize>>=Self::all_poss_idx_for_comb(pos,q,i+1,&new_tmp);
				for j in 0..result.len(){
					if result[j].len()>0{
						all_poss.push(result[j].clone());
					}
				}
			}
		}
		
		all_poss
	}
	pub fn stability_sethumaran_A_le(&self)->(Vec<Vec<i8>>,Vec<i8>,Vec<[usize;2]>,Vec<Vec<usize>>){
		let mut A_el_vec:Vec<i8>=vec![];
		let mut check_vec:Vec<[usize;2]>=vec![];
		let mut check_poss:Vec<Vec<usize>>=vec![];
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		let mut empty:Vec<i8>=vec![0;mlen*wlen];
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			println!("mi:{}, wi:{}, vi:{:?}",mi,wi,vi);
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			// ASSUMPTION: MEN ARE SCHOOLS AND WOMEN ARE STUDENTS! OTHERWISE, IT WILL NOT WORK!
			let mut A_le_mi:Vec<Vec<i8>>=vec![];
			if posw+1<self.quota[mi]{
				for i in 0..posw+1{
					let comb_wi:usize=self.m[mi][i];
					let posm_comb:usize=self.w[comb_wi].iter().position(|x| *x==mi).unwrap();
					for j in 0..posm_comb+1{
						let comb_mij:usize=self.w[comb_wi][j];
						empty_vi[comb_mij*wlen+comb_wi]=1;
					}
					empty_vi[mi*wlen+comb_wi]=1;
				}
				check_vec.push([mi,wi]);
				check_poss.push(vec![]);
				A_el_vec.push(self.quota[mi] as i8);
				A_el.push(empty_vi);						
			}
			else{
				
				// REMARK: THIS IS FOR THE RETRIEVAL OF ALL POSSIBLE COMBS FULFILLING THE QUOTA!
				//let all_poss:Vec<Vec<usize>>=Self::all_poss_idx_for_comb(posw+1,self.quota[mi],0,&vec![]);
				let all_poss:Vec<Vec<usize>>=Self::all_poss_idx_for_comb(posw,self.quota[mi]-1,0,&vec![]);
				println!("ALL POSS:{:?}",all_poss);
				for k in 0..all_poss.len(){
					//for i in 0..posw+1{
					let mut empty_vi_comb:Vec<i8>=empty_vi.clone();
					for i in 0..self.quota[mi]-1{
						let comb_wi:usize=self.m[mi][all_poss[k][i]];
						let posm_comb:usize=self.w[comb_wi].iter().position(|x| *x==mi).unwrap();
						for j in 0..posm_comb+1{
							let comb_mij:usize=self.w[comb_wi][j];
							empty_vi_comb[comb_mij*wlen+comb_wi]=1;
						}
						empty_vi_comb[mi*wlen+comb_wi]=1;
					}
					empty_vi_comb[mi*wlen+wi]=1;
					
					check_poss.push(all_poss[k].clone());
					check_vec.push([mi,wi]);
					A_el_vec.push(self.quota[mi] as i8);
					A_el.push(empty_vi_comb.clone());											
				}
				
			}					
		}
		(A_el,A_el_vec,check_vec,check_poss)
	}	
	pub fn stability_balinski_A_le_lemma1(&self)->(Vec<Vec<i8>>,Vec<i8>,Vec<[usize;2]>,Vec<Vec<usize>>){
		let mut A_el_vec:Vec<i8>=vec![];
		let mut check_vec:Vec<[usize;2]>=vec![];
		let mut check_poss:Vec<Vec<usize>>=vec![];
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		let mut empty:Vec<i8>=vec![0;mlen*wlen];
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			println!("mi:{}, wi:{}, vi:{:?}",mi,wi,vi);
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			// ASSUMPTION: MEN ARE SCHOOLS AND WOMEN ARE STUDENTS! OTHERWISE, IT WILL NOT WORK!
			let mut A_le_mi:Vec<Vec<i8>>=vec![];
			//if posw+1<self.quota[mi]{
			for i in 0..posw{
				let wij:usize=self.m[mi][i];
				//let posm_comb:usize=self.w[wij].iter().position(|x| *x==mi).unwrap();
				/*
				for j in 0..posm_comb+1{
					let comb_mij:usize=self.w[comb_wi][j];
					empty_vi[comb_mij*wlen+comb_wi]=1;
				}
				*/
				empty_vi[mi*wlen+wij]=1;
			}
			for i in 0..posm+1{
				let mij:usize=self.w[wi][i];
				empty_vi[mij*wlen+wi]=self.quota[mi] as i8;
			}
			check_vec.push([mi,wi]);
			check_poss.push(vec![]);
			A_el_vec.push(self.quota[mi] as i8);
			A_el.push(empty_vi);						
			//}
			/*
			else{
				
				// REMARK: THIS IS FOR THE RETRIEVAL OF ALL POSSIBLE COMBS FULFILLING THE QUOTA!
				//let all_poss:Vec<Vec<usize>>=Self::all_poss_idx_for_comb(posw+1,self.quota[mi],0,&vec![]);
				let all_poss:Vec<Vec<usize>>=Self::all_poss_idx_for_comb(posw,self.quota[mi]-1,0,&vec![]);
				println!("ALL POSS:{:?}",all_poss);
				for k in 0..all_poss.len(){
					//for i in 0..posw+1{
					let mut empty_vi_comb:Vec<i8>=empty_vi.clone();
					for i in 0..self.quota[mi]-1{
						let comb_wi:usize=self.m[mi][all_poss[k][i]];
						let posm_comb:usize=self.w[comb_wi].iter().position(|x| *x==mi).unwrap();
						for j in 0..posm_comb+1{
							let comb_mij:usize=self.w[comb_wi][j];
							empty_vi_comb[comb_mij*wlen+comb_wi]=1;
						}
						empty_vi_comb[mi*wlen+comb_wi]=1;
					}
					empty_vi_comb[mi*wlen+wi]=1;
					
					check_poss.push(all_poss[k].clone());
					check_vec.push([mi,wi]);
					A_el_vec.push(self.quota[mi] as i8);
					A_el.push(empty_vi_comb.clone());											
				}
				
			}
			*/					
		}
		(A_el,A_el_vec,check_vec,check_poss)
	}	


	pub fn prepare_LP_highs_many2one(&mut self)->Vec<usize>{
		println!("prepare_LP_highs");
		let (c,b_ub,b_eq,bounds)=self.basic_data();
		//let A_eq=self.get_a_eq();
		let (sc_A_eq,st_A_eq)=self.get_a_eq();
		//let A_ub=self.pref_minus();
		//let A_le=self.stability_rothblum_A_le();
		//let (A_le,A_le_vec,check_vec,check_poss)=self.stability_sethumaran_A_le();
		let (A_le,A_le_vec,check_vec,check_poss)=self.stability_balinski_A_le_lemma1();
		println!("st_A_eq:\n{:?}",sc_A_eq);
		println!("A_LE");
		for i in 0..A_le.len(){
			println!("{:?}",A_le[i]);
		}
		let mut pb = RowProblem::new();		
		let mut vars:Vec<Col>=vec![];
		for i in 0..c.len(){
			vars.push(pb.add_column(1.0,0..));
		}
		for i in 0..st_A_eq.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..st_A_eq[i].len(){
				row.push((vars[j],st_A_eq[i][j] as f64));
			}
			//pb.add_row(..=1,&row);	
			pb.add_row(..=1,&row);	
			//pb.add_row(..=b_eq[i] as f64,&row);	
		}
		for i in 0..sc_A_eq.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..sc_A_eq[i].len(){
				row.push((vars[j],sc_A_eq[i][j] as f64));
			}
			//pb.add_row(..=1,&row);	
			//pb.add_row(..=b_eq[i] as f64,&row);	
			pb.add_row(..self.quota[i] as f64,&row);	
		}
		//for i in 0..A_ub.len(){
		for i in 0..A_le.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			//for j in 0..A_ub[i].len(){
			for j in 0..A_le[i].len(){
				//row.push((vars[j],A_ub[i][j] as f64));
				row.push((vars[j],A_le[i][j] as f64));
			}
			//pb.add_row(..0,&row);	
			
			//pb.add_row(..1,&row);	
			//pb.add_row(..A_le_vec[i],&row);	
			pb.add_row(..self.quota[i/self.w.len()] as f64,&row);	
		}
		println!("MATCHING LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);
		
		let matchres:Vec<usize>=self.transform_to_match(solution.columns());
		println!("MATCH RESULT:\n{:?}",matchres);
		matchres
	}
	
	fn pref_minus(&self)->Vec<Vec<i8>>{
		println!("Calculating A_ub");
		let mut A_ub:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		//for i in 0..v.len(){
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		for vi in &v{
			/*
			let mut empty:Vec<i8>=vec![];
			for i in 0..v.len(){
				empty.push(0);
			}
			*/
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).expect("");
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*mlen+wj]-=1;
			}
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			if posm<mlen-1{
				for j in posm+1..mlen{
					let mj:usize=self.w[wi][j];
					//empty_vi[mj*mlen+wi]+=1;
					empty_vi[mj*wlen+wi]+=1;
				}
			}
			A_ub.push(empty_vi);
		}
		A_ub
	}
	fn basic_data(&self)->(Vec<i8>,Vec<usize>,Vec<usize>,Vec<usize>){
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let n:usize=mlen*wlen;
		let mut c:Vec<i8>=vec![];
		let mut b_ub:Vec<usize>=vec![];
		let mut b_eq:Vec<usize>=vec![];
		let mut bounds:Vec<usize>=vec![];
		for i in 0..n{
			//c.push(-1);
			// BE VERY CAREFUL !!!!!!!
			c.push(-1);
			b_ub.push(0);
			bounds.push(0);
		}
		for i in 0..2*mlen{
			b_eq.push(1);
		}
		(c,b_ub,b_eq,bounds)
	}
	fn capacity_constraints(&self){
		
	}
	fn save_param(&mut self){
		let (c,b_ub,b_eq,bounds)=self.basic_data();
		//let A_eq:Vec<Vec<usize>>=self.get_a_eq();
		// BE CAREFUL !!! THINK of sc_A_eq instead of just A_eq!!!!!!!
		//let (A_eq,sc_Aeq):Vec<Vec<usize>>=self.get_a_eq();
		let (A_eq,sc_Aeq):(Vec<Vec<usize>>,Vec<Vec<usize>>)=self.get_a_eq();
		let A_ub:Vec<Vec<i8>>=self.pref_minus();
		self.A_eq=A_eq.clone();
		self.b_eq=b_eq.clone();
		self.A_ub=A_ub.clone();
		self.b_ub=b_ub.clone();
		
		let mut c_tmp:Vec<i8>=vec![];
		// This is due to flexibility to min-max-adjustments in c!
		// Comment if c should be negative!
		for i in 0..c.len(){
			c_tmp.push(1);
		}
		self.c=c_tmp.clone();
	}
	fn get_a_eq(&self)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		println!("Calculating A_eq");
		let mlen:usize=self.m.len(); 
		let wlen:usize=self.w.len();
		let n:usize=mlen;
		let mut A_eq:Vec<Vec<usize>>=vec![];
		let mut sc_A_eq:Vec<Vec<usize>>=vec![];
		let mut st_A_eq:Vec<Vec<usize>>=vec![];
		let mut empty:Vec<usize>=vec![];
		//for i in 0..n*n{
		for i in 0..mlen*wlen{
			empty.push(0);
		}
		//for i in 0..n{
		for i in 0..mlen{
			let mut empty_i:Vec<usize>=empty.clone();
			//for j in 0..n{
			for j in 0..wlen{
				//empty_i[i*n+j]=1;
				empty_i[i*wlen+j]=1;
			}
			A_eq.push(empty_i.clone());
			sc_A_eq.push(empty_i);
		}
		//for i in 0..n{
		for i in 0..wlen{
			let mut empty_i:Vec<usize>=empty.clone();
			//for j in 0..n{
			for j in 0..mlen{
				//empty_i[j*n+i]=1;
				empty_i[j*wlen+i]=1;
			}
			A_eq.push(empty_i.clone());
			st_A_eq.push(empty_i);
		}
		//A_eq
		(sc_A_eq,st_A_eq)
	}
	fn transform_to_match(&self,input:&[f64])->Vec<usize>{
		let mut matchres:Vec<usize>=vec![];
		//let n:usize=self.m.len();
		let n:usize=self.w.len();
		for i in 0..n{
			matchres.push(0);
		}
		for i in 0..input.len(){
			if input[i]==1.0{
				let m:usize=i/n;
				let w:usize=i%n;
				//matchres[m]=w;
				matchres[w]=m;
			}
		}
		println!("MATCH: {:?}",matchres);
		matchres
	}
}

pub struct one2one{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	adj:Vec<Vec<[usize;2]>>,
	bp:Vec<Vec<bool>>,
	A_eq:Vec<Vec<usize>>,
	A_ub:Vec<Vec<i8>>,
	pub pb:RowProblem,
	matches:Vec<Vec<usize>>,
	current_match:Vec<usize>,
}
impl one2one{
	pub fn init()->Self{
		let m_:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
		let w_:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
		let adj_:Vec<Vec<[usize;2]>>=IOClass::create_adj(&m_,&w_);
		let bp_:Vec<Vec<bool>>=IOClass::bp_matrix(&adj_);
		let lphelper:LPHelper=LPHelper::init(&m_,&w_);
		let A_eq_:Vec<Vec<usize>>=lphelper.get_a_eq();
		let A_ub_:Vec<Vec<i8>>=lphelper.stability_rothblum_A_le();
		Self{
			m:m_,
			w:w_,
			adj:adj_,
			bp:bp_,
			A_eq:A_eq_,
			A_ub:A_ub_,
			pb:RowProblem::new(),
			matches:vec![],
			current_match:vec![],
		}
	}
	pub fn update(&mut self){
		self.pb=self.prepare_lp_problem();
	}
	pub fn prepare_lp(){
		

	}
	//pub fn stability_bp_LP_HIGHS_IMPORTANT_fractional_test()->Vec<usize>{
	pub fn run_lp(&self)->Vec<usize>{
		println!("####### STABILITY BP HIGHS FRACTIONAL TEST");
		println!("INTEGER SOLUTION BUT NO FRACTIONAL SOLUTION UNFORTUNATELY!");
		let mut matchLP:matchingLP=matchingLP::new();
		let m:Vec<Vec<usize>>=matchLP.m.clone();
		let w:Vec<Vec<usize>>=matchLP.w.clone();
		//let adj:Vec<Vec<[usize;2]>>=create_adj(&m,&w);
		let adj:Vec<Vec<[usize;2]>>=IOClass::create_adj(&m,&w);
		//let A_ub:Vec<Vec<usize>>=bp_matrix_usize(&adj);
		let A_ub:Vec<Vec<i8>>=matchLP.stability_rothblum_A_le();
		//let (mut c,mat):(Vec<i8>,Vec<Vec<i8>>)=self.fractional_setup_rothblum93_3();
		let A_eq:Vec<Vec<usize>>=matchLP.get_a_eq();
		//let (m_A_eq,w_A_eq):(Vec<Vec<usize>>,Vec<Vec<usize>>)=matchLP.get_a_eq_2();
		//let m_A_eq_toggle:Vec<Vec<usize>>=toggle_matrix(&m_A_eq);
		//let w_A_eq_toggle:Vec<Vec<usize>>=toggle_matrix(&w_A_eq);
		let A_eq_toggle:Vec<Vec<usize>>=toggle_matrix(&A_eq);
		let mlen:usize=m.len();
		let wlen:usize=w.len();
		let c:Vec<usize>=vec![1;m.len()*w.len()];
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
		// the off vector holds if we have a quadratic matrix with n x n rows/columns!
		//let mut off:Vec<usize>=vec![];
		for i in 0..A_ub.len(){
			//if !off.contains(&i){
				let mut row:Vec<(Col,f64)>=vec![];
				for j in 0..A_ub[i].len(){
					row.push((vars[j],A_ub[i][j] as f64));
				}
				pb.add_row(1.0..,&row);	
			//}
		}
		println!("MATCHING BP LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);		
		let matchres:Vec<usize>=transform_to_match(solution.columns(),mlen);
		println!("MATCH RESULT:\n{:?}",matchres);
		let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		pos_idx	
	}
	pub fn lp_go(&self)->Vec<usize>{
		println!("LP GO");
		let solution = self.pb.clone().optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);		
		let matchres:Vec<usize>=transform_to_match(solution.columns(),self.m.len());
		println!("MATCH RESULT:\n{:?}",matchres);
		let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		pos_idx			
	}
	pub fn lp_go_by_pb(&self,pb:RowProblem)->Vec<usize>{
		println!("LP GO BY PB");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);		
		let matchres:Vec<usize>=transform_to_match(solution.columns(),self.m.len());
		println!("MATCH RESULT:\n{:?}",matchres);
		let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		pos_idx			
	}
	pub fn get_all_bits(tmp:&Vec<i8>,idx:usize,n:usize)->Vec<Vec<i8>>{
		let mut allbits:Vec<Vec<i8>>=vec![];
		if idx==n{
			return vec![tmp.to_vec()];
		}
		else{
			let mut tmp0:Vec<i8>=tmp.clone();
			let mut tmp1:Vec<i8>=tmp.clone();
			tmp0.push(0);
			tmp1.push(1);
			allbits.append(&mut Self::get_all_bits(&tmp0,idx+1,n));
			allbits.append(&mut Self::get_all_bits(&tmp1,idx+1,n));
		}
		allbits
	}
	pub fn get_linear_bits(n:usize)->Vec<Vec<i8>>{
		let mut linearbits:Vec<Vec<i8>>=vec![];
		for i in 0..n{
			let mut row:Vec<i8>=vec![0;n];
			row[i]=1;
			linearbits.push(row);
		}
		linearbits
	}
	fn get_enumerate_constraint(&self,bit:&Vec<i8>,onematch:&Vec<usize>)->Vec<Vec<f64>>{
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let zero:Vec<f64>=vec![0.0;mlen*wlen];
		let mut constraints:Vec<Vec<f64>>=vec![];
		for i in 0..bit.len(){
			if bit[i]==1{
				let mut zero_i:Vec<f64>=zero.clone();
				let pos:usize=i*wlen+onematch[i];
				zero_i[pos]=1.0;
				constraints.push(zero_i);
			}
		}
		constraints
	}
	//pub fn enumerate_from_match(&self,matchstack:&Vec<Vec<usize>>,pb:RowProblem,onematch:Vec<usize>){
	pub fn enumerate_from_match(&self,matchstack:&Vec<Vec<usize>>,onematch:Vec<usize>){
		let allbits:Vec<Vec<i8>>=Self::get_all_bits(&vec![],0,onematch.len());
		//let allbits:Vec<Vec<i8>>=Self::get_linear_bits(onematch.len());
		let zeromatch:Vec<usize>=vec![0;onematch.len()];
		println!("ALLBITS\n:{:?}",allbits);
		let mut new_matches:Vec<Vec<usize>>=vec![];
		for i in 0..allbits.len(){
			let constraints:Vec<Vec<f64>>=self.get_enumerate_constraint(&allbits[i],&onematch);
			// RESET THE LP
			let A_ub:Vec<Vec<i8>>=self.A_ub.clone();
			//let (mut c,mat):(Vec<i8>,Vec<Vec<i8>>)=self.fractional_setup_rothblum93_3();
			let A_eq:Vec<Vec<usize>>=self.A_eq.clone();
			// COMMENTED!
			//let A_eq_toggle:Vec<Vec<usize>>=toggle_matrix(&A_eq);
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			let c:Vec<usize>=vec![1;self.m.len()*self.w.len()];
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
				pb.add_row(..1,&row);	
			}
			// the off vector holds if we have a quadratic matrix with n x n rows/columns!
			//let mut off:Vec<usize>=vec![];
			for i in 0..A_ub.len(){
				//if !off.contains(&i){
					let mut row:Vec<(Col,f64)>=vec![];
					for j in 0..A_ub[i].len(){
						row.push((vars[j],A_ub[i][j] as f64));
					}
					pb.add_row(1.0..,&row);	
				//}
			}
			for i in 0..constraints.len(){
				//if !off.contains(&i){
					let mut row:Vec<(Col,f64)>=vec![];
					for j in 0..constraints[i].len(){
						row.push((vars[j],constraints[i][j]));
					}
					pb.add_row(..0.0,&row);	
				//}
			}
			println!("ENUMERATE FROM MATCH ALLBITS[{}]:{:?}",i,allbits[i]);
			let solution = pb.optimise(Sense::Maximise).solve().get_solution();
			//println!("SOLUTION:\n{:?}",solution);		
			let matchres:Vec<usize>=transform_to_match(solution.columns(),self.m.len());
			println!("MATCH RESULT:\n{:?}",matchres);
			let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
			//pos_idx			
			if matchres != zeromatch{
				if !matchstack.contains(&matchres){
					if !new_matches.contains(&matchres){
						new_matches.push(matchres);
					}
				}
			}			
		}
		println!("NEW MATCHES\n");
		for i in 0..new_matches.len(){
			println!("{:?}",new_matches[i]);
		}
		
	}
	
	pub fn enumerate_lp(&self,pb:RowProblem)->Vec<usize>{
		println!("ENUMERATE");
		println!("####### STABILITY BP HIGHS FRACTIONAL TEST");
		println!("INTEGER SOLUTION BUT NO FRACTIONAL SOLUTION UNFORTUNATELY!");


		let mlen:usize=self.m.len();
		//let wlen:usize=self.w.len();
		//let c:Vec<usize>=vec![1;self.m.len()*self.w.len()];
		//let A_eq=self.A_eq.clone();
		//let A_ub=self.A_ub.clone();
		//let mut pb = RowProblem::new();
		
		/*
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
		// the off vector holds if we have a quadratic matrix with n x n rows/columns!
		//let mut off:Vec<usize>=vec![];
		for i in 0..A_ub.len(){
			//if !off.contains(&i){
				let mut row:Vec<(Col,f64)>=vec![];
				for j in 0..A_ub[i].len(){
					row.push((vars[j],A_ub[i][j] as f64));
				}
				pb.add_row(1.0..,&row);	
			//}
		}
		// TEST CLONE PB!
		let pb_clone:RowProblem=pb.clone();
		*/
		
		
		println!("MATCHING BP LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);		
		let matchres:Vec<usize>=transform_to_match(solution.columns(),mlen);
		println!("MATCH RESULT:\n{:?}",matchres);
		let pos_idx:Vec<usize>=get_positive_idx(&solution.columns().to_vec());
		pos_idx	
	}
	pub fn prepare_lp_problem(&self)->RowProblem{
		println!("PREPARE LP PROBLEM!");
		let mut matchLP:matchingLP=matchingLP::new();
		let m:Vec<Vec<usize>>=self.m.clone();
		let w:Vec<Vec<usize>>=self.w.clone();
		let adj:Vec<Vec<[usize;2]>>=IOClass::create_adj(&m,&w);
		let A_ub:Vec<Vec<i8>>=self.A_ub.clone();
		//let (mut c,mat):(Vec<i8>,Vec<Vec<i8>>)=self.fractional_setup_rothblum93_3();
		let A_eq:Vec<Vec<usize>>=self.A_eq.clone();
		
		// COMMENTED!
		//let A_eq_toggle:Vec<Vec<usize>>=toggle_matrix(&A_eq);
		let mlen:usize=m.len();
		let wlen:usize=w.len();
		let c:Vec<usize>=vec![1;m.len()*w.len()];
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
		// the off vector holds if we have a quadratic matrix with n x n rows/columns!
		//let mut off:Vec<usize>=vec![];
		for i in 0..A_ub.len(){
			//if !off.contains(&i){
				let mut row:Vec<(Col,f64)>=vec![];
				for j in 0..A_ub[i].len(){
					row.push((vars[j],A_ub[i][j] as f64));
				}
				pb.add_row(1.0..,&row);	
			//}
		}
		pb
	}
}
struct LPHelper{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
}
impl LPHelper{
	pub fn init(m_:&Vec<Vec<usize>>,w_:&Vec<Vec<usize>>)->Self{
		Self{
			m:m_.clone(),
			w:w_.clone(),
		}
	}
	fn get_a_eq(&self)->Vec<Vec<usize>>{
		//println!("Calculating A_eq");
		let mlen:usize=self.m.len(); 
		let wlen:usize=self.w.len();
		let n:usize=mlen;
		let mut A_eq:Vec<Vec<usize>>=vec![];
		let mut empty:Vec<usize>=vec![];
		for i in 0..n*n{
			empty.push(0);
		}
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[i*n+j]=1;
			}
			A_eq.push(empty_i);
		}
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[j*n+i]=1;
			}
			A_eq.push(empty_i);
		}
		A_eq
	}
	fn stability_rothblum_A_le(&self)->Vec<Vec<i8>>{
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*wlen+wj]=1;
			}
			for j in 0..posm{
				let mj:usize=self.w[wi][j];
				empty_vi[mj*wlen+wi]=1;
			}
			A_el.push(empty_vi);						
		}
		A_el
		/*
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*wlen+wj]=1;
			}
			for j in 0..posm{
				let mj:usize=self.w[wi][j];
				empty_vi[mj*wlen+wi]=1;
			}
			A_el.push(empty_vi);						
		}
		A_el
		*/
		
	}
	fn get_vertices(&self)->Vec<Vec<usize>>{
		let mut vertices:Vec<Vec<usize>>=vec![];
		for i in 0..self.m.len(){
			for j in 0..self.w.len(){
				vertices.push(vec![i,j]);
			}
		}
		vertices
	}
}


struct matchingLP{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	matches:Vec<Vec<usize>>,
	c:Vec<i8>,
	A_eq:Vec<Vec<usize>>,
	b_eq:Vec<usize>,
	A_ub:Vec<Vec<i8>>,
	b_ub:Vec<usize>,
}
impl matchingLP{
	fn new()->Self{
		Self{
			m:IOClass::read_txt("pref/m.txt".to_string()),
			w:IOClass::read_txt("pref/w.txt".to_string()),
			matches:vec![],
			c:vec![],
			A_eq:vec![],
			b_eq:vec![],
			A_ub:vec![],
			b_ub:vec![],
		}
	}
	fn get_vertices(&self)->Vec<Vec<usize>>{
		let mut vertices:Vec<Vec<usize>>=vec![];
		//let lenm:usize=self.m.len();
		//let lenw:usize=self.w.len();
		for i in 0..self.m.len(){
			for j in 0..self.w.len(){
				vertices.push(vec![i,j]);
			}
		}
		vertices
	}
	fn get_id_vertices(mi:usize,wj:usize,mlen:usize,wlen:usize)->usize{
		return mi*mlen+wj;
	}
	fn pref_minus(&self)->Vec<Vec<i8>>{
		println!("Calculating A_ub");
		let mut A_ub:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		//for i in 0..v.len(){
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		for vi in &v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).expect("");
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*mlen+wj]-=1;
			}
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			if posm<wlen-1{
				for j in posm+1..wlen{
					let mj:usize=self.w[wi][j];
					empty_vi[mj*mlen+wi]+=1;
				}
			}
			A_ub.push(empty_vi);
		}
		A_ub
	}
	fn stability_rothblum_A_le(&self)->Vec<Vec<i8>>{
		let mut A_el:Vec<Vec<i8>>=vec![];
		let mut v:Vec<Vec<usize>>=self.get_vertices();
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut empty:Vec<i8>=vec![];
		for i in 0..v.len(){
			empty.push(0);
		}
		for vi in v{
			let mut empty_vi:Vec<i8>=empty.clone();
			let mi:usize=vi[0];
			let wi:usize=vi[1];
			let mlen:usize=self.m.len();
			let wlen:usize=self.w.len();
			empty_vi[mi*wlen+wi]=1;
			let posw:usize=self.m[mi].iter().position(|x| *x==wi).unwrap();
			let posm:usize=self.w[wi].iter().position(|x| *x==mi).unwrap();
			// men's side!
			for j in 0..posw{
				let wj:usize=self.m[mi][j];
				empty_vi[mi*wlen+wj]=1;
			}
			for j in 0..posm{
				let mj:usize=self.w[wi][j];
				empty_vi[mj*wlen+wi]=1;
			}
			A_el.push(empty_vi);						
		}
		A_el
	}
	fn basic_data(&self)->(Vec<i8>,Vec<usize>,Vec<usize>,Vec<usize>){
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let n:usize=mlen*wlen;
		let mut c:Vec<i8>=vec![];
		let mut b_ub:Vec<usize>=vec![];
		let mut b_eq:Vec<usize>=vec![];
		let mut bounds:Vec<usize>=vec![];
		for i in 0..n{
			//c.push(-1);
			// BE VERY CAREFUL !!!!!!!
			c.push(-1);
			b_ub.push(0);
			bounds.push(0);
		}
		for i in 0..2*mlen{
			b_eq.push(1);
		}
		(c,b_ub,b_eq,bounds)
	}
	fn save_param(&mut self){
		let (c,b_ub,b_eq,bounds)=self.basic_data();
		let A_eq:Vec<Vec<usize>>=self.get_a_eq();
		let A_ub:Vec<Vec<i8>>=self.pref_minus();
		self.A_eq=A_eq.clone();
		self.b_eq=b_eq.clone();
		self.A_ub=A_ub.clone();
		self.b_ub=b_ub.clone();
		
		let mut c_tmp:Vec<i8>=vec![];
		// This is due to flexibility to min-max-adjustments in c!
		// Comment if c should be negative!
		for i in 0..c.len(){
			c_tmp.push(1);
		}
		self.c=c_tmp.clone();
	}
	fn get_a_eq(&self)->Vec<Vec<usize>>{
		println!("Calculating A_eq");
		let mlen:usize=self.m.len(); 
		let wlen:usize=self.w.len();
		let n:usize=mlen;
		let mut A_eq:Vec<Vec<usize>>=vec![];
		let mut empty:Vec<usize>=vec![];
		for i in 0..n*n{
			empty.push(0);
		}
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[i*n+j]=1;
			}
			A_eq.push(empty_i);
		}
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[j*n+i]=1;
			}
			A_eq.push(empty_i);
		}
		A_eq
	}
	fn get_a_eq_2(&self)->(Vec<Vec<usize>>,Vec<Vec<usize>>){
		println!("Calculating A_eq");
		let mlen:usize=self.m.len(); 
		let wlen:usize=self.w.len();
		let n:usize=mlen;
		let mut A_eq:Vec<Vec<usize>>=vec![];
		let mut m_A_eq:Vec<Vec<usize>>=vec![];
		let mut w_A_eq:Vec<Vec<usize>>=vec![];
		//let mut empty:Vec<usize>=vec![];
		let mut empty:Vec<usize>=vec![0;mlen*wlen];
		/*
		for i in 0..n*n{
			empty.push(0);
		}
		*/
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[i*n+j]=1;
			}
			A_eq.push(empty_i.clone());
			m_A_eq.push(empty_i);
		}
		for i in 0..n{
			let mut empty_i:Vec<usize>=empty.clone();
			for j in 0..n{
				empty_i[j*n+i]=1;
			}
			A_eq.push(empty_i.clone());
			w_A_eq.push(empty_i);
		}
		//A_eq
		(m_A_eq,w_A_eq)
	}

	fn fractional_setup_rothblum93_3(&self)->(Vec<i8>,Vec<Vec<i8>>){
		println!("FRACTIONAL SETUP ROTHBLUM '93");
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut c:Vec<i8>=vec![];
		let mut mat:Vec<Vec<i8>>=vec![];
		//for i in 0..mlen+wlen+mlen*wlen{
		for i in 0..mlen*wlen{
			c.push(1)
		}
		//let block:usize=mlen+wlen;
		let block:usize=0;
		
		//for i in 0..mlen*wlen{
		for i in 0..mlen{
			for j in 0..wlen{
				//let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				//row[i]=1;
				//row[mlen+j]=1;
				//row[block+i*wlen+j]=-1;
				row[block+i*wlen+j]=-1;
				let posw:usize=self.m[i].iter().position(|x| *x==j).unwrap();
				let posm:usize=self.w[j].iter().position(|x| *x==i).unwrap();
				if posw<wlen-1{
				//if posw<wlen{
					for k in posw+1..wlen{
					//for k in 0..posw{
						let wk:usize=self.m[i][k];
						row[block+i*wlen+wk]=-1;
					}
				}
				if posm<mlen-1{
				//if posm<mlen{
					for k in posm+1..mlen{
					//for k in 0..posm{
						let mk:usize=self.w[j][k];
						row[block+mk*wlen+j]=-1;
					}
				}
				
				mat.push(row);
			}
		}
		//println!("c:{:?}",c);
		//println!("mat:");
		for i in 0..mat.len(){
			//println!("{:?}",mat[i]);
		}
		(c,mat)
	}

	fn fractional_setup_rothblum93(&self)->(Vec<i8>,Vec<Vec<i8>>){
		println!("FRACTIONAL SETUP ROTHBLUM '93");
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut c:Vec<i8>=vec![];
		let mut mat:Vec<Vec<i8>>=vec![];
		//for i in 0..mlen+wlen+mlen*wlen{
		for i in 0..mlen*wlen{
			/*
			if i<mlen+wlen{
				c.push(1);
			}
			else{
				c.push(-1);
				
								
			}
			*/
			c.push(-1)
		}
		//let block:usize=mlen+wlen;
		let block:usize=0;
		
		//for i in 0..mlen*wlen{
		for i in 0..mlen{
			for j in 0..wlen{
				//let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				row[i]=1;
				row[mlen+j]=1;
				row[block+i*wlen+j]=-1;
				let posw:usize=self.m[i].iter().position(|x| *x==j).unwrap();
				let posm:usize=self.w[j].iter().position(|x| *x==i).unwrap();
				if posw<wlen-1{
				//if posw<wlen{
					for k in posw+1..wlen{
					//for k in 0..posw{
						let wk:usize=self.m[i][k];
						row[block+i*wlen+wk]=-1;
					}
				}
				if posm<mlen-1{
				//if posm<mlen{
					for k in posm+1..mlen{
					//for k in 0..posm{
						let mk:usize=self.w[j][k];
						row[block+mk*wlen+j]=-1;
					}
				}
				mat.push(row);
			}
		}
		//println!("c:{:?}",c);
		//println!("mat:");
		for i in 0..mat.len(){
			//println!("{:?}",mat[i]);
		}
		(c,mat)
	}
	fn fractional_setup_rothblum93_2(&self)->(Vec<i8>,Vec<Vec<i8>>){
		println!("FRACTIONAL SETUP ROTHBLUM '93 TESTRUN");
		let mlen:usize=self.m.len();
		let wlen:usize=self.w.len();
		let mut c:Vec<i8>=vec![];
		let mut mat:Vec<Vec<i8>>=vec![];
		//for i in 0..mlen+wlen+mlen*wlen{
		for i in 0..mlen*wlen{
			c.push(-1)
		}
		//let block:usize=mlen+wlen;
		let block:usize=0;
		
		//for i in 0..mlen*wlen{
		for i in 0..mlen{
			for j in 0..wlen{
				//let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				let mut row:Vec<i8>=vec![0;block+mlen*wlen];
				//row[i]=1;
				//row[mlen+j]=1;
				row[block+i*wlen+j]=-1;
				let posw:usize=self.m[i].iter().position(|x| *x==j).unwrap();
				let posm:usize=self.w[j].iter().position(|x| *x==i).unwrap();
				if posw<wlen-1{
				//if posw<wlen{
					for k in posw+1..wlen{
					//for k in 0..posw{
						let wk:usize=self.m[i][k];
						row[block+i*wlen+wk]=-1;
					}
				}
				if posm<mlen-1{
				//if posm<mlen{
					for k in posm+1..mlen{
					//for k in 0..posm{
						let mk:usize=self.w[j][k];
						row[block+mk*wlen+j]=-1;
					}
				}
				mat.push(row);
			}
		}
		//println!("c:{:?}",c);
		//println!("mat:");
		for i in 0..mat.len(){
			//println!("{:?}",mat[i]);
		}
		(c,mat)
	}
	
	fn prepare_LP_highs(&mut self){
		println!("prepare_LP_highs");
		let (c,b_ub,b_eq,bounds)=self.basic_data();
		let A_eq=self.get_a_eq();
		let A_ub=self.pref_minus();
		let mut pb = RowProblem::new();		
		let mut vars:Vec<Col>=vec![];
		for i in 0..c.len(){
			vars.push(pb.add_column(1.0,0..));
		}
		for i in 0..A_eq.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_eq[i].len(){
				row.push((vars[j],A_eq[i][j] as f64));
			}
			//pb.add_row(..=1,&row);	
			pb.add_row(..=b_eq[i] as f64,&row);	
		}
		for i in 0..A_ub.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_ub[i].len(){
				row.push((vars[j],A_ub[i][j] as f64));
			}
			//pb.add_row(..0,&row);	
			pb.add_row(..0,&row);	
		}
		println!("MATCHING LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);
		
		let matchres:Vec<usize>=self.transform_to_match(solution.columns());
		println!("MATCH RESULT:\n{:?}",matchres);
		
	}
	fn prepare_LP_highs_rothblum(&mut self){
		println!("prepare_LP_highs");
		let (c,b_ub,b_eq,bounds)=self.basic_data();
		let A_eq=self.get_a_eq();
		//let A_ub=self.pref_minus();
		let A_le=self.stability_rothblum_A_le();
		let mut pb = RowProblem::new();		
		let mut vars:Vec<Col>=vec![];
		for i in 0..c.len(){
			vars.push(pb.add_column(1.0,0..));
		}
		for i in 0..A_eq.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			for j in 0..A_eq[i].len(){
				row.push((vars[j],A_eq[i][j] as f64));
			}
			//pb.add_row(..=1,&row);	
			pb.add_row(..=b_eq[i] as f64,&row);	
		}
		//for i in 0..A_ub.len(){
		for i in 0..A_le.len(){
			let mut row:Vec<(Col,f64)>=vec![];
			//for j in 0..A_ub[i].len(){
			for j in 0..A_le[i].len(){
				//row.push((vars[j],A_ub[i][j] as f64));
				row.push((vars[j],A_le[i][j] as f64));
			}
			//pb.add_row(..0,&row);	
			
			pb.add_row(..1,&row);	
		}
		println!("MATCHING LP HIGHS");
		let solution = pb.optimise(Sense::Maximise).solve().get_solution();
		println!("SOLUTION:\n{:?}",solution);
		
		let matchres:Vec<usize>=self.transform_to_match(solution.columns());
		println!("MATCH RESULT:\n{:?}",matchres);
		
	}
	
	fn transform_to_match(&self,input:&[f64])->Vec<usize>{
	//fn transform_to_match(&self,input:&Vec<usize>)->Vec<usize>{
		let mut matchres:Vec<usize>=vec![];
		let n:usize=self.m.len();
		for i in 0..n{
			matchres.push(0);
		}
		for i in 0..input.len(){
			if input[i] as f64==1.0{
				let m:usize=i/n;
				let w:usize=i%n;
				matchres[m]=w;
			}
		}
		println!("MATCH: {:?}",matchres);
		matchres
	}
	fn transform_to_match_usize(&self,input:&Vec<usize>)->Vec<usize>{
		let mut matchres:Vec<usize>=vec![];
		let n:usize=self.m.len();
		for i in 0..n{
			matchres.push(0);
		}
		for i in 0..input.len(){
			if input[i] as f64==1.0{
				let m:usize=i/n;
				let w:usize=i%n;
				matchres[m]=w;
			}
		}
		println!("MATCH: {:?}",matchres);
		matchres
	}
}
// switch all the values within a matrix: 0 -> 1 and 1 -> 0
fn toggle_matrix(a:&Vec<Vec<usize>>)->Vec<Vec<usize>>{
	let mut b:Vec<Vec<usize>>=vec![];
	for i in 0..a.len(){
		let mut row:Vec<usize>=vec![];
		for j in 0..a[i].len(){
			if a[i][j]==0{
				row.push(1);
			}
			else{
				row.push(0);
			}
		}
		b.push(row);
	}
	b
}
fn transform_to_match(input:&[f64],n:usize)->Vec<usize>{
//fn transform_to_match(&self,input:&Vec<usize>)->Vec<usize>{
	let mut matchres:Vec<usize>=vec![];
	//let n:usize=self.m.len();
	for i in 0..n{
		matchres.push(0);
	}
	for i in 0..input.len(){
		if input[i] as f64==1.0{
			let m:usize=i/n;
			let w:usize=i%n;
			matchres[m]=w;
		}
	}
	println!("MATCH: {:?}",matchres);
	matchres
}
fn get_positive_idx(a:&Vec<f64>)->Vec<usize>{
	let mut idx:Vec<usize>=vec![];
	for i in 0..a.len(){
		if a[i]>0.0{
			idx.push(i);
		}
	}
	idx
}

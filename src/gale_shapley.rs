#![allow(warnings)]
use super::helper::{IOClass};
use super::image;



pub struct gs_lists{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	gsm:Vec<usize>,
	gsw:Vec<usize>,
	m_shorted:Vec<Vec<usize>>,
	w_shorted:Vec<Vec<usize>>,
	bm:Vec<Vec<bool>>,
	nodecount:usize,
}
impl gs_lists{
	pub fn new(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>)->Self{
		let mut gs_:gale_shapley=gale_shapley::new();
		let mpref:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
		let wpref:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
		gs_.init(&mpref,&wpref);
		gs_.run();
		let gsm_res:Vec<usize>=gs_.matchres.clone().expect("");

		let mut gsw_:gale_shapley=gale_shapley::new();
		gsw_.init(&wpref,&mpref);
		gsw_.run();
		let gsw_orig:Vec<usize>=gsw_.matchres.clone().expect("");
		let gsw_res:Vec<usize>=gale_shapley::traverse_wtmp(&gsw_orig);
		
		println!("MATCH for MEN:{:?}\nMATCH for WOMEN:{:?}",gsm_res,gsw_res);
		let adj:Vec<Vec<[usize;2]>>=IOClass::create_adj(&mpref,&wpref);
		let shorted_lists:Vec<Vec<bool>>=create_lattice_mask(&adj,&gsm_res,&gsw_orig);
		let trav_shorted=traverse_bool_matrix(&shorted_lists);
		image::img_bp_matrix(&trav_shorted,"pic/shorted/shorted_lists.png".to_string());
		//let (mshort,wshort,node_count)=get_vertices(&trav_shorted);
		let (mshort,wshort,node_count)=get_vertices(&shorted_lists);
		println!("m:{:?}\nw:{:?}\nnodecount:{}",mshort,wshort,node_count);
		
		Self{
			m:mpref,
			w:wpref,
			gsm:gsm_res,
			gsw:gsw_res,
			m_shorted:mshort,
			w_shorted:wshort,
			bm:shorted_lists,
			nodecount:node_count,
		}
	}
	pub fn get_nodecount(&self)->usize{
		return self.nodecount;
	}
}



pub fn test(){
	let mut gs_:gale_shapley=gale_shapley::new();
	let mpref:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
	let wpref:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
	gs_.init(&mpref,&wpref);
	gs_.run();
	gs_.print_match();
	let gsm:Vec<usize>=gs_.matchres.clone().expect("");

	let mut gsw_:gale_shapley=gale_shapley::new();
	let mpref:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
	let wpref:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
	gsw_.init(&wpref,&mpref);
	gsw_.run();
	gsw_.print_match();
	let gsw_orig:Vec<usize>=gsw_.matchres.clone().expect("");
	let gsw:Vec<usize>=gale_shapley::traverse_wtmp(&gsw_orig);
	
	println!("MATCH for MEN:{:?}\nMATCH for WOMEN:{:?}",gsm,gsw);
	let adj:Vec<Vec<[usize;2]>>=IOClass::create_adj(&mpref,&wpref);
	let shorted_lists:Vec<Vec<bool>>=create_lattice_mask(&adj,&gsm,&gsw_orig);
	let trav_shorted=traverse_bool_matrix(&shorted_lists);
	image::img_bp_matrix(&trav_shorted,"pic/shorted/shorted_lists.png".to_string());
	let (mshort,wshort,node_count)=get_vertices(&trav_shorted);
	println!("m:{:?}\nw:{:?}\nnodecount:{}",mshort,wshort,node_count);
}

pub struct gale_shapley{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	proposals:Vec<usize>,
	unmatched:Vec<usize>,
	wtmp:Vec<usize>,
	// when the algorithm finishes, the vale of wtmp will be copied to matchres
	matchres:Option<Vec<usize>>
}
impl gale_shapley{
	pub fn new()->Self{
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
	pub fn init(&mut self,mpref:&Vec<Vec<usize>>,wpref:&Vec<Vec<usize>>){
		self.m=mpref.clone();
		self.w=wpref.clone();
		let n:usize=self.m.len();
		for i in 0..n{
			self.proposals.push(0);
			self.unmatched.push(i);
			self.wtmp.push(n);
		}
	}
	pub fn run(&mut self){
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
		self.matchres=Some(Self::traverse_wtmp(&self.wtmp.clone()));
	}
	pub fn traverse_wtmp(wtmp:&Vec<usize>)->Vec<usize>{
		let mut mtmp:Vec<usize>=vec![];
		for i in 0..wtmp.len(){
			let pos:usize=wtmp.iter().position(|x| *x==i).unwrap();
			mtmp.push(pos);
		}
		mtmp
	}
	// this function will display the wtmp vector as mtmp vector where male i shows the match (w(m_i))!
	pub fn print_match(&self){
		println!("wrong! mtmp and wtmp are exchanged!");
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
// create a bool nxn mask to check which pairs are generally possible and which aren't
fn create_lattice_mask(adj:&Vec<Vec<[usize;2]>>,gsm:&Vec<usize>,gsw:&Vec<usize>)->Vec<Vec<bool>>{
	let n:usize=adj.len();
	let mut mask:Vec<Vec<bool>>=vec![];
	// create restrictions vectors (see lattice by Knuth) for m and w
	let mut m_rstrct:Vec<[usize;2]>=vec![];
	let mut w_rstrct:Vec<[usize;2]>=vec![];
	for i in 0..adj.len(){
		m_rstrct.push([0,n]);
		w_rstrct.push([0,n]);
	}
	for i in 0..gsm.len(){
		let wi:usize=gsm[i];
		m_rstrct[i][0]=adj[i][wi][0];
		w_rstrct[wi][1]=adj[i][wi][1];
	}
	for i in 0..gsw.len(){
		let mi:usize=gsw[i];
		w_rstrct[i][0]=adj[mi][i][1];
		m_rstrct[mi][1]=adj[mi][i][0];
	}
	for i in 0..adj.len(){
		let mut mask_i:Vec<bool>=vec![];
		let m_minpos=adj[i][gsm[i]][0];
		for j in 0..adj[0].len(){
			// if w is outside the range of m's feasible preference list, this field is labeled false
			if adj[i][j][0]<m_rstrct[i][0] || adj[i][j][0]>m_rstrct[i][1]{
				mask_i.push(false);
			}
			else{
				// if m is outside of w's feasible preference list, this field is labeled false
				if adj[i][j][1]<w_rstrct[j][0] || adj[i][j][1]>w_rstrct[j][1]{
					mask_i.push(false);
				}
				else{
					// otherwise, if both m and w are in the feasible range, this field is labeled true
					mask_i.push(true);
				}
			}		
		}
		mask.push(mask_i);
	}
	mask
}



pub fn traverse_bool_matrix(boolmatrix:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let mut bm:Vec<Vec<bool>>=vec![];
	for i in 0..boolmatrix.len(){
		let mut row:Vec<bool>=vec![];
		for j in 0..boolmatrix[i].len(){
			row.push(boolmatrix[j][i]);
		}
		bm.push(row);
	}
	bm
}
//pub fn get_vertices(trav_bm:&Vec<Vec<bool>>)->Vec<[usize;2]>{
pub fn get_vertices(trav_bm:&Vec<Vec<bool>>)->(Vec<Vec<usize>>,Vec<Vec<usize>>,usize){
	let mut m:Vec<Vec<usize>>=vec![vec![];trav_bm.len()];
	let mut w:Vec<Vec<usize>>=vec![vec![];trav_bm.len()];
	let mut vertices:Vec<[usize;2]>=vec![];
	let mut node_count:usize=0;
	for i in 0..trav_bm.len(){
		for j in 0..trav_bm[i].len(){
			if trav_bm[i][j]{
				m[i].push(j);
				w[j].push(i);
				node_count+=1;
			}
		}
	}
	//vertices
	(m,w,node_count)
}

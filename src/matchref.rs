#![allow(warnings)]
use super::helper::{IOClass};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet,HashMap};
use std::hash::{Hash,Hasher};
use std::ptr;

#[derive(PartialEq,Eq,Debug)]
pub struct matchref{
	sub: HashMap<usize,Vec<Rc<RefCell<direct>>>>,
	up: Vec<Rc<RefCell<matchref>>>,
	members:Vec<Rc<RefCell<node>>>,
	count:usize,
	member:Rc<RefCell<node>>,
	drct:Vec<Rc<RefCell<direct>>>,
}
impl matchref{
	fn new(new_node:Rc<RefCell<node>>)->Rc<RefCell<matchref>>{
		let val:usize=new_node.borrow().val;
		Rc::new(RefCell::new(
			Self{
				sub:HashMap::from([(val,vec![])]),
				up:vec![],
				members:vec![],
				count:0,
				member:new_node.clone(),
				drct:vec![],
			}
		))
	}
	fn insert(&mut self, new:Rc<RefCell<node>>)->&mut matchref{
		self
	}
	fn from_prenode(pre:&prenode)->Rc<RefCell<Self>>{
		let mut drct=direct::new(pre.name);
		Rc::new(RefCell::new(
			Self{
				sub:HashMap::from([(pre.name,vec![drct])]),
				up:vec![],
				members:vec![],
				count:0,
				member:node::new(pre.name),
				drct:vec![],
			}
		))
	}
	fn from_matchref(m:Rc<RefCell<matchref>>,n:Rc<RefCell<node>>)->Rc<RefCell<Self>>{
		Rc::new(RefCell::new(
			Self{
				sub:HashMap::new(),
				up:vec![],
				members:vec![],
				count:m.borrow().count+1,
				member:n,
				drct:vec![],
			}
		))
	}
}
#[derive(Debug)]
pub struct prenode{
	members:HashSet<usize>,
	name:usize
}
impl prenode{
	fn new(name_:usize)->Self{
		Self{
			members:HashSet::new(),
			name:name_,
		}
	}
}
#[derive(PartialEq,Eq,Debug)]
pub struct node{
	val:usize,
	matches:Vec<Rc<RefCell<matchref>>>
}
impl node{
	fn new(val_:usize)->Rc<RefCell<node>>{
		Rc::new(RefCell::new(
			Self{
				val:val_,
				matches:vec![]
			}
		))
	}
}

#[derive(PartialEq,Eq,Debug)]
pub struct direct{
	val:usize,
	cn:usize,
	down:Option<Rc<RefCell<direct>>>,
}
impl direct{
	fn new(val_:usize)->Rc<RefCell<Self>>{
		Rc::new(RefCell::new(
			Self{
				val:val_,
				down:None,
				cn:0,
			}
		))
	}
	fn add(&mut self, sub:Rc<RefCell<direct>>){
		self.cn=sub.borrow().cn+1;
		self.down=Some(sub);
	}
}


#[derive(Debug)]
pub struct matching{
	parts:HashMap<usize,Vec<Rc<RefCell<matchref>>>>,
	tmp:Vec<Rc<RefCell<direct>>>
}
impl matching{
	fn new()->Self{
		Self{
			parts: HashMap::new(),
			tmp:vec![]
		}
	}
	fn insert(&mut self,pre:&prenode){
		println!("INSERT PRE.NAME={}",pre.name);
		self.tmp=vec![];
		let mut mr=matchref::from_prenode(&pre);
		let iter:Vec<usize>=self.parts.clone().into_keys().collect();
		let transf:HashSet<usize>=iter.into_iter().collect();
		let intersect=pre.members.intersection(&transf);
		let mut ord_inters1:Vec<&usize>=intersect.clone().into_iter().collect();
		ord_inters1.sort();
		for elem in ord_inters1{
			let hash=&self.parts[&elem];
			for i in 0..hash.len(){
				let keys:HashSet<usize>=hash[i].borrow().sub.clone().into_keys().collect();
				let inters_keys=pre.members.intersection(&keys);
				let mut ord_inters2:Vec<&usize>=inters_keys.clone().into_iter().collect();
				ord_inters2.sort();
				for elem2 in ord_inters2{
					if self.tmp.len()==0 && hash[i].borrow().sub[&elem2].len()==1{
						//println!("MR !!!!!!! elem2: {}", elem2);
						mr.borrow_mut().sub.insert(*elem2,vec![hash[i].borrow().sub[&elem2][0].clone()]);
						//println!("IF IF IF IF: ELEM2: __{}__", elem2);
						self.tmp.append(&mut hash[i].borrow().sub[&elem2].clone());
					}
					else{
						// THINK TWICE !!!!!!!
						/*
						if self.tmp.contains(&hash[i].borrow().sub[&elem2]){
							
						}
						*/
						let mut cn:usize=0;
						for el in &hash[i].borrow().sub[&elem2]{
							if self.tmp.contains(&el){
								let mut drct_el=direct::new(*elem);
								drct_el.borrow_mut().add(el.clone());
								let mut drct_el2=drct_el.clone();
								if !mr.borrow().sub.keys().into_iter().collect::<Vec<&usize>>().contains(&elem){
									mr.borrow_mut().sub.insert(*elem,vec![drct_el.clone()]);
								}
								else{
								}
								self.tmp.push(drct_el2.clone());
								mr.borrow_mut().drct.push(drct_el2);
								cn+=1;
							}
						}
					}					
				}
			}
		}
		self.parts.insert(pre.name,vec![mr]);		
	}
}
pub fn matchref_run(){	
	let mut m_pref: Vec<Vec<usize>> = IOClass::read_txt("pref/m.txt".to_string());
	let mut w_pref: Vec<Vec<usize>> = IOClass::read_txt("pref/w.txt".to_string());
	let adj_test=IOClass::create_adj(&m_pref,&w_pref);
	let bp_matrix=IOClass::bp_matrix(&adj_test);
	println!("TEST");
	let bitlength:usize=64;
	let (bp_reshuffled,bp_order)=IOClass::bp_reshuffle(&bp_matrix);
	println!("TEST MATCHREF");
	//let bp_lower_half:Vec<Vec<bool>>=bp_matrix_lower_half(&bp_matrix);
	let bp_half:Vec<Vec<bool>>=IOClass::bp_matrix_half(&bp_reshuffled);
	let bp_lower_half:Vec<Vec<bool>>=IOClass::bp_matrix_lower_half(&bp_reshuffled);
	let mut a:HashMap<usize,HashSet<usize>>=HashMap::new();
	a.insert(1,HashSet::new());
	let res=&a[&1];
	
	let mut a_n:Rc<RefCell<node>>=node::new(0);
	let mut b_n:Rc<RefCell<node>>=node::new(0);
	let mut d_n:Rc<RefCell<node>>=node::new(0);
	let mut a_m:Rc<RefCell<matchref>>=matchref::new(a_n.clone());
	let mut b_m:Rc<RefCell<matchref>>=matchref::new(b_n.clone());
	let mut c_n=a_n.clone();
	
	
	dbg!(a_n==c_n);
	dbg!(Rc::ptr_eq(&a_n,&d_n));
	let mut refvec:Vec<&Rc<RefCell<node>>>=vec![];
	refvec.push(&a_n);
	refvec.push(&b_n);
	//dbg!(refvec.contains(&d_n));
	dbg!(refvec.contains(&&d_n));
	
	let mut hashvec:Vec<Rc<RefCell<node>>>=vec![];
	for i in 0..10{
		hashvec.push(node::new(i));
	}
	let mut hashs:HashSet<HashSet<usize>>=HashSet::new();	
	let mut hashvec:Vec<HashSet<usize>>=vec![];
	for i in 0..10{
		let mut hash_i:HashSet<usize>=HashSet::new();
		for j in 0..i{
			hash_i.insert(j);
		}
		hashvec.push(hash_i);
	}
	let bp=&bp_lower_half;
	let mut prevec:Vec<prenode>=vec![];
	let mut matches:matching=matching::new();
	for i in 0..bp.len(){
		let mut pre:prenode=prenode::new(i);
		for j in 0..i{
			if bp[i][j]{
				pre.members.insert(j);
			}
		}
		prevec.push(pre);
	}	
	let mut m:matching=matching::new();
	for i in 0..prevec.len(){
		m.insert(&prevec[i]);
	}	
	let mut ptrvec:Vec<&usize>=vec![];
	for i in 0..10{
		let ptr:&&usize=&&i;
		let new:usize=i.clone();
		println!("i: {}, ptr: {:?}",i,ptr::addr_of!(new));
	}
	let nn=node::new(1);
	let ptr_nn=ptr::addr_of!(nn);
	println!("i: , ptr: {:?}",ptr::addr_of!(nn));
	println!("i: , ptr: {:?}",ptr_nn);
	println!("matchref run end");
}

#[derive(PartialEq,Eq,Debug,Clone)]
struct hashres{
	mem: HashSet<usize>,
}
impl hashres{
	fn new(val_:usize)->Rc<RefCell<Self>>{
		let mut hash:HashSet<usize>=HashSet::new();
		hash.insert(val_);
		Rc::new(RefCell::new(
			Self {
				mem: hash,
			}
		))
	}
}



pub fn transform_hashset_2vec(h:HashSet<usize>)->Vec<usize>{
	let mut vec:Vec<usize>=vec![];
	for el in h{
		vec.push(el);
	}
	vec
}
// IMPORTANT IMPORTANT IMPORTANT !!!!!!!

#[derive(Debug,Clone)]
struct HashSetWrapper(Rc<RefCell<HashSet<usize>>>);



impl Hash for HashSetWrapper{
	fn hash<H: Hasher>(&self, state:&mut H){
		let inner=self.0.borrow();
		for value in inner.iter(){
			value.hash(state);
		}
	}
}

impl PartialEq for HashSetWrapper{
	fn eq(&self, other:&Self)->bool{
		let self_inner=self.0.borrow();
		let other_inner=other.0.borrow();
		//self_inner==other_inner
		//self_inner==&*other_inner
		// VERY IMPORTANT !!!!!!!
		self_inner.eq(&*other_inner)
	}
}

impl Eq for HashSetWrapper {
	/*
	fn eq(&self, other:&Self)->bool{
		let self_inner=self.0.borrow();
		let other_inner=other.0.borrow();
		self_inner==other_inner
	}
	*/	
}

pub fn test_hashmatch()->Vec<Vec<usize>>{
	
	let mut m_pref: Vec<Vec<usize>> = IOClass::read_txt("pref/m.txt".to_string());
	let mut w_pref: Vec<Vec<usize>> = IOClass::read_txt("pref/w.txt".to_string());
	let adj_test=IOClass::create_adj(&m_pref,&w_pref);
	let bp_matrix=IOClass::bp_matrix(&adj_test);
	let bitlength:usize=64;

	//let (bp_reshuffled,bp_order)=IOClass::bp_reshuffle(&bp_matrix);
	//let bp_half:Vec<Vec<bool>>=IOClass::bp_matrix_half(&bp_reshuffled);
	let bp_half:Vec<Vec<bool>>=IOClass::bp_matrix_half(&bp_matrix);
	//let bp:Vec<Vec<bool>>=IOClass::bp_matrix_lower_half(&bp_reshuffled);
	let bp:Vec<Vec<bool>>=IOClass::bp_matrix_lower_half(&bp_matrix);
	let bp_half_val:Vec<Vec<usize>>=IOClass::bp_matrix_2usize_block2(&bp_half,17);	
	let n25_val:usize=67108863;
	let n:usize=bp.len().isqrt();
	let mut tmp_n25:Vec<usize>=vec![];		
	println!("LATTICE MATCHING WITH HASHVALUES");	
	let mut pr:Vec<prenode>=vec![];
	for i in 0..bp.len(){
		let mut pre:prenode=prenode::new(i);
		for j in 0..bp[i].len(){
			if bp[i][j]{
				pre.members.insert(j);
			}
		}
		pr.push(pre);
	}
	let mut compl:Vec<HashSet<HashSetWrapper>>=vec![];
	let mut first:HashSet<usize>=HashSet::new();
	first.insert(0);
	let mut firsthash:HashSet<HashSetWrapper>=HashSet::new();
	let inner_set=HashSetWrapper(Rc::new(RefCell::new(first)));
	firsthash.insert(inner_set);
	let mut firstref:Rc<RefCell<hashres>>=hashres::new(0);
	let n:usize=bp.len().isqrt();
	let mut max_cn:usize=0;	
	let mut matches:Vec<Vec<usize>>=vec![];
	for i in 0..pr.len(){
		let pre=&pr[i];
		let mut hash:HashSet<HashSetWrapper>=HashSet::new();
		let mut tmp:HashSet<HashSetWrapper>=HashSet::new();
		let mut tmp_vec:Vec<HashSet<usize>>=vec![];
		for el in &pre.members{
			//dbg!(el);
			if *el<compl.len(){
				let list_el=&compl[*el];
				// CLONED !!!!!!!
				//let is:HashSet<HashSetWrapper>=list_el.intersection(&tmp).cloned().collect();
				let is:HashSet<HashSetWrapper>=list_el.difference(&tmp).cloned().collect();
				tmp=tmp.union(&is).cloned().collect();
			}
		}
		if tmp.len()==0{
			let mut one:HashSet<usize>=HashSet::new();
			one.insert(i);
			hash.insert(HashSetWrapper(Rc::new(RefCell::new(one))));
		}
		else{
			for el in tmp{
				let mut is:HashSet<_>=el.0.borrow_mut().intersection(&pre.members).cloned().collect();
				if is==*el.0.borrow(){
					el.0.borrow_mut().insert(i);
					if el.0.borrow().len()>max_cn{
						max_cn=el.0.borrow().len();
					}
					if el.0.borrow().len()==n{
						println!("MATCH: {:?}", el.0.borrow());
						//let mut matchresult:Vec<usize>=el.0.borrow();
						let hashset:HashSet<usize>=el.0.borrow().clone();
						matches.push(transform_hashset_2vec(el.0.borrow().clone()));
						
					}
				}
				else{
					is.insert(i);
					if is.len()>max_cn{
						max_cn=is.len();
					}
					if is.len()==n{
						println!("MATCH: {:?}", is);
					}
					if !tmp_vec.contains(&is){
						let hashwrapper=HashSetWrapper(Rc::new(RefCell::new(is.clone())));
						tmp_vec.push(is);
						hash.insert(hashwrapper);
					}
				}				
			}
		}
		compl.push(hash);
	}
	println!("END LATTICE MATCHING");
	let dec_matches=IOClass::decompose_matches(&matches,n);
	IOClass::print_matches_decomposed(&dec_matches);
	matches
}

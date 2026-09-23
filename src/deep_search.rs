#![allow(warnings)]
use super::helper::{IOClass};
use super::image;
pub fn test(){
	let zero_bp=IOClass::create_zero_bp(5);
	let mmbp=IOClass::update_bp_manymany(&zero_bp);
	image::img_bp_matrix(&mmbp,"pic/mmbp_test.png".to_string());
	let mut ds:deep_search=deep_search::new();
	
	let matches:Vec<Vec<[usize;2]>>=ds.run_wrapper();
}

pub struct deep_search{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
	bp:Vec<Vec<bool>>,
	qm:Vec<usize>,
	qm_orig:Vec<usize>,
	qw:Vec<usize>,
	qw_orig:Vec<usize>,
	n:usize,
}
impl deep_search{
	pub fn new()->Self{
		let m_=IOClass::read_txt("pref/m.txt".to_string());
		let w_=IOClass::read_txt("pref/w.txt".to_string());
		let bp_=IOClass::get_bp_automatically();
		let bpmm=IOClass::update_bp_manymany(&bp_);
		Self{
			m:m_,
			w:w_,
			bp:bpmm.clone(),
			qm:vec![1,1,2],
			qm_orig:vec![1,1,2],
			qw:vec![2,1,1],
			qw_orig:vec![2,1,1],
			n:bpmm.len().isqrt(), // Think of N-sided!!!
		}
	}
	pub fn check_everone_is_matched(&self,tmp:&Vec<[usize;2]>)->bool{
		let mut everyone:bool=true;
		for i in 0..self.m.len(){
			let mut in_tmp:bool=false;
			for j in 0..tmp.len(){
				if tmp[j][0]==i{
					in_tmp=true;
				}
			}
			if !in_tmp{
				return false;
			}
		}
		for i in 0..self.w.len(){
			let mut in_tmp:bool=false;
			for j in 0..tmp.len(){
				if tmp[j][1]==i{
					in_tmp=true;
				}
			}
			if !in_tmp{
				return false;
			}
		}
		everyone
	}
	pub fn get_agent_chain(&self,tmp:&Vec<[usize;2]>,set:usize,agent:usize)->Vec<[usize;2]>{
		let mut collect:Vec<[usize;2]>=vec![];
		for i in 0..tmp.len(){
			if tmp[i][set]==agent{
				collect.push(tmp[i]);
			}
		}
		collect
	}
	pub fn get_least(&self,tmp:&Vec<[usize;2]>,set:usize,agent:usize)->([usize;2],usize){
		let mut collect_agent:Vec<[usize;2]>=self.get_agent_chain(&tmp,set,agent);
		let mut least:[usize;2]=[0;2];
		let mut least_pos:usize=0;
		for i in 0..collect_agent.len(){
			if set==0{
				let other_agent:usize=collect_agent[i][1];
				let pos:usize=self.m[agent].iter().position(|x| *x==other_agent).unwrap();
				if pos>least_pos{
					least_pos=pos;
					least=collect_agent[i];
				}
			}
			else{
				let other_agent:usize=collect_agent[i][0];
				let pos:usize=self.w[agent].iter().position(|x| *x==other_agent).unwrap();
				if pos>least_pos{
					least_pos=pos;
					least=collect_agent[i];
				}
			}
		}
		(least,least_pos)
	}
	pub fn get_least2(&self,tmp:&Vec<[usize;2]>,set:usize,agent:usize,used_q:usize)->([usize;2],usize){
		let mut collect_agent:Vec<[usize;2]>=self.get_agent_chain(&tmp,set,agent);
		let mut least:[usize;2]=[0;2];
		let mut least_pos:usize=0;
		for i in 0..collect_agent.len(){
			if set==0{
				let other_agent:usize=collect_agent[i][1];
				let pos:usize=self.m[agent].iter().position(|x| *x==other_agent).unwrap();
				if pos>least_pos{
					least_pos=pos;
					least=collect_agent[i];
				}
			}
			else{
				let other_agent:usize=collect_agent[i][0];
				let pos:usize=self.w[agent].iter().position(|x| *x==other_agent).unwrap();
				if pos>least_pos{
					least_pos=pos;
					least=collect_agent[i];
				}
			}
		}
		if collect_agent.len() != used_q{
			//println!("BIG PROBLEM IN GET_LEAST2, QUOTA MISMATCH!");
			return ([10,10],10);
		}
		(least,least_pos)
	}
	pub fn get_least3(&self,tmp:&Vec<[usize;2]>,set:usize,agent:usize,used_q:usize)->(Option<[usize;2]>,Option<usize>){
		println!("GET LEAST3: SET:{}, AGENT:{}, TMP:{:?}",set,agent,tmp);
		let mut collect_agent:Vec<[usize;2]>=self.get_agent_chain(&tmp,set,agent);
		let mut least:Option<[usize;2]>=None;
		let mut least_pos:Option<usize>=None;
		//println!("set:{}, agent:{}, collect_agent.len():{}",set,agent,collect_agent.len());
		for i in 0..collect_agent.len(){
			if set==0{
				let other_agent:usize=collect_agent[i][1];
				let pos:usize=self.m[agent].iter().position(|x| *x==other_agent).unwrap();
				if least_pos.is_none(){
					least_pos=Some(pos);
					least=Some(collect_agent[i]);
				}
				else{
					if pos>least_pos.unwrap(){
						least_pos=Some(pos);
						least=Some(collect_agent[i]);
					}
				}
			}
			else{
				let other_agent:usize=collect_agent[i][0];
				let pos:usize=self.w[agent].iter().position(|x| *x==other_agent).unwrap();
				if least_pos.is_none(){
					least_pos=Some(pos);
					least=Some(collect_agent[i]);
				}
				else{
					if pos>least_pos.unwrap(){
						least_pos=Some(pos);
						least=Some(collect_agent[i]);
					}
				}
			}
		}
		//println!("SET:{}, AGENT:{}, COLLECT_AGENT:{:?}",set,agent,collect_agent);
		if collect_agent.len() != used_q{
			println!("BIG PROBLEM IN GET_LEAST3, QUOTA MISMATCH!");
			return (Some([10,10]),Some(10));
		}
		(least,least_pos)
	}
	pub fn check_inside(chain:&Vec<[usize;2]>,pair:[usize;2])->bool{
		for i in 0..chain.len(){
			if chain[i][0]==pair[0]{
				if chain[i][1]==pair[1]{
					return true;
				}
			}
		}
		false
	}
	pub fn run_wrapper(&self)->Vec<Vec<[usize;2]>>{
		let qm_start:Vec<usize>=self.qm_orig.clone();
		let qw_start:Vec<usize>=self.qw_orig.clone();
		//let matches=self.run(&vec![],[0,0],&qm_start,&qw_start);
		let matches=self.run2(&vec![],[0,0],&qm_start,&qw_start,&vec![]);
		matches
	}
	pub fn check_quota_consistency(&self,tmp:&Vec<[usize;2]>,qm:&Vec<usize>,qw:&Vec<usize>)->bool{
		//println!("CHECK QUOTA CONSISTENCY");
		let mut consistent:bool=true;
		for i in 0..qm.len(){
			let mut count:usize=0;
			for j in 0..tmp.len(){
				if tmp[j][0]==i{
					count+=1;
				}
			}
			if count != self.qm_orig[i]-qm[i]{
				println!("####### BIG ERROR IN QUOTA QM AT I={}, QM:{:?}, QW:{:?}, TMP:{:?}",i,qm,qw,tmp);
				return false;
			}
		}
		for i in 0..qw.len(){
			let mut count:usize=0;
			for j in 0..tmp.len(){
				if tmp[j][1]==i{
					count+=1;
				}
			}
			if count != self.qw_orig[i]-qw[i]{
				println!("####### BIG ERROR IN QUOTA QW AT I={}, QM:{:?}, QW:{:?}, TMP:{:?}",i,qm,qw,tmp);
				return false;
			}
		}
		true
	}
	pub fn run(&self,tmp:&Vec<[usize;2]>,pair_prev:[usize;2],qm_:&Vec<usize>,qw_:&Vec<usize>)->Vec<Vec<[usize;2]>>{
		let n:usize=self.n;
		let mut qm:Vec<usize>=qm_.clone();
		let mut qw:Vec<usize>=qw_.clone();
		let mut matches:Vec<Vec<[usize;2]>>=vec![];
		println!("RUN CALL !!! !!! !!! !!! PAIR:{:?}, TMP:{:?}\nQM:{:?}, QW:{:?}\n",pair_prev,tmp,qm_,qw_);
		//if pair_prev==[n-1,n-1]{
		if *qm_==vec![0;n] && *qw_==vec![0;n]{
			if self.check_everone_is_matched(&tmp){
				return vec![tmp.to_vec()];
			}
			else{
				return vec![];
			}
		}
		else{
			for i in 0..self.m.len(){
				for j in 0..self.w.len(){
					let pair:[usize;2]=[i,j];
					if !tmp.contains(&pair){
					//if !Self::check_inside(&tmp,pair){
						let mut stable:bool=true;
						for k in 0..tmp.len(){
							let tmpm:usize=tmp[k][0];
							let tmpw:usize=tmp[k][1];
							if !self.bp[tmpm*n+tmpw][i*n+j]{
								stable=false;
								break;
							}
						}
						if stable{
							//let mut new_tmp:Vec<[usize;2]>=tmp.clone();
							let mut still_ok:bool=true;
							let (leastm,leastm_pos):(Option<[usize;2]>,Option<usize>)=self.get_least3(&tmp,0,i,self.qm_orig[i]-qm[i]);
							let (leastw,leastw_pos):(Option<[usize;2]>,Option<usize>)=self.get_least3(&tmp,1,j,self.qw_orig[j]-qw[j]);
							if qm[i]==0{
								let pos:usize=self.m[i].iter().position(|x| *x==j).unwrap();
								if !leastm_pos.is_none(){
									if pos>leastm_pos.unwrap(){
										still_ok=false;
									}
									else{
										//println!("WILL ACCEPT IT! QM[{}]:{:?}",i,qm);
									}
								}
								else{
									//println!("QUOTA MISMATCH IN QM[{}]! QM:{:?}",i,qm);
								}
							}
							if qw[j]==0{
								let pos:usize=self.w[j].iter().position(|x| *x==i).unwrap();
								if !leastw_pos.is_none(){
									if pos>leastw_pos.unwrap(){
										still_ok=false;
									}
									else{
										//println!("WILL ACCEPT IT! QW[{}]:{:?}",j,qw);
									}
								}
								else{
									//println!("QUOTA MISMATCH IN QW[{}]! QW:{:?}",j,qw);
								}
								
							}
							if still_ok{
								let mut new_tmp:Vec<[usize;2]>=tmp.clone();
								let mut delvec:Vec<usize>=vec![];
								let mut to_incr_qm:Vec<usize>=vec![];
								let mut to_incr_qw:Vec<usize>=vec![];
								if qm[i]==0{
									//println!("i:{}, j:{}, LEASTM:{:?}",i,j,leastm);
									let delposm:usize=tmp.iter().position(|x| *x==leastm.unwrap()).unwrap();
									//qm[i]+=1;
									to_incr_qm.push(i);
									//qw[leastm.unwrap()[1]]+=1;
									to_incr_qw.push(leastm.unwrap()[1]);
									if qw[leastm.unwrap()[1]]>self.qw_orig[leastm.unwrap()[1]]{
										println!("BIG PROBLEM! QW_ORIG EXCEEDED! i:{}, j:{}",i,j);
									}
									//println!("######## KICK OUT LEASTM: {:?}",leastm);
									delvec.insert(0,delposm);
								}
								if qw[j]==0{
									let delposw:usize=tmp.iter().position(|x| *x==leastw.unwrap()).unwrap();
									//qw[j]+=1;
									to_incr_qw.push(j);
									//qm[leastw.unwrap()[0]]+=1;
									to_incr_qm.push(leastw.unwrap()[0]);
									if qm[leastw.unwrap()[0]]>self.qm_orig[leastw.unwrap()[0]]{
										println!("BIG PROBLEM! QM_ORIG EXCEEDED! i:{}, j:{}",i,j);										
									}
									//println!("######## KICK OUT LEASTW: {:?}",leastw);
									delvec.insert(0,delposw);
								}
								let mut qm_incr:Vec<usize>=qm_.clone();
								let mut qw_incr:Vec<usize>=qw_.clone();
								for k in 0..to_incr_qm.len(){
									qm_incr[to_incr_qm[k]]+=1;
								}
								for k in 0..to_incr_qw.len(){
									qw_incr[to_incr_qw[k]]+=1;
								}

								for delidx in 0..delvec.len(){
									new_tmp.remove(delvec[delidx]);
								}
								//println!("BEFORE CHANGING PARAMETER!");
								self.check_quota_consistency(&new_tmp,&qm,&qw);
								new_tmp.push([i,j]);
								//qm[i]-=1;
								qm_incr[i]-=1;
								//qw[j]-=1;
								qw_incr[j]-=1;
								//println!("AFTER CHANGING PARAMETER!");
								let consistent:bool=self.check_quota_consistency(&new_tmp,&qm_incr,&qw_incr);
								println!("### i:{}, j:{}, QM[{}]:{:?}, QW[{}]:{:?}], TMP:{:?}, QM:{:?}, QW:{:?}",i,j,i,qm[i],j,qw[j],new_tmp,qm,qw);
								//matches.append(&mut self.run(&new_tmp,[i,j],&qm,&qw));
								if consistent{
									matches.append(&mut self.run(&new_tmp,[i,j],&qm_incr,&qw_incr));
								}
								else{
									println!("######################## NOT CONSISTENT! ##########################");
								}
							}
						}
					}
				}
			}
		}
		matches
	}
	pub fn run2(&self,tmp:&Vec<[usize;2]>,pair_prev:[usize;2],qm_:&Vec<usize>,qw_:&Vec<usize>,not_working_pairs_:&Vec<[usize;2]>)->Vec<Vec<[usize;2]>>{
		let n:usize=self.n;
		let mut qm:Vec<usize>=qm_.clone();
		let mut qw:Vec<usize>=qw_.clone();
		let mut matches:Vec<Vec<[usize;2]>>=vec![];
		//println!("RUN CALL !!! !!! !!! !!! PAIR:{:?}, TMP:{:?}\nQM:{:?}, QW:{:?}\n",pair_prev,tmp,qm_,qw_);
		//if pair_prev==[n-1,n-1]{
		if *qm_==vec![0;n] && *qw_==vec![0;n]{
			if self.check_everone_is_matched(&tmp){
				return vec![tmp.to_vec()];
			}
			else{
				return vec![];
			}
		}
		else{
			for i in 0..self.m.len(){
				for j in 0..self.w.len(){
					let pair:[usize;2]=[i,j];
					if !not_working_pairs_.contains(&pair){
					if !tmp.contains(&pair){
					//if !Self::check_inside(&tmp,pair){
						let mut stable:bool=true;
						for k in 0..tmp.len(){
							let tmpm:usize=tmp[k][0];
							let tmpw:usize=tmp[k][1];
							if !self.bp[tmpm*n+tmpw][i*n+j]{
								stable=false;
								break;
							}
						}
						if stable{
							//let mut new_tmp:Vec<[usize;2]>=tmp.clone();
							let mut still_ok:bool=true;
							let (leastm,leastm_pos):(Option<[usize;2]>,Option<usize>)=self.get_least3(&tmp,0,i,self.qm_orig[i]-qm[i]);
							let (leastw,leastw_pos):(Option<[usize;2]>,Option<usize>)=self.get_least3(&tmp,1,j,self.qw_orig[j]-qw[j]);
							if qm[i]==0{
								let pos:usize=self.m[i].iter().position(|x| *x==j).unwrap();
								if !leastm_pos.is_none(){
									if pos>leastm_pos.unwrap(){
										still_ok=false;
									}
									else{
										println!("WILL ACCEPT IT! QM[{}]:{:?}",i,qm);
									}
								}
								else{
									//println!("QUOTA MISMATCH IN QM[{}]! QM:{:?}",i,qm);
								}
							}
							if qw[j]==0{
								let pos:usize=self.w[j].iter().position(|x| *x==i).unwrap();
								if !leastw_pos.is_none(){
									if pos>leastw_pos.unwrap(){
										still_ok=false;
									}
									else{
										println!("WILL ACCEPT IT! QW[{}]:{:?}",j,qw);
									}
								}
								else{
									//println!("QUOTA MISMATCH IN QW[{}]! QW:{:?}",j,qw);
								}
								
							}
							if still_ok{
								let mut new_tmp:Vec<[usize;2]>=tmp.clone();
								let mut delvec:Vec<usize>=vec![];
								let mut to_incr_qm:Vec<usize>=vec![];
								let mut to_incr_qw:Vec<usize>=vec![];
								if qm[i]==0{
									println!("i:{}, j:{}, LEASTM:{:?}",i,j,leastm);
									let delposm:usize=tmp.iter().position(|x| *x==leastm.unwrap()).unwrap();
									//qm[i]+=1;
									to_incr_qm.push(i);
									//qw[leastm.unwrap()[1]]+=1;
									to_incr_qw.push(leastm.unwrap()[1]);
									if qw[leastm.unwrap()[1]]>self.qw_orig[leastm.unwrap()[1]]{
										println!("BIG PROBLEM! QW_ORIG EXCEEDED! i:{}, j:{}",i,j);
									}
									//println!("######## KICK OUT LEASTM: {:?}",leastm);
									delvec.insert(0,delposm);
								}
								if qw[j]==0{
									let delposw:usize=tmp.iter().position(|x| *x==leastw.unwrap()).unwrap();
									//qw[j]+=1;
									to_incr_qw.push(j);
									//qm[leastw.unwrap()[0]]+=1;
									to_incr_qm.push(leastw.unwrap()[0]);
									if qm[leastw.unwrap()[0]]>self.qm_orig[leastw.unwrap()[0]]{
										println!("BIG PROBLEM! QM_ORIG EXCEEDED! i:{}, j:{}",i,j);										
									}
									println!("######## KICK OUT LEASTW: {:?}",leastw);
									delvec.insert(0,delposw);
								}
								let mut qm_incr:Vec<usize>=qm_.clone();
								let mut qw_incr:Vec<usize>=qw_.clone();
								for k in 0..to_incr_qm.len(){
									qm_incr[to_incr_qm[k]]+=1;
								}
								for k in 0..to_incr_qw.len(){
									qw_incr[to_incr_qw[k]]+=1;
								}
								let mut not_working_pairs:Vec<[usize;2]>=not_working_pairs_.clone();
								for delidx in 0..delvec.len(){
									not_working_pairs.push(new_tmp[delvec[delidx]]);
									new_tmp.remove(delvec[delidx]);
								}
								//println!("BEFORE CHANGING PARAMETER!");
								self.check_quota_consistency(&new_tmp,&qm,&qw);
								new_tmp.push([i,j]);
								//qm[i]-=1;
								qm_incr[i]-=1;
								//qw[j]-=1;
								qw_incr[j]-=1;
								//println!("AFTER CHANGING PARAMETER!");
								let consistent:bool=self.check_quota_consistency(&new_tmp,&qm_incr,&qw_incr);
								//println!("### i:{}, j:{}, QM[{}]:{:?}, QW[{}]:{:?}], TMP:{:?}, QM:{:?}, QW:{:?}",i,j,i,qm[i],j,qw[j],new_tmp,qm,qw);
								//matches.append(&mut self.run(&new_tmp,[i,j],&qm,&qw));
								if consistent{
									matches.append(&mut self.run2(&new_tmp,[i,j],&qm_incr,&qw_incr,&not_working_pairs));
								}
								else{
									println!("######################## NOT CONSISTENT! ##########################");
								}
							}
						}
					}
					}
				}
			}
		}
		matches
	}	
}

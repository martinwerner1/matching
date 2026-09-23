#![allow(warnings)]

pub fn test(){
	//let outcome:Vec<Vec<[usize;2]>>=lattice_layer(&vec![],0,4);
	let outcome_orig:Vec<Vec<[usize;2]>>=lattice_layer_diff(&vec![],0,4,3);
	let outcome:Vec<Vec<[usize;2]>>=increment_layer(&outcome_orig,2);
	println!("outcome");
	for i in 0..outcome.len(){
		println!("i:{} # {:?}",i,outcome[i]);
	}
}

pub fn lattice_layer(tmp:&Vec<[usize;2]>,idx:usize,n:usize)->Vec<Vec<[usize;2]>>{
	let mut outcome:Vec<Vec<[usize;2]>>=vec![];
	if idx==n{
		return vec![tmp.to_vec()];
	}
	else{
		let mut vec:[usize;2]=[idx,0];
		for j in 0..n{
			let mut ok:bool=true;
			for i in 0..tmp.len(){
				if tmp[i][1]==j{
					ok=false;
					break;
				}
			}
			if ok{
				vec[1]=j;
				let mut new_tmp:Vec<[usize;2]>=tmp.clone();
				new_tmp.push(vec);
				outcome.append(&mut lattice_layer(&new_tmp,idx+1,n));
			}
		}
	}
	outcome
}

pub fn lattice_layer_diff(tmp:&Vec<[usize;2]>,idx:usize,n:usize,max:usize)->Vec<Vec<[usize;2]>>{
	let mut outcome:Vec<Vec<[usize;2]>>=vec![];
	if idx==max{
		return vec![tmp.to_vec()];
	}
	else{
		let mut vec:[usize;2]=[idx,0];
		for j in 0..n{
			let mut ok:bool=true;
			for i in 0..tmp.len(){
				if tmp[i][1]==j{
					ok=false;
					break;
				}
			}
			if ok{
				vec[1]=j;
				let mut new_tmp:Vec<[usize;2]>=tmp.clone();
				new_tmp.push(vec);
				outcome.append(&mut lattice_layer_diff(&new_tmp,idx+1,n,max));
			}
		}
	}
	outcome
}
pub fn increment_layer(vec:&Vec<Vec<[usize;2]>>,incr:usize)->Vec<Vec<[usize;2]>>{
	let mut new:Vec<Vec<[usize;2]>>=vec.clone();
	for i in 0..new.len(){
		for j in 0..new[i].len(){
			new[i][j][0]+=incr;
		}
	}
	new
}

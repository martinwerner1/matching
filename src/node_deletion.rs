#![allow(warnings)]

use super::helper::{IOClass};
use super::image;
use super::gale_shapley;
use ndarray;

fn node_deletion_complete(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let emptyvertices=node_deletion_empty_vertices(&bp);
	//println!("EMPTYVERTICES: {:?}",emptyvertices);
	let res=node_deletion_bp(&bp,&emptyvertices);
	res
}

pub fn node_deletion_empty_vertices(bp:&Vec<Vec<bool>>)->Vec<([usize;2],usize)>{
	let n:usize=bp.len().isqrt();
	let mut empty_vertices:Vec<([usize;2],usize)>=vec![];
	for i in 0..bp.len(){
		for j in 0..n{
			if i/n !=j{
				if bp[i][j*n]==false{
					let mut false_count:usize=0;
					for k in 0..n{
						if bp[i][j*n+k]==false{
							false_count+=1;
						}
						else{
							break;
						}
					}
					if false_count==n{
						empty_vertices.push(([i/n,i%n],j));
					}
				}
			}
		}
	}
	empty_vertices
}
fn node_deletion_bp(bp:&Vec<Vec<bool>>,emptyset:&Vec<([usize;2],usize)>)->Vec<Vec<bool>>{
	let mut new_bp:Vec<Vec<bool>>=bp.clone();
	let n:usize=bp.len().isqrt();
	for i in 0..emptyset.len(){
		let (vertices,player)=emptyset[i];
		let m:usize=vertices[0];
		let w:usize=vertices[1];
		new_bp[m*n+w]=vec![false;n*n];
		for j in 0..bp.len(){
			new_bp[j][m*n+w]=false;
		}
	}
	new_bp
}
fn transform_bp_gender(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let n:usize=bp.len().isqrt();
	let mut new_bp:Vec<Vec<bool>>=vec![vec![false;n*n];n*n];
	for m1 in 0..n{
		for w1 in 0..n{
			for m2 in 0..n{
				for w2 in 0..n{
					if bp[m1*n+w1][m2*n+w2]{
						new_bp[w1*n+m1][w2*n+m2]=true;
					}
				}
			}
		}
	}
	new_bp
}








pub fn bp_matrix_for_women(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let mut wbp:Vec<Vec<bool>>=vec![];
	let n:usize=bp.len().isqrt();
	for i in 0..n{
		for j in 0..n{
			let row:usize=j*n+i;
			let mut wbp_row:Vec<bool>=vec![];
			for k in 0..n{
				for l in 0..n{
					wbp_row.push(bp[row][k+l*n]);
				}
			}
			wbp.push(wbp_row);
		}
	}
	wbp
}
fn clear_bp_interdependence(bp:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	println!("CLEAR BP INTERDEPENDENCE");
	let mut change_bp:Vec<Vec<bool>>=transform_bp_gender(&bp);
	let mut change_bp_cleared:Vec<Vec<bool>>=clear_bp(&change_bp);
	let mut bp_back:Vec<Vec<bool>>=transform_bp_gender(&change_bp_cleared);
	let mut bp_back_cleared:Vec<Vec<bool>>=clear_bp(&bp_back);
	if bp_back_cleared==*bp{
		return bp_back_cleared;
	}
	else{
		return clear_bp_interdependence(&bp_back_cleared);
	}
}
fn clear_bp(bp_:&Vec<Vec<bool>>)->Vec<Vec<bool>>{
	let mut bp=bp_.clone();
	let mut bp_tmp=node_deletion_complete(&bp);
	while bp_tmp != bp{
		bp=bp_tmp.clone();
		bp_tmp=node_deletion_complete(&bp);
	}
	bp_tmp
}

pub fn get_nodes_idx(bp:&Vec<Vec<bool>>)->Vec<usize>{
	let mut nodes_idx:Vec<usize>=vec![];
	for i in 0..bp.len(){
		let mut anynodes_there:bool=false;
		for j in 0..bp[i].len(){
			if bp[i][j]{
				anynodes_there=true;
				break;
			}
		}
		if anynodes_there{
			nodes_idx.push(i);
		}
	}
	nodes_idx
}
pub fn tests(){
	let bp=IOClass::get_bp_automatically();
	let wbp=transform_bp_gender(&bp);
	//let wbp2=bp_matrix_for_women(&bp);
	//println!("WOMEN BP THE SAME:{}",wbp1==wbp2);
	
	image::img_bp_matrix_lines(&bp,"pic/bp_test/mbp.png".to_string());
	image::img_bp_matrix_lines(&wbp,"pic/bp_test/wbp.png".to_string());
	//image::img_bp_matrix_lines(&wbp2,"pic/bp_test/wbp2.png".to_string());
	
	let bp1=node_deletion_complete(&bp);
	let wbp1=transform_bp_gender(&bp1);
	let wbp1=node_deletion_complete(&wbp1);
	image::img_bp_matrix_lines(&bp1,"pic/bp_test/mbp1.png".to_string());
	image::img_bp_matrix_lines(&wbp1,"pic/bp_test/wbp1.png".to_string());
	
	println!("START NODE DELETION!");
	let bp_interdependence=clear_bp_interdependence(&bp);
	println!("BP INTERDEPENDENCE");
	for i in 0..bp_interdependence.len(){
		//println!("{:?}",bp_interdependence[i]);
	}
	image::img_bp_matrix_lines(&bp_interdependence,"pic/bp_test/mbp_interdependence.png".to_string());
	let nodes_idx:Vec<usize>=get_nodes_idx(&bp_interdependence);
	let ratio:f64=nodes_idx.len() as f64/bp.len() as f64;
	println!("COUNT NODES:{}\nRATIO: {}",nodes_idx.len(),format!("{:2}",ratio));
	
	
	let m=IOClass::read_txt("pref/m.txt".to_string());
	let w=IOClass::read_txt("pref/m.txt".to_string());
	let gslists:gale_shapley::gs_lists=gale_shapley::gs_lists::new(&m,&w);
	println!("GS NODECOUNT: {}",gslists.get_nodecount());
	
}

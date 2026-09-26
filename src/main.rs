#![allow(warnings)]
use display::helper::{IOClass};
use display::lattice::{lattice};
use display::lp::{one2one,many2one};
use display::*;
use display::lattice_display;
use display::node_deletion;
use display::lattice_matching;
use display::matchref;
use display::bitsync;
use display::deep_search;

fn main(){

	// N-SIDED MATCHING BASED ON INTERSECTION METHOD, RANDOM PREFERENCES
	N_sided_matching::test_deepsearch_n_sided();

	// CREATE LATEX CODE FOR ILLUSTRATION OF VERTEX DELETION MECHANISM, PACKAGE:PSTRICKS
	//test_edges_bp4();

	// LINEAR PROGRAMMING
	//test_lp();
	
	// VERTEX DELETION MECHANISM FOR REMOVING UNSTABLE PAIRS
	//node_deletion::tests();
	
	// CREATE LATEX CODE FOR STABILITY MATRIX (PACKAGE:PSTRICKS), UNCOMMENT BOTH LINES FOR USAGE
	//let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	//tex::psgrid_test(&bp,false); // second parameter -> false: full stability matrix, true: upper half of stability matrix (needed for bitsync)
	
	// GS ALGORITHM WITH GS REDUCED LISTS
	//gale_shapley::test();

	// DISPLAY OF THE LATTICE STRUCTURE AT GIVEN SAMPLE
	//lattice_display::test();
	
	// MATCHREF ALGORITHM WITH REFERENCE/SMART POINTER
	//matchref::matchref_run();
	
	// LATTICE MATCHING ALGORITHM WITH HASH VALUES (SLOW MODE)
	//matchref::test_hashmatch();
	
	// DEEPSEARCH FOR MANY-TO-MANY MATCHING, UNDER CONSTRUCTION!
	//deep_search::test();
	
	// BITSYNC ALGORITHM
	//bitsync::test();
}

fn test_lattice(){
	let ltc:lattice=lattice::init();
	ltc.print_all_pref();
	lattice::show_hash();
}
fn test_lp(){
	//let lp_match:Vec<usize>=one2one::run_lp();
	let mut lp:one2one=one2one::init();
	lp.update();
	lp.run_lp();
	
	/*
	let lpmatch:Vec<usize>=lp.lp_go();
	let allbits=one2one::get_all_bits(&vec![],0,4);
	println!("ALLBITS\n{:?}",allbits);
	let pb=lp.pb.clone();
	lp.enumerate_from_match(&vec![],vec![0,3,4,2,1]);
	
	let mut manyLP:many2one=many2one::new();
	manyLP.update();
	let resultLP:Vec<usize>=manyLP.prepare_LP_highs_many2one();
	
	let all_poss:Vec<Vec<usize>>=many2one::all_poss_idx_for_comb(3,3,0,&vec![]);
	println!("all_poss\n{:?}",all_poss);
	//let (A_le,A_le_vec,check_vec,check_poss)=manyLP.stability_sethumaran_A_le();
	let (A_le,A_le_vec,check_vec,check_poss)=manyLP.stability_balinski_A_le_lemma1();
	println!("####### A_LE");
	for i in 0..A_le.len(){
		println!("{:?} ### q:{} #### {:?} #### poss:{:?}",A_le[i],A_le_vec[i],check_vec[i],check_poss[i]);
	}
	//println!("####### A_LE VEC: {:?}",A_le_vec);
	//println!("####### CHECK VEC: {:?}",check_vec);
	println!("MATCH RESULT MANY2ONE LP\n{:?}",resultLP);
	*/
}


fn test_edges_bp(){
	let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	println!("bp:{:?}",bp);
	let edges:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,1);
	let edges2:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,2);
	println!("edges:");
	print_edges(&edges);
	let edges_matrix_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges);
	let edges_matrix:Vec<Vec<[[usize;2];2]>>=get_diff2_edge_matrix(&edges_matrix_orig,0);
	println!("EDGES_MATRIX: {:?}",edges_matrix);
	let vertices:Vec<[usize;2]>=get_vertices(5,6);
	draw_graph(&vertices,&edges,0,&vec![]);
	let m:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
	let w:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
	//pref_display(&m,0);
	//pref_display_both(&m,&w,0);
	//let (sort_edges,reddashed)=sort_edges2(&edges);
	

	let (sort_edm,dashed)=sort_edges2(&edges);
	let sort_edges=edm_2_edgevec(&sort_edm);
	let dashed_vec:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed);

	let (sort_edm2,dashed2)=sort_edges2(&sort_edges);
	let sort_edges2=edm_2_edgevec(&sort_edm2);
	let dashed_vec2:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed2);


	println!("EDGE MATRIX SORT");
	for i in 0..sort_edm.len(){
		println!("i:{}, {:?}",i,sort_edm[i]);
	}
	println!("DASHED VEC: {:?}",dashed_vec);
	println!("sort edges");
	print_edges(&sort_edges);
	
	draw_graph(&vertices,&sort_edges,0,&dashed_vec2);
}


fn test_edges_bp2(){
	let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	let edges:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,1);
	let edges_matrix_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges);	
	let vertices:Vec<[usize;2]>=get_vertices(5,7);
	println!("VERTICES AT THE VERY BEGINNING: {:?}",vertices);
	let (sort_edm,dashed)=sort_edges3(&edges_matrix_orig,0,5);	
	let edges2:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,2);
	let dashed_vec:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed);	
	let edges_matrix_diff2:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges2);
	println!("EDGE MATRIX DIFF2 ORIG:{:?}",edges_matrix_diff2);	
	let vertices2:Vec<[usize;2]>=remove_vertices_notin_edge_matrix(&vertices,&sort_edm,6);
	println!("EDGE_MATRIX_DIFF2:{:?}",edges_matrix_diff2);
	println!("VERTICES REMOVED!: {:?}",vertices2);
	let edges_matrix_diff2_clear_compl:Vec<Vec<[[usize;2];2]>>=clean_edge_matrix(&edges_matrix_diff2,&vertices2);
	let edges_matrix_diff2_clear=get_diff2_edge_matrix(&edges_matrix_diff2_clear_compl,0);	
	println!("EDM DIFF2 CLEAR:{:?}",edges_matrix_diff2_clear);
	let (sort_edm2,dashed2)=sort_edges3(&edges_matrix_diff2_clear,1,3);
	println!("DASHED2: {:?}",dashed2);
	let dashed_vec2:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed2);
	let sort_edges2:Vec<[[usize;2];2]>=edm_2_edgevec(&sort_edm2);	
	draw_graph(&vertices2,&sort_edges2,0,&dashed_vec2);	
}
fn test_edges_bp3(){
	let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	let edges:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,1);
	let edges_matrix_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges);
	
	let vertices:Vec<[usize;2]>=get_vertices(5,7);
	println!("VERTICES AT THE VERY BEGINNING: {:?}",vertices);
	let (sort_edm,dashed)=sort_edges3(&edges_matrix_orig,0,5);
	let edges2:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,2);
	
	let dashed_vec:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed);
	let edges_matrix_diff2:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges2);	
	println!("EDGE MATRIX DIFF2 ORIG:{:?}",edges_matrix_diff2);
	let vertices2:Vec<[usize;2]>=remove_vertices_notin_edge_matrix(&vertices,&sort_edm,6);
	println!("EDGE_MATRIX_DIFF2:{:?}",edges_matrix_diff2);
	println!("VERTICES REMOVED!: {:?}",vertices2);
	let edges_matrix_diff2_clear_compl:Vec<Vec<[[usize;2];2]>>=clean_edge_matrix(&edges_matrix_diff2,&vertices2);
	let edges_matrix_diff2_clear=get_diff2_edge_matrix(&edges_matrix_diff2_clear_compl,0);	
	println!("EDM DIFF2 CLEAR:{:?}",edges_matrix_diff2_clear);
	let (sort_edm2,dashed2)=sort_edges3(&edges_matrix_diff2_clear,1,3);
	println!("DASHED2: {:?}",dashed2);
	let dashed_vec2:Vec<[[usize;2];2]>=edm_2_edgevec(&dashed2);
	let sort_edges2:Vec<[[usize;2];2]>=edm_2_edgevec(&sort_edm2);
	draw_graph(&vertices2,&sort_edges2,0,&dashed_vec2);
}
fn test_edges_bp4(){
	let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	let edges:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,1);
	let edm_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges);
	
	let edges2:Vec<[[usize;2];2]>=get_edges_from_bp(&bp,2);
	let vertices:Vec<[usize;2]>=get_vertices(5,7);
	// DIFF = 2
	let edm2_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edges2);
	let edm2a=get_diff2_edge_matrix(&edm2_orig,1);
	let edm2b=get_diff2_edge_matrix(&edm2_orig,0);
	
	// FOR DIFF=1 -> WORKS!
	let (new_v,new_edm,dashed)=edm_transform_wrapper(&vertices,&edm_orig,0,4,6,&[1,0]);
	
	println!("THE FIRST NETWORK ###");
	
	draw_graph_wrapper(&vertices,&new_edm,&dashed,0,&[1,0],0.0,14.0,"mdiff1".to_string());
	
	println!("NEW V:{:?}",new_v);
	
	let (new_v2a,new_edm2a,dashed2a)=edm_transform_wrapper(&new_v,&edm2a,0,4,6,&[2,0]);
	draw_graph_wrapper(&new_v,&new_edm2a,&dashed2a,0,&[2,0],0.0,7.0,"mdiff2a".to_string());
	
	println!("NEW V2A:{:?}",new_v2a);
	
	let (new_v2b,new_edm2b,dashed2b)=edm_transform_wrapper(&new_v2a,&edm2b,1,3,6,&[2,1]);
	draw_graph_wrapper(&new_v2a,&new_edm2b,&dashed2b,0,&[2,1],0.0,0.0,"mdiff2b".to_string());
	println!("NEW V2B:{:?}",new_v2b);

	let vw=invert_vm_wrapper(&new_v2b);
	println!("WOMEN VERTICES: {:?}",vw);
	let bpw=node_deletion::bp_matrix_for_women(&bp);
	let edgesw:Vec<[[usize;2];2]>=get_edges_from_bp(&bpw,1);
	let edmw_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edgesw);
	
	println!("EDMW_ORIG: {:?}",edmw_orig);
	// DIFF=2 FOR WOMEN
	let edgesw2:Vec<[[usize;2];2]>=get_edges_from_bp(&bpw,2);
	let edmw2_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&edgesw2);
	let edmw2a=get_diff2_edge_matrix(&edmw2_orig,1);
	let edmw2b=get_diff2_edge_matrix(&edmw2_orig,0);

	let (new_vw,new_edmw,dashedw)=edm_transform_wrapper(&vw,&edmw_orig,0,4,6,&[1,0]);
	draw_graph_wrapper(&vw,&new_edmw,&dashedw,1,&[1,0],9.0,14.0,"wdiff1".to_string());
	
	let (new_vw2a,new_edmw2a,dashedw2a)=edm_transform_wrapper(&new_vw,&edmw2a,0,4,6,&[2,0]);
	draw_graph_wrapper(&new_vw,&new_edmw2a,&dashedw2a,1,&[2,0],9.0,7.0,"wdiff2a".to_string());
	
	let (new_vw2b,new_edmw2b,dashedw2b)=edm_transform_wrapper(&new_vw2a,&edmw2b,1,3,6,&[2,1]);
	draw_graph_wrapper(&new_vw2a,&new_edmw2b,&dashedw2b,1,&[2,1],9.0,0.0,"wdiff2b".to_string());
	
}

pub fn draw_graph_wrapper(v:&Vec<[usize;2]>,edm:&Vec<Vec<[[usize;2];2]>>,dashed:&Vec<Vec<[[usize;2];2]>>,mode:usize,colmode:&[usize;2],xshift:f64,yshift:f64,grouplabel:String){
	let dashed_vec=edm_2_edgevec(&dashed);
	let edges=edm_2_edgevec(&edm);
	//draw_graph2(&v,&edges,mode,&dashed_vec,&colmode);
	draw_graph3(&v,&edges,mode,&dashed_vec,&colmode,xshift,yshift,grouplabel);
}
pub fn edm_transform_wrapper(v_:&Vec<[usize;2]>,edm_orig_:&Vec<Vec<[[usize;2];2]>>,
	lower_bound:usize,upper_bound:usize,donotremove:usize,colmode:&[usize;2])->(
	Vec<[usize;2]>,Vec<Vec<[[usize;2];2]>>,Vec<Vec<[[usize;2];2]>>){
	
	let clear_edm=clean_edge_matrix(&edm_orig_,&v_);
	let (sort_edm,dashed)=sort_edges3(&clear_edm,lower_bound,upper_bound);
	//let v_clear=remove_vertices_notin_edge_matrix(&v_,&sort_edm,donotremove);
	let v_clear=remove_vertices_notin_edge_matrix2(&v_,&sort_edm,donotremove,colmode);
	(v_clear,sort_edm,dashed)
}

fn print_nodenames_all(){
	print_nodenames();
	print_nodenames_horizontal();
	print_nodenames_men_side();
	print_nodenames_horizontal_men_side();

}
fn print_node_coord(){
	let xshift:usize=8;
	display(xshift);
	display_1st_a(xshift);
	display_2nd_a(xshift);
	display_2nd_b(xshift);	
}

fn display(xshift:usize){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=19;
	//let xshift:usize=7;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("w{}{}",j+1,ny-i).to_string();
			let xcoord:f64=(xshift+j) as f64*xfactor;
			println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
			
			//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

		}
	}
}


fn print_nodenames_men_side(){
	let xstart:f64=-0.5;
	let ystart:usize=0;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;

	let ny:usize=5;
	let nx:usize=5;
	for i in 0..ny{
		let nodename:String=format!("w_{{{}}}",ny-i).to_string();
		let xcoord:f64=(xstart) as f64*xfactor;
		let ycoord:f64=(ystart+i) as f64*yfactor;
		//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
		//println!("\\rput({},{}){{\\scriptsize ${}$}}",xcoord,ycoord,nodename);
		println!("\\rput({},{}){{\\tiny ${}$}}",xcoord,ycoord,nodename);

	}
}
fn print_nodenames_horizontal_men_side(){
	let xstart:f64=0.0;
	let ystart:f64=16.7;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let ny:usize=5;
	let nx:usize=7;
	for i in 0..nx{
		//let nodename:String=format!("m_{{{}}}",ny-i).to_string();
		let nodename:String=format!("m_{{{}}}",i+1).to_string();
		let xcoord:f64=(xstart+i as f64) as f64*xfactor;
		let ycoord:f64=(ystart) as f64*yfactor;
		//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
		//println!("\\rput({},{}){{\\scriptsize ${}$}}",xcoord,ycoord,nodename);
		println!("\\rput({},{}){{\\tiny ${}$}}",xcoord,ycoord,nodename);

	}
}

fn print_nodenames(){
	let xstart:f64=7.5;
	let ystart:usize=19;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;

	let ny:usize=5;
	let nx:usize=5;
	for i in 0..ny{
		let nodename:String=format!("m_{{{}}}",ny-i).to_string();
		let xcoord:f64=(xstart) as f64*xfactor;
		let ycoord:f64=(ystart+i) as f64*yfactor;
		//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
		//println!("\\rput({},{}){{\\scriptsize ${}$}}",xcoord,ycoord,nodename);
		println!("\\rput({},{}){{\\tiny ${}$}}",xcoord,ycoord,nodename);

	}
}
fn print_nodenames_horizontal(){
	let xstart:f64=8.0;
	let ystart:f64=23.7;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;

	let ny:usize=5;
	let nx:usize=7;
	for i in 0..nx{
		//let nodename:String=format!("m_{{{}}}",ny-i).to_string();
		let nodename:String=format!("w_{{{}}}",i+1).to_string();
		let xcoord:f64=(xstart+i as f64) as f64*xfactor;
		let ycoord:f64=(ystart) as f64*yfactor;
		//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
		//println!("\\rput({},{}){{\\scriptsize ${}$}}",xcoord,ycoord,nodename);
		println!("\\rput({},{}){{\\tiny ${}$}}",xcoord,ycoord,nodename);

	}
}


fn display_1st_a(xshift:usize){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=12;
	//let xshift:usize=7;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("w_1st_{}{}",j+1,ny-i+5).to_string();
			let xcoord:f64=(xshift+j) as f64*xfactor;
			println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
			
			//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

		}
	}
}
fn display_2nd_a(xshift:usize){
	let ny:usize=5;
	let nx:usize=7;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=5;
	//let xshift:usize=7;	
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			if j%2==0{
				let nodename:String=format!("w{}{}",j+1,ny-i+10).to_string();
				let xcoord:f64=(xshift+j) as f64*xfactor;
				println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
				//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

			}
		}
	}
}
fn display_2nd_b(xshift:usize){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=0;
	//let xshift:usize=7;	
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			if (j+1)%2==0{
				let nodename:String=format!("w{}{}",j+1,ny-i+15).to_string();
				let xcoord:f64=(xshift+j) as f64*xfactor;
				println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
				//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
			}
		}
	}
}
fn get_edges_from_bp(bp:&Vec<Vec<bool>>,layer:usize)->Vec<[[usize;2];2]>{
	let mut edges:Vec<[[usize;2];2]>=vec![];
	let n:usize=bp.len().isqrt();
	for i in 0..n{
		for j in 0..n{
			let vertex:[usize;2]=[i,j];
			let row:usize=i*n+j;
			for k in 0..n{
				if bp[row][((i+layer)%n)*n+k]{
					edges.push([vertex,[i+layer,k]]);
				}
			}
		}
	}
	edges
}
fn get_vertices(nx:usize,ny:usize)->Vec<[usize;2]>{
	let mut vertices:Vec<[usize;2]>=vec![];
	for i in 0..ny{
		for j in 0..nx{
			vertices.push([i,j]);
		}
	}
	vertices
}
pub fn get_vertice_matrix(v:&Vec<[usize;2]>)->Vec<Vec<[usize;2]>>{
	let (ny,nx)=get_nx_ny(&v);
	let mut mat:Vec<Vec<[usize;2]>>=vec![vec![];ny];
	for i in 0..v.len(){
		let col:usize=v[i][0];
		//println!("col:{}, i:{}, v[{}]:{:?}, nx:{}, ny:{}, mat.len():{}",col,i,i,v[i],nx,ny,mat.len());
		mat[col].push(v[i]);
	}
	mat
}
pub fn vertice_matrix_2_vec(vm:&Vec<Vec<[usize;2]>>)->Vec<[usize;2]>{
	let mut v:Vec<[usize;2]>=vec![];
	for i in 0..vm.len(){
		for j in 0..vm[i].len(){
			v.push(vm[i][j]);
		}
	}
	v
}
pub fn check_vertice_consistency(vm_:&Vec<Vec<[usize;2]>>,xlen:usize)->Vec<Vec<[usize;2]>>{
	let mut vm:Vec<Vec<[usize;2]>>=vm_.clone();
	for i in 0..vm.len(){
	//for i in 0..xlen{
		if i+xlen<vm.len(){
			let mut delvec:Vec<usize>=vec![];
			for j in 0..vm[i].len(){
				let equiv:[usize;2]=[i+xlen,vm[i][j][1]];
				if !vm[i+xlen].contains(&equiv){
					delvec.insert(0,j);
				}
			}
			for j in 0..delvec.len(){
				vm[i].remove(delvec[j]);
			}
		}
	}
	vm
}
pub fn check_vertice_consistency_wrapper(vm:&Vec<Vec<[usize;2]>>,xlen:usize)->Vec<Vec<[usize;2]>>{
	//let vm=get_vertice_matrix(&v);
	let vm_corrected=check_vertice_consistency(&vm,xlen);
	//let v=vertice_matrix_2_vec(&vm_corrected);
	vm_corrected
}
// tranform the vertices for men to vertices for women (x and y are swapped for the coordinates of vertices)!
pub fn invert_vm(vm_:&Vec<Vec<[usize;2]>>,xlen:usize)->Vec<Vec<[usize;2]>>{
	let mut vm=check_vertice_consistency_wrapper(vm_,xlen);
	let nx:usize=vm.len();
	println!("INVERT VM, NX:{}, XLEN:{}",nx,xlen);
	let mut delvec:Vec<usize>=vec![];
	for i in xlen..vm.len(){
		delvec.insert(0,i);
	}
	for i in 0..delvec.len(){
		vm.remove(delvec[i]);
	}
	// later change xlen for many-to-one matching or for many-to-many matching!
	let mut vm_w:Vec<Vec<[usize;2]>>=vec![vec![];xlen];
	for i in 0..vm.len(){
		for j in 0..vm[i].len(){
			let val:[usize;2]=vm[i][j];
			let val0:usize=val[0];
			let val1:usize=val[1];
			println!("vm_w.len():{}, val1:{}, val0:{}, nx:{}, vm.len():{}",vm_w.len(),val1,val0,nx,vm.len());
			vm_w[val1].push([val1,val0]);
		}
	}
	let diff:usize=nx-xlen;
	for i in 0..diff{
		//let vm_w_i:Vec<[usize;2]>=vm_w[i].clone();
		let mut to_push:Vec<[usize;2]>=vec![];
		for j in 0..vm_w[i].len(){
			to_push.push([i+xlen,vm_w[i][j][1]]);
		}
		
		//vm_w.push(vm_w_i);
		vm_w.push(to_push);
	}
	let (nyw,nxw)=get_nx_ny_by_vm(&vm_w);
	println!("OUTPUT NYW:{}, NXW:{}",nyw,nxw);
	vm_w
}
pub fn invert_vm_wrapper(v:&Vec<[usize;2]>)->Vec<[usize;2]>{
	let (ny,nx)=get_nx_ny(&v);
	let vm=get_vertice_matrix(&v);
	let vm_w=invert_vm(&vm,nx);
	let v_women=vertice_matrix_2_vec(&vm_w);
	v_women
}
fn get_nx_ny(v:&Vec<[usize;2]>)->(usize,usize){
	let mut nx:usize=0;
	let mut ny:usize=0;
	for i in 0..v.len(){
		if v[i][0]>ny{
			ny=v[i][0];
		}
	}
	for i in 0..v.len(){
		if v[i][1]>nx{
			nx=v[i][1];
		}
	}
	(ny+1,nx+1)
}
pub fn get_nx_ny_by_vm(vm:&Vec<Vec<[usize;2]>>)->(usize,usize){
	let nx:usize=vm.len();
	let mut ny:usize=0;
	for i in 0..vm.len(){
		for j in 0..vm[i].len(){
			if vm[i][j][1]>ny{
				ny=vm[i][j][1];
			}
		}
	}
	(ny+1,nx)
	
}
//fn draw_graph(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>){
fn draw_graph(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>,mode:usize,dashed:&Vec<[[usize;2];2]>){
	//let ny:usize=v.len();
	//let nx:usize=v[0].len();
	let pattern:Vec<&str>=vec!["m","w"];
	let (nx,ny):(usize,usize)=get_nx_ny(&v);
	println!("PRINT NX: {} NY:{}",nx,ny);
	//let (ny,nx):(usize,usize)=get_nx_ny(&v);
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=0;
	let xshift:usize=7;	
	println!("vertices:{:?}",v);
	let ycoord_top:f64=(((yshift+ny) as f64)+0.5)*yfactor;

	for j in 0..nx{
	//for j in 0..ny{
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[mode],(j%(ny))+1).to_string();
		let xcoord:f64=(xshift+j) as f64*xfactor;
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord_top,nodename);

	}
	let xcoord:f64=(-0.5+(xshift) as f64)*xfactor;
	for i in 0..ny{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[(mode+1)%2],i+1).to_string();
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
		
	}
	for i in 0..ny{
	//for i in 0..nx{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("w{}{}",j,i).to_string();
			let xcoord:f64=(xshift+j) as f64*xfactor;
			//let xcoord:f64=(xshift+v[i][1]) as f64*xfactor;
			if v.contains(&[j,i]){
				println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
			}
			else{
				//let mut in_dashed:bool=false;
				for k in 0..dashed.len(){
					if dashed[k].contains(&[j,i]){
						println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
						break;
					}
				}
			}
		}
	}
	for i in 0..e.len(){
			let first:[usize;2]=e[i][0];
			let secon:[usize;2]=e[i][1];
			println!("\\ncline{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}
	for i in 0..dashed.len(){
			let first:[usize;2]=dashed[i][0];
			let secon:[usize;2]=dashed[i][1];
			println!("\\ncline[linecolor=red,linestyle=dotted]{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}

}
//fn draw_graph(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>){
fn draw_graph2(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>,mode:usize,dashed:&Vec<[[usize;2];2]>,colmode:&[usize;2]){
	//let ny:usize=v.len();
	//let nx:usize=v[0].len();
	let pattern:Vec<&str>=vec!["m","w"];
	let (nx,ny):(usize,usize)=get_nx_ny(&v);
	println!("PRINT NX: {} NY:{}",nx,ny);
	//let (ny,nx):(usize,usize)=get_nx_ny(&v);
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=0;
	let xshift:usize=7;	
	println!("vertices:{:?}",v);
	let ycoord_top:f64=(((yshift+ny) as f64)+0.5)*yfactor;

	for j in 0..nx{
	//for j in 0..ny{
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[mode],(j%(ny))+1).to_string();
		let xcoord:f64=(xshift+j) as f64*xfactor;
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord_top,nodename);

	}
	let xcoord:f64=(-0.5+(xshift) as f64)*xfactor;
	for i in 0..ny{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[(mode+1)%2],i+1).to_string();
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
		
	}
	for i in 0..ny{
	//for i in 0..nx{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("w{}{}",j,i).to_string();
			let xcoord:f64=(xshift+j) as f64*xfactor;
			//let xcoord:f64=(xshift+v[i][1]) as f64*xfactor;
			if j%colmode[0]==colmode[1]{
				if v.contains(&[j,i]){
					println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
				}
				else{
					//let mut in_dashed:bool=false;
					for k in 0..dashed.len(){
						if dashed[k].contains(&[j,i]){
							println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
							break;
						}
					}
				}
			}
		}
	}
	for i in 0..e.len(){
			let first:[usize;2]=e[i][0];
			let secon:[usize;2]=e[i][1];
			println!("\\ncline{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}
	for i in 0..dashed.len(){
			let first:[usize;2]=dashed[i][0];
			let secon:[usize;2]=dashed[i][1];
			println!("\\ncline[linecolor=red,linestyle=dotted]{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}
}
//fn draw_graph(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>){
fn draw_graph3(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>,mode:usize,dashed:&Vec<[[usize;2];2]>,colmode:&[usize;2],xshift:f64,yshift:f64,grouplabel:String){
	//let ny:usize=v.len();
	//let nx:usize=v[0].len();
	let pattern:Vec<&str>=vec!["m","w"];
	let (nx,ny):(usize,usize)=get_nx_ny(&v);
	println!("PRINT NX: {} NY:{}",nx,ny);
	//let (ny,nx):(usize,usize)=get_nx_ny(&v);
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	//let yshift:usize=0;
	//let xshift:usize=7;	
	println!("vertices:{:?}",v);
	let ycoord_top:f64=(((yshift+ny as f64) as f64)+0.5)*yfactor;

	for j in 0..nx{
	//for j in 0..ny{
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[mode],(j%(ny))+1).to_string();
		let xcoord:f64=(xshift+j as f64) as f64*xfactor;
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord_top,nodename);

	}
	let xcoord:f64=(-0.5+(xshift) as f64)*xfactor;
	for i in 0..ny{
		let ycoord:f64=(yshift+(ny-i) as f64) as f64*yfactor;
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[(mode+1)%2],i+1).to_string();
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
		
	}
	for i in 0..ny{
	//for i in 0..nx{
		let ycoord:f64=(yshift+(ny-i) as f64) as f64*yfactor;
		for j in 0..nx{
			//let nodename:String=format!("w{}{}",j,i).to_string();
			let nodename:String=format!("{}_{}{}",grouplabel,j,i).to_string();
			let xcoord:f64=(xshift+j as f64) as f64*xfactor;
			//let xcoord:f64=(xshift+v[i][1]) as f64*xfactor;
			if j%colmode[0]==colmode[1]{
				if v.contains(&[j,i]){
					println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
				}
				else{
					//let mut in_dashed:bool=false;
					for k in 0..dashed.len(){
						if dashed[k].contains(&[j,i]){
							println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
							break;
						}
					}
				}
			}
		}
	}
	for i in 0..e.len(){
			let first:[usize;2]=e[i][0];
			let secon:[usize;2]=e[i][1];
			println!("\\ncline{{{}_{}{}}}{{{}_{}{}}}",grouplabel,first[0],first[1],grouplabel,secon[0],secon[1]);
	}
	for i in 0..dashed.len(){
			let first:[usize;2]=dashed[i][0];
			let secon:[usize;2]=dashed[i][1];
			println!("\\ncline[linecolor=red,linestyle=dotted]{{{}_{}{}}}{{{}_{}{}}}",grouplabel,first[0],first[1],grouplabel,secon[0],secon[1]);
	}
}
fn draw_graph_diff_two(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>,mode:usize,dashed:&Vec<[[usize;2];2]>){
	//let ny:usize=v.len();
	//let nx:usize=v[0].len();
	let pattern:Vec<&str>=vec!["m","w"];
	let (nx,ny):(usize,usize)=get_nx_ny(&v);
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=0;
	let xshift:usize=7;	
	println!("vertices:{:?}",v);
	let ycoord_top:f64=(((yshift+ny) as f64)+0.5)*yfactor;

	for j in 0..nx{
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[mode],(j%(ny))+1).to_string();
		let xcoord:f64=(xshift+j) as f64*xfactor;
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord_top,nodename);

	}
	let xcoord:f64=(-0.5+(xshift) as f64)*xfactor;
	for i in 0..ny{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		let nodename:String=format!("\\tiny ${}_{{{}}}$",pattern[(mode+1)%2],i+1).to_string();
		println!("\\rput({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
		
	}
	for i in 0..ny{
		let ycoord:f64=(yshift+ny-i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("w{}{}",j,i).to_string();
			let xcoord:f64=(xshift+j) as f64*xfactor;
			//let xcoord:f64=(xshift+v[i][1]) as f64*xfactor;
			println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);			
		}
	}
	for i in 0..e.len(){
			let first:[usize;2]=e[i][0];
			let secon:[usize;2]=e[i][1];
			println!("\\ncline{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}
	for i in 0..dashed.len(){
			let first:[usize;2]=dashed[i][0];
			let secon:[usize;2]=dashed[i][1];
			println!("\\ncline[linecolor=red,linestyle=dotted]{{w{}{}}}{{w{}{}}}",first[0],first[1],secon[0],secon[1]);
	}

}
fn pref_display(pref:&Vec<Vec<usize>>,mode:usize){
	let n:usize=pref.len();
	let pattern:Vec<&str>=vec!["m","w","d"];
	let mut txt:String=String::new();
	for i in 0..n{
		txt+=&format!("${}_{{{}}}: ",pattern[mode],i+1).to_string();
		for j in 0..pref[i].len(){
			txt+=&format!("{}_{{{}}} ",pattern[(mode+1)%2],pref[i][j]+1).to_string();
			if j<pref[i].len()-1{
				txt+="\\succ ";
			}
		}
		txt+="$\\\\ \n";
	}
	println!("{}",txt);
}
fn pref_display_both(m:&Vec<Vec<usize>>,w:&Vec<Vec<usize>>,mode:usize){
	let n:usize=m.len();
	let pattern:Vec<&str>=vec!["m","w","d"];
	let mut txt:String=String::new();
	for i in 0..n{
		txt+=&format!("${}_{{{}}}: ",pattern[0],i+1).to_string();
		for j in 0..m[i].len(){
			txt+=&format!("{}_{{{}}} ",pattern[(0+1)%2],m[i][j]+1).to_string();
			if j<m[i].len()-1{
				txt+="\\succ ";
			}
		}
		txt+="$\\qquad\\qquad";
		txt+=&format!("${}_{{{}}}: ",pattern[1],i+1).to_string();
		for j in 0..w[i].len(){
			txt+=&format!("{}_{{{}}} ",pattern[(1+1)%2],w[i][j]+1).to_string();
			if j<w[i].len()-1{
				txt+="\\succ ";
			}
		}
		txt+="$\\\\ \n";
	}
	println!("{}",txt);
}
fn get_dim_by_edges(e:&Vec<[[usize;2];2]>)->(usize,usize){
	let mut ny:usize=0;
	let mut nx:usize=0;
	for i in 0..e.len(){
		if e[i][1][0]>nx{
			nx=e[i][1][0];
		}
		if e[i][0][1]>ny{
			ny=e[i][0][1];
		}
		if e[i][1][1]>ny{
			ny=e[i][1][1];
		}
	}
	(ny+1,nx+1)
}
//fn edges_matrix(v:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>)->Vec<Vec<[[usize;2];2]>>{
fn edges_matrix(e:&Vec<[[usize;2];2]>)->Vec<Vec<[[usize;2];2]>>{
	//let (nx,ny)=get_nx_ny(&v);
	let (nx,ny)=get_dim_by_edges(&e);
	let mut matrix:Vec<Vec<[[usize;2];2]>>=vec![vec![];nx];
	for i in 0..e.len(){
		let col:usize=e[i][0][0];
		matrix[col].push(e[i]);
	}
	
	println!("EDGES_MATRIX!");
	for i in 0..matrix.len(){
		println!("i:{}, {:?}",i,matrix[i]);
	}
	
	matrix
}
// get edges for difference 2 in bp
pub fn get_diff2_edge_matrix(edm:&Vec<Vec<[[usize;2];2]>>,diff_mod:usize)->Vec<Vec<[[usize;2];2]>>{
	let mut edm2:Vec<Vec<[[usize;2];2]>>=edm.clone();
	for i in 0..edm2.len(){
		if i%2==diff_mod{
			edm2[i]=vec![];
		}
	}
	edm2
}
pub fn edm_2_edgevec(edm:&Vec<Vec<[[usize;2];2]>>)->Vec<[[usize;2];2]>{
	let mut vec:Vec<[[usize;2];2]>=vec![];
	for i in 0..edm.len(){
		for j in 0..edm[i].len(){
			vec.push(edm[i][j]);
		}
	}
	vec
}

fn print_edge_matrix(edgematrix:&Vec<Vec<[[usize;2];2]>>){
	println!("PRINT EDGE MATRIX");
	for i in 0..edgematrix.len(){
		println!("i:{}",i);
		for j in 0..edgematrix[i].len(){
			println!("j:{}, {:?}",j,edgematrix[i][j][0]);
		}
	}
	
}

pub fn remove_edges_notin_edge_matrix(e_:&Vec<[[usize;2];2]>,edm:&Vec<Vec<[[usize;2];2]>>)->Vec<[[usize;2];2]>{
	let mut e:Vec<[[usize;2];2]>=e_.clone();
	let mut delvec:Vec<usize>=vec![];
	for i in 0..e.len(){
		let first:[usize;2]=e[i][0].clone();
		let first_i:usize=e[i][0][0];
		//if !edm[first_i].contains(&first){
		if !edm[first_i].contains(&e[i]){
			delvec.insert(0,i);
		}
	}
	for i in 0..delvec.len(){
		e.remove(delvec[i]);
	}
	e
}
pub fn remove_vertices_notin_edges(v_:&Vec<[usize;2]>,e:&Vec<[[usize;2];2]>)->Vec<[usize;2]>{
	let mut v:Vec<[usize;2]>=v_.clone();
	let mut delvec:Vec<usize>=vec![];
	for i in 0..v.len(){
		let mut is_there:bool=false;
		for j in 0..e.len(){
			let first:[usize;2]=e[i][0];
			let second:[usize;2]=e[i][1];
			if v[i]==first || v[i]==second{
				is_there=true;
			}
		}
		if !is_there{
			delvec.insert(0,i);
		}
	}
	for i in 0..delvec.len(){
		v.remove(delvec[i]);
	}
	v
}
// assumption: edm is consistent, i.e. every vertex has input and output edges! here: no check for consistency in edm! otherwise we have chain reaction of errors!
pub fn remove_vertices_notin_edge_matrix(v_:&Vec<[usize;2]>,edm:&Vec<Vec<[[usize;2];2]>>,donotremove:usize)->Vec<[usize;2]>{
	println!("FN REMOVE VERTICES NOT IN EDM!\nINPUT V: {:?}\nEDM: {:?}",v_,edm);
	let mut v:Vec<[usize;2]>=v_.clone();
	let mut delvec:Vec<usize>=vec![];
	for i in 0..v.len(){
		let mut is_there:bool=false;
		for j in 0..edm.len(){
			for k in 0..edm[j].len(){
				let first:[usize;2]=edm[j][k][0];
				let second:[usize;2]=edm[j][k][1];
				if (v[i]==first) || (v[i]==second){
					is_there=true;
				}
			}
		}
		if !is_there{
			if v[i][0]<donotremove{
				delvec.insert(0,i);
			}
		}
	}
	for i in 0..delvec.len(){
		v.remove(delvec[i]);
	}
	println!("FN REMOVE VERTICES NOT IN EDM!\nOUTPUT V: {:?}",v);
	v
}

// assumption: edm is consistent, i.e. every vertex has input and output edges! here: no check for consistency in edm! otherwise we have chain reaction of errors!
pub fn remove_vertices_notin_edge_matrix2(v_:&Vec<[usize;2]>,edm:&Vec<Vec<[[usize;2];2]>>,donotremove:usize,colmode:&[usize;2])->Vec<[usize;2]>{
	println!("FN REMOVE VERTICES NOT IN EDM!\nINPUT V: {:?}\nEDM: {:?}",v_,edm);
	let mut v:Vec<[usize;2]>=v_.clone();
	let mut delvec:Vec<usize>=vec![];
	for i in 0..v.len(){
		let mut is_there:bool=false;
		for j in 0..edm.len(){
			for k in 0..edm[j].len(){
				let first:[usize;2]=edm[j][k][0];
				let second:[usize;2]=edm[j][k][1];
				if (v[i]==first) || (v[i]==second){
					is_there=true;
				}
			}
		}
		if !is_there{
			if v[i][0]<donotremove{
				if v[i][0]%colmode[0]==colmode[1]{
					delvec.insert(0,i);
				}
			}
		}
	}
	for i in 0..delvec.len(){
		v.remove(delvec[i]);
	}
	println!("FN REMOVE VERTICES NOT IN EDM!\nOUTPUT V: {:?}",v);
	v
}
pub fn clean_edge_matrix(edm_:&Vec<Vec<[[usize;2];2]>>,v:&Vec<[usize;2]>)->Vec<Vec<[[usize;2];2]>>{
	let mut edm:Vec<Vec<[[usize;2];2]>>=edm_.clone();
	println!("FN CLEAN EDM! INPUT\nEDM_IN:{:?}\nVERTICES:{:?}",edm,v);
	for i in 0..edm.len(){
		let mut delvec:Vec<usize>=vec![];
		for j in 0..edm[i].len(){
			let first:[usize;2]=edm[i][j][0];
			let second:[usize;2]=edm[i][j][1];
			/*
			if (!v.contains(&first)) && (!v.contains(&second)){
				delvec.insert(0,j);
			}
			else{
				if !v.contains(&first){
					delvec.insert(0,j);
				}
				if !v.contains(&second){
					delvec.insert(0,j);
				
				}
			}
			*/
			if !v.contains(&first){
				delvec.insert(0,j);
			}
			if !v.contains(&second){
				if !delvec.contains(&j){
					delvec.insert(0,j);
				}
			}
		}
		for j in 0..delvec.len(){
			edm[i].remove(delvec[j]);
		}
	}
	println!("FN CLEAN EDM OUTPUT:{:?}",edm);
	edm
}
fn sort_edges(e:&Vec<[[usize;2];2]>)->(Vec<[[usize;2];2]>,Vec<[[usize;2];2]>){
	let mut edges:Vec<[[usize;2];2]>=e.clone();
	let mut edges_reddotted:Vec<[[usize;2];2]>=vec![];
	let mut delvec:Vec<[[usize;2];2]>=vec![];
	let mut delete_for_dashed:Vec<[[usize;2];2]>=vec![];
	for i in 0..e.len(){
		let first:[usize;2]=e[i][0];
		let second:[usize;2]=e[i][1];
		let mut input:bool=false;
		for j in 0..i{
			if e[j][1]==first{
				input=true;
				break;
			}
		}
		if !input{
			// delete from edges! and chain reaction! never allow to propose!
			//delvec.push(first);
			if i<e.len()-1{
				for j in i+1..e.len(){
					if e[j][0]==first{
						if !delvec.contains(&e[j]){
							delvec.insert(0,e[j]);
						}
					}
				}
			}
		}
		if i<e.len()-1{
			let mut output:bool=false;
			for j in i+1..e.len(){
				if e[j][0]==second{
					output=true;
					break;
				}
			}
			if !output{
				//delete_for_dashed.push(e[i]);
				delete_for_dashed.insert(0,e[i]);
			}
		}
	}
	for i in 0..delete_for_dashed.len(){
		let pos:usize=edges.iter().position(|x| *x==delete_for_dashed[i]).unwrap();
		edges.remove(pos);
	}
	for i in 0..delvec.len(){
		if edges.contains(&delvec[i]){
			println!("i:{}, \ndelvec:{:?}, \nedges:{:?}\ndelvec[{}]:{:?}",i,delvec,edges,i,delvec[i]);
			let pos:usize=edges.iter().position(|x| *x==delvec[i]).unwrap();
			edges.remove(pos);
		}
	}
	edges_reddotted=delete_for_dashed.clone();
	(edges,edges_reddotted)
}
//fn sort_edges2(e:&Vec<[[usize;2];2]>)->(Vec<[[usize;2];2]>,Vec<[[usize;2];2]>){
fn sort_edges2(e:&Vec<[[usize;2];2]>)->(Vec<Vec<[[usize;2];2]>>,Vec<Vec<[[usize;2];2]>>){
	let (nx,ny)=get_dim_by_edges(&e);
	//let mut edm_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&e);
	let mut edm:Vec<Vec<[[usize;2];2]>>=edges_matrix(&e);
	let start:usize=1;
	let end:usize=5;
	//let mut edm:Vec<Vec<[[usize;2];2]>>=get_diff2_edge_matrix(&edm_orig,0);
	let mut edges:Vec<[[usize;2];2]>=e.clone();
	let mut edges_reddotted:Vec<[[usize;2];2]>=vec![];
	let mut delvec:Vec<[[usize;2];2]>=vec![];
	let mut delmat:Vec<Vec<usize>>=vec![vec![];nx];
	let mut delete_for_dashed:Vec<[[usize;2];2]>=vec![];
	let mut delete_for_dashed2:Vec<Vec<[[usize;2];2]>>=vec![vec![];nx];
	//let mut delete_for_dashed2:Vec<Vec<usize>>=vec![vec![];nx];
	for i in 0..edm.len(){
		for j in 0..edm[i].len(){
			let first:[usize;2]=edm[i][j][0];
			let second:[usize;2]=edm[i][j][1];
			//if i>0{
			if i>start{
				let mut input:bool=false;
				for k in 0..i{
					for l in 0..edm[k].len(){
						if edm[k][l][1]==first{
							input=true;
							break;
						}
					}
				}
				if !input{
					// delete from edges! and chain reaction! never allow to propose!
					//delvec.push(first);
					//println!("NO INPUT! i:{}, j:{}, edm[{}][{}]:{:?}",i,j,i,j,edm[i][j]);
					delmat[i].insert(0,j);
				}
			}
			//if i<edm.len()-1{
			if i<end-2{
				let mut output:bool=false;
				for k in i+1..edm.len(){
					for l in 0..edm[k].len(){
						if edm[k][l][0]==second{
							output=true;
							break;
						}
					}
				}
				if !output{
					//delete_for_dashed.push(e[i]);
					println!("NO OUTPUT! i:{}, j:{}, edm[{}][{}]:{:?}",i,j,i,j,edm[i][j]);
					delete_for_dashed2[i].push(edm[i][j]);
					
				}
			}
		}
		
		for j in 0..delmat[i].len(){
			edm[i].remove(delmat[i][j]);
		}
	}
	for i in 0..delete_for_dashed2.len(){
		if delete_for_dashed2[i].len()>0{
			for j in 0..delete_for_dashed2[i].len(){
				let pos:Option<usize>=edm[i].iter().position(|x| *x==delete_for_dashed2[i][j]);
				if !pos.is_none(){
					edm[i].remove(pos.unwrap());
				}
			}
		}
	}
	//edges_reddotted=delete_for_dashed.clone();
	//(edges,edges_reddotted)
	(edm,delete_for_dashed2)
}
//fn sort_edges3(e:&Vec<[[usize;2];2]>,start:usize,end:usize)->(Vec<Vec<[[usize;2];2]>>,Vec<Vec<[[usize;2];2]>>){
fn sort_edges3(edm_:&Vec<Vec<[[usize;2];2]>>,start:usize,end:usize)->(Vec<Vec<[[usize;2];2]>>,Vec<Vec<[[usize;2];2]>>){
	//let (nx,ny)=get_dim_by_edges(&e);
	let ny:usize=edm_[0].len();
	let nx:usize=edm_.len();
	//let mut edm_orig:Vec<Vec<[[usize;2];2]>>=edges_matrix(&e);
	//let mut edm:Vec<Vec<[[usize;2];2]>>=edges_matrix(&e);
	let mut edm:Vec<Vec<[[usize;2];2]>>=edm_.clone();
	//let start:usize=1;
	//let end:usize=5;
	//let mut edm:Vec<Vec<[[usize;2];2]>>=get_diff2_edge_matrix(&edm_orig,0);
	//let mut edges:Vec<[[usize;2];2]>=e.clone();
	let mut edges_reddotted:Vec<[[usize;2];2]>=vec![];
	let mut delvec:Vec<[[usize;2];2]>=vec![];
	let mut delmat:Vec<Vec<usize>>=vec![vec![];nx];
	let mut delete_for_dashed:Vec<[[usize;2];2]>=vec![];
	let mut delete_for_dashed2:Vec<Vec<[[usize;2];2]>>=vec![vec![];nx];
	for i in 0..edm.len(){
		for j in 0..edm[i].len(){
			let first:[usize;2]=edm[i][j][0];
			let second:[usize;2]=edm[i][j][1];
			//if i>0{
			if i>start{
				let mut input:bool=false;
				for k in 0..i{
					for l in 0..edm[k].len(){
						if edm[k][l][1]==first{
							input=true;
							break;
						}
					}
				}
				if !input{
					// delete from edges! and chain reaction! never allow to propose!
					//println!("NO INPUT! i:{}, j:{}, edm[{}][{}]:{:?}",i,j,i,j,edm[i][j]);
					delmat[i].insert(0,j);
				}
			}
			//if i<edm.len()-1{
			if i<end{
				let mut output:bool=false;
				for k in i+1..edm.len(){
					for l in 0..edm[k].len(){
						if edm[k][l][0]==second{
							output=true;
							break;
						}
					}
				}
				if !output{
					//delete_for_dashed.push(e[i]);
					println!("NO OUTPUT! i:{}, j:{}, edm[{}][{}]:{:?}",i,j,i,j,edm[i][j]);
					println!("dashed2.len:{}, edm.len:{}, edm[{}].len:{}",delete_for_dashed2.len(),edm.len(),i,edm[i].len());
					delete_for_dashed2[i].push(edm[i][j]);					
				}
			}
		}
		
		for j in 0..delmat[i].len(){
			edm[i].remove(delmat[i][j]);
		}
	}
	for i in 0..delete_for_dashed2.len(){
		if delete_for_dashed2[i].len()>0{
			for j in 0..delete_for_dashed2[i].len(){
				let pos:Option<usize>=edm[i].iter().position(|x| *x==delete_for_dashed2[i][j]);
				if !pos.is_none(){
					edm[i].remove(pos.unwrap());
				}
			}
		}
	}
	(edm,delete_for_dashed2)
}
pub fn print_edges(e:&Vec<[[usize;2];2]>){
	println!("PRINT EDGES");
	for i in 0..e.len(){
		println!("{:?}",e[i]);
	}
}

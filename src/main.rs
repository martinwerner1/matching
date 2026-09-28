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
use display::vdm_edges;
use display::manyone;

fn main(){

	// N-SIDED MATCHING BASED ON INTERSECTION METHOD, RANDOM PREFERENCES
	N_sided_matching::test_deepsearch_n_sided();


	
	// ALL STABLE MATCHES IN MANY-TO-ONE MATCHING (GUSFIELD/IRVING EXAMPLE)
	//manyone::test_manyone_enumeration();

	// DA algorithm for many-to-many matching. No enumeration of stable outcomes is possible if the acceptor's side 
	// has quotas (this is a well-known fact that simple DA enumeration fails in many-to-many matching; see Bansal, Eirinakis etc.). 
	// However it can be still used for many-to-one as well as one-to-one matching (for enumeration) or for the obtaining the proposer's best match results.
	//manyone::test_gusfield_manymany();
	
	// LINEAR PROGRAMMING
	//test_lp();
	
	// VERTEX DELETION MECHANISM FOR REMOVING UNSTABLE PAIRS
	//node_deletion::tests();
	
	// CREATE LATEX CODE FOR STABILITY MATRIX (PACKAGE:PSTRICKS), UNCOMMENT BOTH LINES FOR USAGE
	//let bp:Vec<Vec<bool>>=IOClass::get_bp_automatically();
	//tex::psgrid_test(&bp,false); // second parameter -> false: full stability matrix, true: upper half of stability matrix (needed for bitsync)
	
	// GS ALGORITHM WITH GS REDUCED LISTS
	//gale_shapley::test();
	
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




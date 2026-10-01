#![allow(warnings)]
use super::helper::{IOClass};
use super::node_deletion;



pub fn psgrid_test(bp:&Vec<Vec<bool>>,half:bool){
	ps_grid_from_bp(&bp,half);
}


fn ps_grid_from_bp(bp:&Vec<Vec<bool>>,half:bool){
	let mut txt:&str="\\begin{pspicture}\n";
	let mut bp_half=bp.clone();
	if half{
		bp_half=IOClass::bp_matrix_half(&bp);
	}
	//println!("BP MATRIX HALF!");
	for i in 0..bp_half.len(){
		//println!("{:?}",bp_half[i]);
	}
	//let mut psgrid:PSGrid=PSGrid::init(&bp,5.8);
	let mut psgrid:PSGrid=PSGrid::init(&bp_half,7.0);
	//psgrid.put_shift(7.0,0.0);
	psgrid.get_grid3();
	/*
	println!("PSGRID FOR WOMEN!");
	let bpw=node_deletion::bp_matrix_for_women(&bp);
	let mut psgridw:PSGrid=PSGrid::init(&bpw,5.8);
	psgridw.put_gender(1);
	psgridw.put_shift(7.0,0.0);
	psgridw.get_grid();
	*/
	//println!("OPACITY!");
	
	// COMMENTED TO HOLD EVERYTHING AS COMPACT AS POSSIBLE!
	/*
	psgrid.get_opacity_line_oc(7);
	psgrid.get_opacity_col_oc(7);
	psgrid.fill_cell(7,7,"green".to_string());
	psgrid.fill_cell(7,14,"green".to_string());
	psgrid.fill_cell(7,0,"white".to_string());

	psgrid.fill_cell(7,15,"green".to_string());
	psgrid.fill_cell(7,18,"green".to_string());
	psgrid.fill_cell(7,20,"green".to_string());
	psgrid.fill_cell(14,7,"green".to_string());
	psgrid.fill_cell(15,7,"green".to_string());
	psgrid.fill_cell(18,7,"green".to_string());
	psgrid.fill_cell(20,7,"green".to_string());
	*/
	
	//println!("{}",txt);
}
pub struct PSGrid{
	bp:Vec<Vec<bool>>,
	empty_vertices:Vec<([usize;2],usize)>,
	unit:f64,
	linewidth:f64,
	xshift:f64,
	yshift:f64,
	n:f64,
	size:f64,
	gender:usize,
}
impl PSGrid{
	pub fn init(bp_:&Vec<Vec<bool>>,size_:f64)->Self{
		let n_:f64=bp_.len().isqrt() as f64;
		let unit_:f64=size_ as f64/n_;
		let linewidth_:f64=unit_/(2.0*n_);
		let empty_vertices_=node_deletion::node_deletion_empty_vertices(&bp_);
		Self{
			bp:bp_.clone(),
			empty_vertices:empty_vertices_,
			unit:unit_,
			linewidth:linewidth_,
			xshift:0.0,
			yshift:0.0,
			n:bp_.len().isqrt() as f64,
			size:size_,
			gender:0,
		}
	}
	pub fn put_shift(&mut self,x:f64,y:f64){
		self.xshift=x;
		self.yshift=y;
	}
	pub fn put_gender(&mut self,gender_:usize){
		self.gender=gender_;
	}
//	\psgrid[unit=0.5cm,subgriddiv=10,linewidth=0.01pt,subgridwidth=0.01pt](2,0)(12,10)

//	\psframe[linewidth=0.1cm,dimen=outer](1.2,1.4)(1.4,1.6)
	
	pub fn get_grid(&self){
		let ev=self.empty_vertices.clone();
		let mut txt:String=format!("\\begin{{pspicture}}({},{})\n",self.size,self.size).to_string();
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})({},{})\n",
		//	self.unit,self.n,self.xshift as f64/self.unit,self.yshift as f64/self.unit,(self.xshift+self.size) as f64/self.unit,(self.yshift+self.size) as f64/self.unit);
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})\n",self.size as f64/self.n as f64,self.n,self.n,self.n);
		// COMMENTED!
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})({},{})\n",self.size as f64/self.n,self.n,self.xshift/self.unit,self.yshift/self.unit,self.xshift/self.unit+self.n,self.yshift/self.unit+self.n);
		txt+="\\definecolor{grey}{rgb}{0.7,0.7,0.7}";

		let pattern:Vec<String>=vec!["m".to_string(),"w".to_string()];
		
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;
		for i in 0..self.bp.len(){
			for j in 0..self.bp[i].len(){
				if self.bp[i][j]{
					//txt+=&format!("\\psframe[linewidth={}cm]({},{})({},{})\n",self.linewidth,i as f64*diff,(n*n-j) as f64*diff,(i+1) as f64*diff,(n*n-j-1) as f64*diff);
					txt+=&format!("\\psframe[linewidth={}cm,linecolor=grey]({},{})({},{})\n",self.linewidth,i as f64*diff,(n*n-j) as f64*diff,(i+1) as f64*diff,(n*n-j-1) as f64*diff);
				}
			}
		}

		txt+=&format!("\\psgrid[gridcolor=black,gridwidth=2.5pt,unit={}cm,subgriddiv={}]({},{})({},{})\n",self.size as f64/self.n,self.n,0,0,self.n,self.n);

		for i in 0..ev.len(){
			let (data,col)=ev[i];
			let line:usize=data[0]*n+data[1];
			txt+=&format!("\\psframe[linecolor=red,linewidth={}cm,dimen=inner]({},{})({},{})\n",self.linewidth/n as f64,(col*n) as f64*diff,(n*n-line) as f64*diff,((col+1)*n) as f64*diff,(n*n-line-1) as f64*diff);
			
		}
		// horizontal
		for i in 0..n{
			txt+=&format!("\\rput({},{}){{${}_{}$}}",i as f64*self.unit+0.5*self.unit-(self.unit*n as f64),(n as f64+0.5)*self.unit,pattern[self.gender],i+1);
		}
		txt+="\n";
		// vertical
		for i in 0..n{
			//txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit-(n as f64*self.unit),pattern[self.gender],i+1);
			txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit,pattern[self.gender],i+1);
			
		}
		txt+="\n";
		
		txt+="\\end{pspicture}";
		println!("{}",txt);
	}
	pub fn get_grid2(&self){
		//let ev=self.empty_vertices.clone();
		let mut txt:String=format!("\\begin{{pspicture}}({},{})\n",self.size,self.size).to_string();
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})({},{})\n",
		//	self.unit,self.n,self.xshift as f64/self.unit,self.yshift as f64/self.unit,(self.xshift+self.size) as f64/self.unit,(self.yshift+self.size) as f64/self.unit);
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})\n",self.size as f64/self.n as f64,self.n,self.n,self.n);
		// COMMENTED!
		//txt+=&format!("\\psgrid[gridcolor=yellow,unit={}cm,subgriddiv={}]({},{})({},{})\n",self.size as f64/self.n,self.n,self.xshift/self.unit,self.yshift/self.unit,self.xshift/self.unit+self.n,self.yshift/self.unit+self.n);
		txt+="\\definecolor{grey}{rgb}{0.7,0.7,0.7}";

		let pattern:Vec<String>=vec!["m".to_string(),"w".to_string()];
		
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;
		for i in 0..self.bp.len(){
			for j in 0..self.bp[i].len(){
				if self.bp[i][j]{
					//txt+=&format!("\\psframe[linewidth={}cm]({},{})({},{})\n",self.linewidth,i as f64*diff,(n*n-j) as f64*diff,(i+1) as f64*diff,(n*n-j-1) as f64*diff);
					//txt+=&format!("\\psframe[linewidth={}cm,linecolor=grey]({},{})({},{})\n",self.linewidth,i as f64*diff,(n*n-j) as f64*diff,(i+1) as f64*diff,(n*n-j-1) as f64*diff);
					self.fill_cell(j,i,"grey".to_string());
				}
			}
		}

		txt+=&format!("\\psgrid[gridcolor=black,gridwidth=2.5pt,unit={}cm,subgriddiv={}]({},{})({},{})\n",self.size as f64/self.n,self.n,0,0,self.n,self.n);
		/*
		for i in 0..ev.len(){
			let (data,col)=ev[i];
			let line:usize=data[0]*n+data[1];
			txt+=&format!("\\psframe[linecolor=red,linewidth={}cm,dimen=inner]({},{})({},{})\n",self.linewidth/n as f64,(col*n) as f64*diff,(n*n-line) as f64*diff,((col+1)*n) as f64*diff,(n*n-line-1) as f64*diff);
			
		}
		*/
		// horizontal
		for i in 0..n{
			txt+=&format!("\\rput({},{}){{${}_{}$}}",i as f64*self.unit+0.5*self.unit-(self.unit*n as f64),(n as f64+0.5)*self.unit,pattern[self.gender],i+1);
		}
		txt+="\n";
		// vertical
		for i in 0..n{
			//txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit-(n as f64*self.unit),pattern[self.gender],i+1);
			txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit,pattern[self.gender],i+1);
			
		}
		txt+="\n";
		
		txt+="\\end{pspicture}";
		println!("{}",txt);
	}

	pub fn get_grid3(&self){
		//let ev=self.empty_vertices.clone();
		let mut txt:String=format!("\\documentclass{{article}}\n\\usepackage{{pstricks,pstricks-add,pst-pdf}}\n\\begin{{document}}\n\\begin{{pspicture}}({},{})\n",self.size,self.size).to_string();
		// COMMENTED!
		txt+="\\definecolor{grey}{rgb}{0.7,0.7,0.7}";

		let pattern:Vec<String>=vec!["m".to_string(),"w".to_string()];
		
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;
		for i in 0..self.bp.len(){
			for j in 0..self.bp[i].len(){
				if self.bp[i][j]{
					txt+=&self.fill_cell(j,i,"grey".to_string());
				}
			}
		}

		txt+=&format!("\\psgrid[gridcolor=black,gridwidth=2.5pt,unit={}cm,subgriddiv={}]({},{})({},{})\n",self.size as f64/self.n,self.n,0,0,self.n,self.n);
		/*
		for i in 0..ev.len(){
			let (data,col)=ev[i];
			let line:usize=data[0]*n+data[1];
			txt+=&format!("\\psframe[linecolor=red,linewidth={}cm,dimen=inner]({},{})({},{})\n",self.linewidth/n as f64,(col*n) as f64*diff,(n*n-line) as f64*diff,((col+1)*n) as f64*diff,(n*n-line-1) as f64*diff);
			
		}
		*/
		
		/*
		// COMMENTED FOR HOLDING A STABILITY MATRIX WITHOUT ANY ADDITIONAL MARKERS
		// horizontal
		for i in 0..n{
			txt+=&format!("\\rput({},{}){{${}_{}$}}",i as f64*self.unit+0.5*self.unit-(self.unit*n as f64),(n as f64+0.5)*self.unit,pattern[self.gender],i+1);
		}
		txt+="\n";
		// vertical
		for i in 0..n{
			//txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit-(n as f64*self.unit),pattern[self.gender],i+1);
			txt+=&format!("\\rput({},{}){{${}_{}$}}",-0.5*self.unit-(self.unit*n as f64),(n as f64-(i as f64+0.5))*self.unit,pattern[self.gender],i+1);
			
		}
		txt+="\n";
		*/
		
		txt+="\\end{pspicture}\n\\end{document}\n";
		println!("{}",txt);
	}

	pub fn fill_cell(&self,i:usize,j:usize,color:String)->String{
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;
		let mut txt:String=format!("\\psframe*[linecolor={}]({},{})({},{})\n",color,i as f64*diff,(n*n-j) as f64*diff,(i+1) as f64*diff,(n*n-j-1) as f64*diff);
		//println!("{}",txt);
		txt
	}
	pub fn get_opacity_line(&self,line:usize){
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;
		let unit:f64=self.unit;
		let mut txt:String="\\definecolor{brightgreen}{rgb}{0.7,0.9,0.7}\n".to_string();
		txt+="\\definecolor{stronggreen}{rgb}{0.1,0.9,0.1}\n";
		//for j in 0..n*n{
		println!("diff:{}, line:{}, unit:{}",diff,line,unit);
		for j in 0..self.bp.len(){
			if self.bp[line][j]{
				txt+=&format!("\\psframe[linewidth={}cm,linecolor=stronggreen]({},{})({},{})\n",self.linewidth,line as f64*diff,(n*n-j) as f64*diff,(line+1) as f64*diff,(n*n-j-1) as f64*diff);				
			}
			else{
				txt+=&format!("\\psframe[linewidth={}cm,linecolor=brightgreen]({},{})({},{})\n",self.linewidth,line as f64*diff,(n*n-j) as f64*diff,(line+1) as f64*diff,(n*n-j-1) as f64*diff);				
			}
		}
		println!("{}",txt);
	}
	pub fn get_opacity_col(&self,col:usize){
		let diff:f64=self.unit/self.n;
		let n:usize=self.n as usize;

		let mut txt:String="\\definecolor{brightgreen}{rgb}{0.7,0.9,0.7}\n".to_string();
		txt+="\\definecolor{stronggreen}{rgb}{0.49,0.9,0.49}\n";
		for i in 0..n*n{
			if self.bp[i][col]{
				txt+=&format!("\\psframe*[linecolor=stronggreen]({},{})({},{})\n",i as f64*diff,(n*n-col) as f64*diff,(i+1) as f64*diff,(n*n-col-1) as f64*diff);
			}
			else{
				txt+=&format!("\\psframe*[linecolor=brightgreen]({},{})({},{})\n",i as f64*diff,(n*n-col) as f64*diff,(i+1) as f64*diff,(n*n-col-1) as f64*diff);
				
			}
		}
		println!("{}",txt);
	}
	pub fn get_opacity_line_oc(&self,line:usize){
		let n:usize=self.n as usize;
		let diff:f64=self.unit/self.n;
		
		let mut txt:String="\\definecolor{brightgreen}{rgb}{0.4,0.9,0.4}\n".to_string();

		txt+=&format!("\\psframe*[linecolor=brightgreen,opacity=0.3]({},{})({},{})\n",0.0,(n*n-line) as f64*diff,(n*n) as f64*diff,((n*n) as f64-line as f64-1.0) as f64*diff);
		
		println!("{}",txt);
	}
	pub fn get_opacity_col_oc(&self,line:usize){
		let n:usize=self.n as usize;
		let diff:f64=self.unit/self.n;
		let mut txt:String="\\definecolor{brightgreen}{rgb}{0.4,0.9,0.4}\n".to_string();
		txt+=&format!("\\psframe*[linecolor=brightgreen,opacity=0.3]({},{})({},{})\n",line as f64*diff,0.0,(line as f64+1.0)*diff,(n*n) as f64*diff);
		
		println!("{}",txt);
	}
	
}
pub struct Pref{
	m:Vec<Vec<usize>>,
	w:Vec<Vec<usize>>,
}
impl Pref{
	pub fn new()->Self{
		let m_:Vec<Vec<usize>>=IOClass::read_txt("pref/m.txt".to_string());
		let w_:Vec<Vec<usize>>=IOClass::read_txt("pref/w.txt".to_string());
		Self{
			m:m_,
			w:w_,
		}
	}
	pub fn get_pref(&self)->String{
		let mut txt:String=String::new();
		txt+="\\begin{center}\n\\begin{table}\n\\begin{tabular}{|c|}\\hline\n";
		assert!(self.m.len()==self.w.len());
		for i in 0..self.m.len(){
			txt+=&format!("$m_{{{}}}$: ",i+1);
			for j in 0..self.m[i].len(){
				txt+=&format!("$w_{{{}}}$ ",self.m[i][j]+1);
				if j<self.m[i].len()-1{
					txt+="$\\succ$ ";
				}
			}
			txt+=&format!("\\qquad\\qquad$w_{{{}}}$: ",i+1);
			for j in 0..self.w[i].len(){
				txt+=&format!("$m_{{{}}}$ ",self.w[i][j]+1);
				if j<self.w[i].len()-1{
					txt+="$\\succ$ ";
				}
				else{
					txt+="\\\\ \n";
				}
			}
		}
		txt+="\\hline\n";
		txt+="\\end{tabular}\n\\end{table}\n\\end{center}\n";
		txt
	}
}

pub struct RankMatrix{
	adj:Vec<Vec<[usize;2]>>,
}
impl RankMatrix{
	pub fn new(adj_:&Vec<Vec<[usize;2]>>)->Self{
		Self{
			adj:adj_.to_vec(),
		}
	}
	pub fn get_table_rank(&self)->String{
		let mut txt:String=String::new();
		txt+="\\begin{center}\n\\begin{table}\n";
		txt+="\\centering\n\\begin{tabular}";
		if self.adj.len()>0{
			txt+="{|l ";
			for j in 0..self.adj[0].len(){
				txt+="|c ";
			}
			txt+="|}\\hline\n&";
			for j in 0..self.adj.len(){
				txt+=&format!("$w_{{{}}}$",j+1);
				if j<self.adj[0].len()-1{
					txt+=" & ";
				}
				else{
					txt+="\\\\ \\hline\n";
				}
			}
			for i in 0..self.adj.len(){
				txt+=&format!("$m_{{{}}}$ &",i+1);
				for j in 0..self.adj[i].len(){
					txt+=&format!("$({},{})$",self.adj[i][j][0]+1,self.adj[i][j][1]+1);
					if j<self.adj[i].len()-1{
						txt+=" & ";
					}
					else{
						txt+="\\\\ \\hline\n";
					}
				}
			}
		}
		txt+="\\end{tabular}\n";
		txt+=&format!("\\caption{{Rank matrix $\\xi$ at $n={}$}}",self.adj.len());
		txt+="\\end{table}\n\\end{center}";
		txt		
	}
}


pub struct Gamma{
	bp:Vec<Vec<bool>>,
	bp_num:Vec<Vec<usize>>,
}
impl Gamma{
	pub fn new(bp_:&Vec<Vec<bool>>)->Self{
		let n:usize=bp_.len().isqrt();
		let bp_num_=IOClass::bp_matrix_2usize_block2(&bp_,n);
		Self{
			bp:bp_.to_vec(),
			bp_num:bp_num_,
		}
	}
	pub fn get_table(&self)->String{
		let mut txt:String=String::new();
		let n:usize=self.bp[0].len();
		let n_:usize=self.bp.len().isqrt();
		txt+="\\begin{table}\n\\centering\n\\begin{tabular}";
		txt+="{| l |";
		for i in 0..n{
			if i%n_==0{
				txt+="| ";
			}
			txt+="r ";
		}		
		txt+="| } \\hline \n";
		txt+="Row $i$ & ";		
		for j in 0..n_{
			//txt+=&format!("$m_{{{}}}$ ",j+1);
			txt+=&format!("\\multicolumn{{{}}}{{c|}}{{$m_{{{}}}$}} ",n_,j+1);
			if j<n_-1{
				txt+="& ";
			}
			else{
				txt+="\\\\ \n";
			}
		}
		for i in 0..self.bp.len(){
			if i%n_==0{
				txt+="\\hline";
			}
			txt+=&i.to_string();
			txt+="& ";
			for j in 0..n{
				if self.bp[i][j]{
					txt+=&1.to_string();					
				}
				else{
					txt+=&0.to_string();
				}
				//txt+=&self.bp[i][j].to_string();
				//if j<n-1{
				if j<self.bp[i].len()-1{
					txt+="& ";
				}
				else{
					txt+="\\\\";					
					txt+="\n";
				}
			}
		}
		txt+="\\hline\n";
		txt+="\\end{tabular}\n";
		txt+=&format!("\\caption{{Stability matrix $\\Gamma$ for n={}}}\n\\end{{table}}",n_);
		txt
	}

	pub fn get_table_vertices(&self)->String{
		let mut txt:String=String::new();
		let n:usize=self.bp[0].len();
		let n_:usize=self.bp.len().isqrt();
		txt+="\\begin{table}\n\\centering\n\\resizebox{\\columnwidth}{!}{\\begin{tabular}";
		txt+="{| l | c |";
		for i in 0..n{
			if i%n_==0{
				txt+="| ";
			}
			txt+="r ";
		}		
		txt+="| } \\hline \n";
		txt+="& & ";		
		for j in 0..n_{
			//txt+=&format!("$m_{{{}}}$ ",j+1);
			txt+=&format!("\\multicolumn{{{}}}{{c|}}{{$m_{{{}}}$}} ",n_,j+1);
			if j<n_-1{
				txt+="& ";
			}
			else{
				txt+="\\\\ \\hline \n";
			}
		}
		for row in 0..3{
			for ampersand in 0..n+1{
				txt+="& ";
			}
			txt+="\\\\ \n";
		}
		txt+="$k$ & $v$ & \n";		
		for i in 0..n{
			let m:usize=i/n_;
			let w:usize=i%n_;
			txt+="\\begin{rotate}{90}\n";
			txt+=&format!("$(m_{{{}}},w_{{{}}})$\n\\end{{rotate}}\n",m+1,w+1);
			if i<n-1{
				txt+="&\n";
			}
			else{
				txt+="\\\\ \n";
			}
		}
		for i in 0..self.bp.len(){
			if i%n_==0{
				txt+="\\hline";
			}
			txt+=&i.to_string();
			txt+="& ";
			let m:usize=i/n_;
			let w:usize=i%n_;
			txt+=&format!("$(m_{{{}}},w_{{{}}})$",m+1,w+1);
			txt+="& ";
			for j in 0..n{
				if self.bp[i][j]{
					txt+=&1.to_string();					
				}
				else{
					txt+=&0.to_string();
				}
				//txt+=&self.bp[i][j].to_string();
				//if j<n-1{
				if j<self.bp[i].len()-1{
					txt+="& ";
				}
				else{
					txt+="\\\\";					
					txt+="\n";
				}
			}
		}
		txt+="\\hline\n";
		txt+="\\end{tabular}\n}\n";
		txt+=&format!("\\caption{{Stability matrix $\\Gamma$ for n={}}}\n\\end{{table}}",n_);
		txt
	}
	pub fn get_table_num(&self)->String{
		let mut txt:String=String::new();
		let n:usize=self.bp_num[0].len();
		txt+="\\begin{table}\n\\centering\n\\begin{tabular}";
		txt+="{l ";
		for i in 0..n{
			txt+="r ";
		}
		txt+="}\n";
		txt+="row & ";
		for j in 0..n{
			txt+=&format!("$m_{{{}}}$ ",j+1);
			if j<n-1{
				txt+="& ";
			}
			else{
				txt+="\\\\ \n";
			}
		}
		for i in 0..self.bp_num.len(){
			txt+=&i.to_string();
			txt+="& ";
			for j in 0..n{
				txt+=&self.bp_num[i][j].to_string();
				if j<n-1{
					txt+="& ";
				}
				else{
					txt+="\\\\ \n";
				}
			}
		}
		txt+="\\end{tabular}\n\\end{table}";
		txt
	}
}

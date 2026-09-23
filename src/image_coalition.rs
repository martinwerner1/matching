#![allow(warnings)]

pub fn test(){
	let mut bx:boxes=boxes::new();
	bx.put_size(0.5,0.7);
	bx.put_shift(2.0,3.0);
	bx.put_contents(&vec![1,3,2,4]);
	
	let mut bx1:boxes=boxes::new();
	let mut bx2:boxes=boxes::new();
	let mut bx3:boxes=boxes::new();
	let mut bx4:boxes=boxes::new();
	bx1.put_size(0.5,0.7);
	bx2.put_size(0.5,0.7);
	bx3.put_size(0.5,0.7);
	bx4.put_size(0.5,0.7);
	bx1.put_contents(&vec![1,3,7,1]);
	bx2.put_contents(&vec![2,1,3,4]);
	bx3.put_contents(&vec![3,4,1,2]);
	//bx4.put_contents(&vec![4,2,0,3]);
	let mut bc:blocking_coalition=blocking_coalition::new();
	bc.add_box(bx1);
	bc.add_box(bx2);
	bc.add_box(bx3);
	//bc.add_box(bx4);
	bc.add_dist(1.0,3.0);
	let block_vec:Vec<[usize;2]>=vec![[0,0],[0,2],[1,1],[2,3]];
	bc.create_top_box(&block_vec);
	println!("TOP BOX: {:?}",bc.get_top_box());
	bc.draw_boxes();
	bc.edges_tex();
	//bx.get_tex();
}


pub struct blocking_coalition{
	xdist:f64,
	ydist:f64,
	top:boxes,
	bxs:Vec<boxes>,
	edges:Vec<[[usize;2];2]>,
	n:usize,
}
impl blocking_coalition{
	pub fn new()->Self{
		Self{
			xdist:0.0,
			ydist:0.0,
			top:boxes::new(),
			bxs:vec![],
			edges:vec![],
			n:0,
		}
	}
	pub fn add_box(&mut self,to_add:boxes){
		self.n=to_add.contents.len();
		self.bxs.push(to_add);
	}
	pub fn add_dist(&mut self,x:f64,y:f64){
		self.xdist=x;
		self.ydist=y;
	}
	pub fn create_top_box(&mut self,vec:&Vec<[usize;2]>){
		let n:usize=self.bxs[0].contents.len();
		let mut top_contents:Vec<usize>=vec![0;n];
		for i in 0..vec.len(){
			let a:usize=self.bxs[vec[i][0]].contents[vec[i][1]];
			top_contents[vec[i][1]]=a;
			self.edges.push([vec[i],[0,vec[i][1]]]);
		}
		self.top.contents=top_contents.clone();
		self.top.boxw=self.bxs[0].boxw;
		self.top.boxh=self.bxs[0].boxh;
	}
	
	
	
	
	pub fn get_top_box(&self)->Vec<usize>{
		return self.top.contents.clone();
	}
	
	pub fn edges(&mut self,vec:&Vec<[usize;2]>){
		
	}
	pub fn draw_boxes(&mut self){
		let xd:f64=self.xdist;
		let yd:f64=self.ydist;
		let bxw:f64=self.bxs[0].boxw;
		let n:usize=self.n;
		for i in 0..self.bxs.len(){
			let xsi:f64=i as f64*(n as f64 * bxw+xd);
			self.bxs[i].put_shift(xsi,0.0);
			self.bxs[i].get_tex();
		}
		let mut topxs:f64=0.0;
		let topys:f64=yd+self.bxs[0].boxh;
		let bxs_n:usize=self.bxs.len();
		if bxs_n%2==0{
			//println!("XSHIFT\nbxs_n/2:{}, bxw*n:{}, bxw_n*n+xd:{}, bxw+xd:{}",bxs_n/2,bxw*(n as f64),bxw*(n as f64)+xd,bxw+xd);
			//topxs=(bxs_n/2) as f64*(bxw*n as f64)+(((bxs_n/2)-1)as f64*xd)-0.5*(bxw+xd)+0.5*xd;
			topxs=(bxs_n/2) as f64*(bxw*n as f64)+(((bxs_n/2)-1)as f64*xd)-0.5*(bxw*n as f64)+0.5*xd;

		}
		else{
			topxs=(bxs_n/2) as f64*(bxw*n as f64+xd);
		}
		self.top.put_shift(topxs,topys);
		self.top.get_tex();
		
	}
	pub fn edges_tex(&self)->String{
		let y_up:f64=self.ydist+self.top.boxh;
		let xd:f64=self.xdist;
		let n:usize=self.n;
		let bxw:f64=self.top.boxw;
		let bxh:f64=self.top.boxh;
		let bxs_n:usize=self.bxs.len();
		let mut topxs:f64=0.0;
		if bxs_n%2==0{
			topxs=(bxs_n/2) as f64*(bxw*n as f64)+(((bxs_n/2)-1)as f64*xd)-0.5*(bxw*n as f64)+0.5*xd;
		}
		else{
			topxs=(bxs_n/2) as f64*(bxw*n as f64+xd);
		}
		let mut txt:String="".to_string();
		for i in 0..self.edges.len(){
			let idx:[usize;2]=self.edges[i][0];
			//let x:f64=idx[0] as f64*xd+idx[1] as f64*bxw+0.5*bxw;
			let x:f64=idx[0] as f64*(bxw*n as f64+xd)+idx[1] as f64*bxw+0.5*bxw;
			let y:f64=bxh;
			println!("TOPXS: {}",topxs);
			let x_up:f64=topxs+self.edges[i][1][1] as f64*bxw+0.5*bxw;
			txt+=&format!("\\psline{{->}}({},{})({},{})\n",x,y,x_up,y_up);
		}
		println!("{}",txt);
		txt
	}
	
}
#[derive(Debug)]
pub struct boxes{
	boxw:f64,
	boxh:f64,
	contents:Vec<usize>,
	xshift:f64,
	yshift:f64,
}
impl boxes{
	pub fn new()->Self{
		Self{
			boxw:0.0,
			boxh:0.0,
			contents:vec![],
			xshift:0.0,
			yshift:0.0,
		}
	}
	pub fn put_size(&mut self,boxw_:f64,boxh_:f64){
		self.boxw=boxw_;
		self.boxh=boxh_;
	}
	pub fn put_shift(&mut self,x:f64,y:f64){
		self.xshift=x;
		self.yshift=y;
	}
	pub fn put_contents(&mut self,contents_:&Vec<usize>){
		self.contents=contents_.clone();
	}
	pub fn get_tex(&self)->String{
		let mut txt:String="\\begin{pspicture}\n".to_string();
		let xs:f64=self.xshift;
		let ys:f64=self.yshift;
		let bxw:f64=self.boxw;
		let bxh:f64=self.boxh;
		for i in 0..self.contents.len(){
			txt+=&format!("\\psframe({},{})({},{})\\rput({},{}){{{}}}\n",xs+i as f64*bxw,ys,xs+(i+1)as f64*bxw,ys+bxh,xs+i as f64*bxw+0.5*bxw,ys+0.5*bxh,&format!("$a_{}$",self.contents[i]));
		}
		txt+="\\end{pspicture}\n";
		println!("{}",txt);
		return txt;
	}
}

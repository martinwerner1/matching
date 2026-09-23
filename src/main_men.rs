fn main(){
	display();
	display_1st_a();
	display_2nd_a();
	display_2nd_b();
}

fn display(){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=12;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("m{}{}",j+1,ny-i).to_string();
			let xcoord:f64=j as f64*xfactor;
			println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
			
			//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

		}
	}
}





fn display_1st_a(){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=12;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			let nodename:String=format!("m{}{}",j+1,ny-i).to_string();
			let xcoord:f64=j as f64*xfactor;
			println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
			
			//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

		}
	}
}




fn display_2nd_a(){
	let ny:usize=5;
	let nx:usize=7;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=5;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			if j%2==0{
				let nodename:String=format!("m{}{}",j+1,ny-i+5).to_string();
				let xcoord:f64=j as f64*xfactor;
				println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
				//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);

			}
		}
	}
}
fn display_2nd_b(){
	let ny:usize=5;
	let nx:usize=6;
	let xfactor:f64=1.0;
	let yfactor:f64=0.7;
	let yshift:usize=0;
	for i in 0..ny{
		let ycoord:f64=(yshift+i) as f64*yfactor;
		for j in 0..nx{
			if (j+1)%2==0{
				let nodename:String=format!("m{}{}",j+1,ny-i+10).to_string();
				let xcoord:f64=j as f64*xfactor;
				println!("\\Cnode({:2},{:2}){{{}}}",xcoord,ycoord,nodename);
				//println!("\\Cnode({:2},{:2}){{{}}}\\rput({},{}){{{}}}",xcoord,ycoord,nodename,xcoord,ycoord,nodename);
			}
		}
	}
}

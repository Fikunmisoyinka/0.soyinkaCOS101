fn main() {
	let p:f64 =100.0;
	let r:f64 =1.0;
	let t:f64 =2.0;

	// simple interest
	let a = p * ( 1.0 + (r/100.0))* t;
	println!("Amount is {}", a );
	let si = a -p;
	println!("simple interst is{}",si );
}
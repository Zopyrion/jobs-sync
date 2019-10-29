use std::env;
use std::process;
mod header;

fn main() {
    let args: Vec<_> = env::args().collect();
	
    if args.len() != 2 {
        println!("Usage: ./main <jobs_file>\n");
		process::exit(0);
    }
	
	println!("args len {}\n", args.len());
}

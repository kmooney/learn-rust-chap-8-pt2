use std::io::stdin;

fn pig_latinize(s : &String) -> String {
	let vowels: Vec<&str> = vec!("a", "e", "i", "o", "u");

	let mut retval: String  = String::new();
	let first_letter = &s[0..1];
	let rest_of_string = &s[1..];
	match vowels.iter().find(|&&vowel| vowel == first_letter) {
		Some(_) => {
			retval.push_str(&s.trim()[..]);
			retval.push_str("hay");
		},
		None => {
			retval.push_str(rest_of_string.trim());
			retval.push_str(first_letter);
			retval.push_str("ay");
		}
	} 
	return retval;
}

fn main() {
	
	loop {
		println!("Okay, bud. Type a word and I'll make it pig latin.");
	    let mut input_string:String = String::new();
	    match stdin().read_line(&mut input_string) {
	    	Ok(num_bytes) => {
	    		if num_bytes == 0 {
	    			println!("okay, bye.");
	    			return;
	    		}
	    		println!("I see you typed {}.  The pig latin of that is {}", input_string.trim(), pig_latinize(&input_string))
	    	},
	    	Err(_) => {
	    		println!("I don't know dude.");
	    	}
	    }
	}
}

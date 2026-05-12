fn main() {
	let bin = hex_to_bin("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

	println!("Binary: {}", bin);
}

fn hex_to_bin(input: &str) -> String {
	//add leading zero to hex number if it's not padded
	let hex: String = if input.len() % 2 != 0 {
		format!("0{}", input)
	} 
	else {
		input.to_string()
	};

	//convert to four bit binary
	let binary = hex.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect();
	return binary;
}

/*
fn bin_to_base(bin: &str) -> String {

}
*/

fn main() {
	let bin = hex_to_bin("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

	println!("Binary: {}", bin);

	let base = bin_to_base(bin.as_str());
	println!("Base64: {}", base);

	let bin_again = base_to_bin(base.as_str());

	println!("Binary: {}", bin_again);

	//compare bin and bin_again
	if bin == bin_again {
		println!("binary match!");
	}

	let hex = bin_to_hex(bin_again.as_str());
	
	println!("Hex: {}", hex);
}

pub fn hex_to_bin(input: &str) -> String {
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

pub fn bin_to_base(bin: &str) -> String {
	let alph_map: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; 

	//convert binary to bytes
	let bytes = bin.as_bytes();

	//split into 6 bit chunks
	let bin_chunks: Vec<&str> = bytes.chunks(6)
		.map(|chunk| std::str::from_utf8(chunk).unwrap())
		.collect();

	//ensure that the last chunk has 6 bits by appending 0s
	let mut v: Vec<String> = bin_chunks.iter().map(|s| s.to_string()).collect();
	while v.last().map_or(false, |s| s.len() < 6) {
		if let Some(mut last) = v.pop() {
			last.push('0');
			v.push(last);
		}
	}

	//convert binary to base ten by hand
	let chunks: Vec<u32> = v.iter()
		.map(|chunk| chunk.chars()
				.rev()
				.enumerate()
				.filter(|&(_, c)| c == '1')
				.map(|(i, _)| 2u32.pow(i as u32))
				.sum::<u32>())
		.collect();

	//convert base ten with map
	let chars: Vec<String> = chunks.iter().map(|&n| {
		let byte = alph_map.as_bytes()[n as usize];
		(byte as char).to_string()
	}).collect();

	//merge into single string
	let result = chars.join("");

	return result;	
}

pub fn base_to_bin(base: &str) -> String {
	//split string to char array
	let chars: Vec<char> = base.chars().collect();

	let alph_map: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; 
	
	//convert chars to numbers using mapping
	let chunks: Vec<u32> = chars.iter().map(|&c| alph_map.find(c).map(|n| n as u32).unwrap_or(0)).collect();

	//convert numbers to binary
	let bin_chunks: Vec<String> = chunks.iter().map(|n| format!("{:0>6}", format!("{:b}", n)).to_string()).collect();

	let result = bin_chunks.join("");
		
	return result;

}

pub fn bin_to_hex(bin: &str) -> String {
	//split into 8 bit chunks
	let bytes = bin.as_bytes();
	let bin_chunks: Vec<&str> = bytes.chunks(8)
		.map(|chunk| std::str::from_utf8(chunk).unwrap())
		.collect();

	//convert from binary to numbers
	let chunks: Vec<u32> = bin_chunks.iter()
		.map(|chunk| chunk.chars()
				.rev()
				.enumerate()
				.filter(|&(_, c)| c == '1')
				.map(|(i, _)| 2u32.pow(i as u32))
				.sum::<u32>())
		.collect();
	
	//convert bytes into hex
	let hex: String = chunks.iter().map(|c| format!("{:02x}", c)).collect();

	return hex;
}

pub fn hex_to_plain(hex: &str) -> String {
	let bin = hex_to_bin(hex);

	let bytes: Vec<u8> = bin.as_bytes().chunks(8).map(|b| bin_to_byte(std::str::from_utf8(b).unwrap())).collect();

	let chars: Vec<char> = bytes.iter().map(|b| byte_to_char(*b)).collect();

	let result = chars.iter().collect();

	return result;
}

pub fn char_to_byte(plain: char) -> u8 {
	let byte = plain as u8;

	return byte;
}

pub fn byte_to_char(byte: u8) -> char {
	let plain = byte as char;

	return plain;
}

pub fn byte_to_bin(byte: u8) -> String {
	let bin: String = format!("{:0>8}", format!("{:b}", byte));

	return bin;
}

pub fn bin_to_byte(bin: &str) -> u8 {
	let byte: u8 = u8::from_str_radix(bin, 2).unwrap();

	return byte;
}

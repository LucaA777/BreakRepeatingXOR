#[path = "../Challenge1/converters.rs"]
mod converters;

fn main() {
	let result = fixedXOR_hex("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");

	println!("XOR: {}", result); 

}

pub fn fixedXOR_hex(in1: &str, in2: &str) -> String {
	//convert both hex to bin
	let bin1 = converters::hex_to_bin(in1);
	let bin2 = converters::hex_to_bin(in2);

	//perform xor
	let XOR: Vec<char> = bin1.chars().zip(bin2.chars()).map(|(b1, b2)| {
			if b1 == b2 {
			'0'
			}
			else {
			'1'
			}
			}).collect();

	//combine vector to string
	let result: String = XOR.iter().collect();

	//convert back to hex
	let hex_result = converters::bin_to_hex(result.as_str());

	return hex_result;
}

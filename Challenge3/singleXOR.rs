#[path = "../Challenge1/converters.rs"]
mod converters;

#[path = "../Challenge2/fixedXOR.rs"]
mod fixedXOR;

fn main() {
	let message = singleXOR("abcd", 69);

	println!("Encoded: {}", message);

}

pub fn singleXOR(plain: &str, key: u8) -> String {
	//byte xor on each character
	let encoded: Vec<char> = plain.chars().map(|c| converters::byte_to_char(fixedXOR::fixedXOR_byte(converters::char_to_byte(c), key))).collect();

	let result: String = encoded.iter().collect();

	return result;	
}

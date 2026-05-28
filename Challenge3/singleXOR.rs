#[path = "../Challenge1/converters.rs"]
mod converters;

#[path = "../Challenge2/fixedXOR.rs"]
mod fixedXOR;

fn main() {

	let hex_encoded_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

	let plain_string = converters::hex_to_plain(hex_encoded_string);

	println!("Encoded plaintext: {}", plain_string);

	decode_singleXOR(plain_string.as_str(), 10);
}

pub fn singleXOR(plain: &str, key: u8) -> String {
	//byte xor on each character
	let encoded: Vec<char> = plain.chars().map(|c| converters::byte_to_char(fixedXOR::fixedXOR_byte(converters::char_to_byte(c), key))).collect();

	let result: String = encoded.iter().collect();

	return result;	
}

pub fn decode_singleXOR(plain: &str, number: u8) -> String {
	
	let mut results: Vec<(String, u8, f32)> = vec![];

	for i in 0..=255 {
		let decoded = singleXOR(plain, i);
		let score = english_score(decoded.as_str().as_bytes());
		
		//add to results if score is high enough
		if results.len() < number.into() || results.last().unwrap().2 < score {
			//insert ordered by score
			let mut j = 0;
			while j < results.len() && results[j].2 >= score {
				j += 1;
			}
			results.insert(j, (decoded, i, score)); 

			if results.len() > number.into() {
				results.pop();
			}
		}
	}

	println!("{:?}", results);

	return "HEY!!!".to_string();
}

//with thanks to Google AI summary...
pub fn english_score(text: &[u8]) -> f32 {
	// Frequency distribution for common English letters (case-insensitive + space)
	let mut score = 0.0;
	for &byte in text {
		score += match byte {
			b'e' | b'E' => 12.02,
				b't' | b'T' => 9.10,
				b'a' | b'A' => 8.12,
				b'o' | b'O' => 7.68,
				b'i' | b'I' => 7.31,
				b'n' | b'N' => 6.95,
				b's' | b'S' => 6.28,
				b'h' | b'H' => 5.92,
				b'r' | b'R' => 6.02,
				b'd' | b'D' => 4.32,
				b'l' | b'L' => 3.98,
				b'c' | b'C' => 3.34,
				b' ' => 15.0, // Space gets a high score for valid sentences
				_ => 0.0,
		};
	}
	score
}

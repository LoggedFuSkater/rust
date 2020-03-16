
extern crate crypto;
extern crate hex;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::convert::TryInto;
use std::convert::TryFrom;


mod adverbs;
mod adjectives;
mod nouns;

#[cfg(test)]
mod tests;

/**
    obf obfuscates the input string into a human readable hash.
*/
pub fn obf(input: &str) -> String {
    obfp(input, 0)
}

/**
    obfp obfuscates the input string into a human readable hash with 0-8 padding_bytes at the end.
*/
pub fn obfp(input: &str, padding_bytes: u8) -> String {
    let safe_padding = match padding_bytes {
        0..=8 => padding_bytes,
        _ => 8,
    };
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    let mut result: [u8;20] = [0;20];
    hasher.result(&mut result);

    let adverbs_index = usize::try_from(u32::from_le_bytes(result[0..4].try_into()
        .expect("Could not convert slice to array")))
        .expect("Could not convert u32 to usize");
    let adjectives_index = usize::try_from(u32::from_le_bytes(result[4..8].try_into()
        .expect("Could not convert slice to array")))
        .expect("Could not convert u32 to usize");
    let nouns_index = usize::try_from(u32::from_le_bytes(result[8..12].try_into()
        .expect("Could not convert slice to array")))
        .expect("Could not convert u32 to usize");

    let adverb = adverbs::WORDS[adverbs_index % adverbs::WORDS.len()];
    let adjective = adjectives::WORDS[adjectives_index % adjectives::WORDS.len()];
    let noun = nouns::WORDS[nouns_index % nouns::WORDS.len()];

    let padding: Vec<u8> = result[12..(12 + safe_padding as usize)].try_into().expect("Could not convert padding to Vec");

    format!("{}{}{}{}", adverb, adjective, noun, hex::encode_upper(padding))
}

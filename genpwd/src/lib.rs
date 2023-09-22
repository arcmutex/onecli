use rand::prelude::*;

pub fn gen(
    length: usize,
    exclude_lowercase_char: bool,
    exclude_uppercase_char: bool,
    exclude_digital_char: bool,
    exclude_special_char: bool,
) -> String {
    let mut rng = rand::thread_rng();

    let mut lowercase_chars = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let mut uppercase_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<_>>();
    let mut digital_chars = "0123456789".chars().collect::<Vec<_>>();
    let mut special_chars = "!@#$%^&*+=?".chars().collect::<Vec<_>>();
    let mut all_chars = vec![];

    lowercase_chars.shuffle(&mut rng);
    uppercase_chars.shuffle(&mut rng);
    digital_chars.shuffle(&mut rng);
    special_chars.shuffle(&mut rng);

    let mut candidates = vec![];

    if !exclude_lowercase_char {
        all_chars.extend(lowercase_chars.iter());

        let index: usize = rng.gen_range(0..lowercase_chars.len());
        candidates.push(lowercase_chars[index]);
    }

    if !exclude_uppercase_char {
        all_chars.extend(uppercase_chars.iter());

        let index: usize = rng.gen_range(0..uppercase_chars.len());
        candidates.push(uppercase_chars[index])
    }

    if !exclude_digital_char {
        all_chars.extend(digital_chars.iter());

        let index: usize = rng.gen_range(0..digital_chars.len());
        candidates.push(digital_chars[index])
    }

    if !exclude_special_char {
        all_chars.extend(special_chars.iter());

        let index: usize = rng.gen_range(0..special_chars.len());
        candidates.push(special_chars[index])
    }

    all_chars.shuffle(&mut rng);

    let mut current_len = candidates.len();

    while current_len < length {
        let index: usize = rng.gen_range(0..all_chars.len());
        candidates.push(all_chars[index]);
        current_len += 1;
    }

    candidates.shuffle(&mut rng);

    return candidates.iter().collect::<String>();
}

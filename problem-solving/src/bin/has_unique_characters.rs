use std::collections::HashSet;

fn main() -> Result<(), String> {
    let input = String::from("helo wrld");
    let mut input = input.chars().collect::<Vec<char>>();
    println!("{}", has_unique_characters2(&mut input));
    Ok(())
}

fn has_unique_characters(input: &[char]) -> bool {
    let mut set = HashSet::new();
    for letter in input {
        if set.contains(letter) {
            return false;
        }
        set.insert(letter.clone());
    }
    return true;
}

fn has_unique_characters2(input: &mut [char]) -> bool {
    input.sort();
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            return false;
        }
    }
    return true;
}

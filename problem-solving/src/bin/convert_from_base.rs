fn main() -> Result<(), String> {
    println!("{}", convert_from_base(&['1', '1'], 2)?);
    Ok(())
}

fn convert_from_base(number: &[char], base: u8) -> Result<u32, String> {
    if base < 2 || (base > 10 && base != 16) {
        return Err(format!("{} is not a valid base", base));
    }
    let mut result: u32 = 0;

    for i in 0..number.len() {
        let value = symbol_to_value(number[i])?;
        if value >= base {
            return Err("The symbol cannot be greater than or equal the base".to_string());
        }
        let exp = (number.len() - i - 1) as u32;
        result += value as u32 * base.pow(exp) as u32;
    }
    Ok(result)
}

fn symbol_to_value(symbol: char) -> Result<u8, String> {
    match symbol {
        '0'..='9' => Ok(symbol as u8 - '0' as u8),
        'a'..='f' => Ok(symbol as u8 - 'a' as u8 + 10),
        'A'..='F' => Ok(symbol as u8 - 'A' as u8 + 10),
        _ => Err(format!("Symbol '{}' not recognized", symbol)),
    }
}

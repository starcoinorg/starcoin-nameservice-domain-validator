use regex::Regex;

pub fn validator(input: Vec<u8>) -> Option<Vec<u8>> {
    // reject large inputs
    if input.len() > 2048 {
        return None;
    }
    let str = String::from_utf8(input).ok()?;
    let (result, time_ms) = filter(&str);
    let _gas = time_ms * 1; // factor
    if result {
        Some(str.into_bytes())
    } else {
        None
    }
}

#[inline]
fn filter(s: &String) -> (bool, usize) {
    lazy_static::lazy_static! {
        static ref GENERATED_REGEX: Regex = Regex::new(include!("../regex.txt")).unwrap();
    }
    let timer = std::time::Instant::now();
    let res = GENERATED_REGEX.is_match(s);
    let time_ms = timer.elapsed().as_millis() as usize;
    (res, time_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "HelloWorld你好吗ǎǚΩ😄💃🤡ㄓおはようアアア한글".to_string();
        let output = filter(&input);
        assert_eq!(output.0, true);
    }
}

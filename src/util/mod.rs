use chrono::Local;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub mod consts;
pub mod custom_logger;
pub mod db;
pub mod hint;

pub const ID_CHARS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub fn general_id() -> String {
    let mut rng = thread_rng();
    let mut id = String::with_capacity(8);
    for _ in [(); 8] {
        id.push(*ID_CHARS.choose(&mut rng).unwrap());
    }
    id
}

pub fn now_time() -> String {
    let now = Local::now();
    format!("{}", now.format("%H:%M:%S"))
}

#[cfg(test)]
mod test {
    use crate::util::general_id;

    #[test]
    fn test() {
        println!("{}", u16::MAX);
        println!("{}", general_id());
        println!("{}", general_id());
        println!("{}", general_id());
    }
}

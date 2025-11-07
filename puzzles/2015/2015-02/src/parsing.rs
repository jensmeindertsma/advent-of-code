use crate::present::Present;

pub fn present(input: &str) -> Present {
    let mut parts = input.trim().split("x").map(|x| x.parse().unwrap());

    Present {
        length: parts.next().unwrap(),
        width: parts.next().unwrap(),
        height: parts.next().unwrap(),
    }
}

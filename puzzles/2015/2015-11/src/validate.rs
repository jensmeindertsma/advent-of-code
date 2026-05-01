use itertools::Itertools;

pub fn is_valid_password(password: &str) -> bool {
    let has_straight_three = password
        .as_bytes()
        .iter()
        .tuple_windows()
        .any(|(a, b, c)| c - 1 == *b && b - 1 == *a);

    let has_illegal_letters = password.chars().any(|c| matches!(c, 'i' | 'o' | 'l'));

    let has_two_overlapping = password
        .chars()
        // chunk_by will turn "aabbbcaa" into [aa] [bbb] [c] [aa]
        .chunk_by(|&c| c)
        .into_iter()
        .filter_map(|(c, group)| {
            // We filter out groups that are just 1 character long
            let count = group.count();
            if count == 2 { Some(c) } else { None }
        })
        // by mapping to character we can use .unique to make sure the two pairs are different
        .unique()
        .count()
        >= 2;

    has_straight_three && !has_illegal_letters && has_two_overlapping
}

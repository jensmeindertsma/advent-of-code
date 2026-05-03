mod part_one;
mod part_two;

pub use part_one::part_one;
pub use part_two::part_two;

#[test]
fn one() {
    assert_eq!(part_one("[1,2,3]"), 6);
    assert_eq!(part_one("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(part_one("[[[3]]]"), 3);
    assert_eq!(part_one("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    assert_eq!(part_one("{\"a\":[-1,1]}"), 0);
    assert_eq!(part_one("[-1,{\"a\":1}]"), 0);
    assert_eq!(part_one("[]"), 0);
    assert_eq!(part_one("{}"), 0);

    assert_eq!(part_one(include_str!("../input.txt")), 156366);
}

#[test]
fn two() {
    assert_eq!(part_two("[1,2,3]"), 6);
    assert_eq!(part_two("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    assert_eq!(part_two("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    assert_eq!(part_two("[1,\"red\",5]"), 6);

    assert_eq!(part_two(include_str!("../input.txt")), 96852);
}

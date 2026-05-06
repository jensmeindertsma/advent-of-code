mod parsing;
mod part_one;

pub use part_one::part_one;

#[test]
fn one() {
    use indoc::indoc;

    assert_eq!(
        part_one(indoc! {"
            Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "}),
        62842880
    );

    //assert_eq!(part_one(include_str!("../input.txt")), 21367368);
}

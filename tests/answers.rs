macro_rules! test {
    ($day:expr, $part1:expr, $part2:expr) => {
        paste::paste! {
            #[test]
            fn [<test_day $day>]() {
                assert_cmd::Command::cargo_bin(concat!("day", $day))
                    .unwrap()
                    .assert()
                    .success()
                    .stdout(concat!("part 1: ", $part1, "\npart 2: ", $part2, "\n"));
            }
        }
    };
}

test!(1, 970816, 96047280);
test!(2, 655, 673);
test!(3, 268, 3093068400);
test!(4, 256, 198);
test!(5, 864, 739);
test!(6, 6703, 3430);

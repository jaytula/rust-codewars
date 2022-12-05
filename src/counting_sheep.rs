// https://www.codewars.com/kata/54edbc7200b811e956000556/solutions/rust

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep
        .iter()
        .fold(0, |acc, &val| if val { acc + 1 } else { acc })
}

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}

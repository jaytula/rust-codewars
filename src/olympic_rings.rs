// https://www.codewars.com/kata/57d06663eca260fe630001cc/solutions/rust

fn olympic_ring(s: &str) -> String {
    let q = s.chars().fold(0, |acc, val| {
        if "AabDdegPpQqROo".contains(val) {
            return acc + 1;
        }
        if "B".contains(val) {
            return acc + 2;
        }
        acc
    }) / 2;

    match q {
        0..=1 => "Not even a medal!",
        2 => "Bronze!",
        3 => "Silver!",
        _ => "Gold!",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::olympic_ring;

    #[test]
    fn basic() {
        assert_eq!(olympic_ring("wHjMudLwtoPGocnJ"), "Bronze!");
        assert_eq!(olympic_ring("eCEHWEPwwnvzMicyaRjk"), "Bronze!"); 
        assert_eq!(olympic_ring("JKniLfLW"), "Not even a medal!"); 
        assert_eq!(olympic_ring("EWlZlDFsEIBufsalqof"), "Silver!"); 
        assert_eq!(olympic_ring("IMBAWejlGRTDWetPS"), "Gold!");
        assert_eq!(olympic_ring("yZEomfQGWznwpHbazlkLV"), "Bronze!");
    }
}

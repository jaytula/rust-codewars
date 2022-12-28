// https://www.codewars.com/kata/562f91ff6a8b77dfe900006e/solutions/rust

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let mut how_many_times = 1;
    let system_a = |i: i32| ticket * i;
    let system_b =
        |i: i32| card as f64 + (1..=i).fold(0.0, |acc, val| acc + (ticket as f64) * perc.powf(val as f64));

    loop {
        let a_amount = system_a(how_many_times);
        let b_amount = system_b(how_many_times).ceil() as i32;
        if a_amount > b_amount {
            return how_many_times;
        }
        how_many_times += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
        println!(" card: {:?};", card);
        println!("ticket: {:?};", ticket);
        println!("perc: {:?};", perc);
        let ans = movie(card, ticket, perc);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(500, 15, 0.9, 43);
        dotest(100, 10, 0.95, 24);
        // 2 * 10 = 20
        // 0 + .95*10
        dotest(0, 10, 0.95, 2);
    }
}

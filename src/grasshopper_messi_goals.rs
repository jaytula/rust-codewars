// https://www.codewars.com/kata/55ca77fa094a2af31f00002a/solutions/rust

static LA_LIGA_GOALS: u32 = 43;
static CHAMPIONS_LEAGUE_GOALS: u32 = 10;
static COPA_DEL_REY_GOALS: u32 = 5;

static TOTAL_GOALS: u32 = LA_LIGA_GOALS + CHAMPIONS_LEAGUE_GOALS + COPA_DEL_REY_GOALS;

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(TOTAL_GOALS, 58, "total goals should equal to 58");
    }
}

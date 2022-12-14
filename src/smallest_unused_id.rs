// https://www.codewars.com/kata/55eea63119278d571d00006a/solutions/rust

fn next_id(ids: &[usize]) -> usize {
    //this will be awesome!
    let mut ids = ids.to_vec();
    ids.sort();
    let mut prev = 0usize;
    for (i, num) in ids.iter().enumerate() {
        if i == 0 && *num != 0usize { return 0; }
        if prev == *num { continue; }
        if prev + 1 != *num { return prev + 1; }
        prev = *num
    }

    match ids.last() {
        None => 0,
        Some(num) => *num+1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        // assert_eq!(next_id(&[0,1,2,4,5]), 3);
        assert_eq!(next_id(&[0,1,2,3,4,5,6,7,8,9,10]), 11);
    }
}

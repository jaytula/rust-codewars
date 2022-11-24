fn subtract_sum(n: u32) -> &'static str {
    let mut new_n = n;
    let mut has_run = false;
    let fruits = ["kiwi","pear","kiwi","banana","melon","banana","melon","pineapple","apple","pineapple","cucumber","pineapple","cucumber","orange","grape","orange","grape","apple","grape","cherry","pear","cherry","pear","kiwi","banana","kiwi","apple","melon","banana","melon","pineapple","melon","pineapple","cucumber","orange","apple","orange","grape","orange","grape","cherry","pear","cherry","pear","apple","pear","kiwi","banana","kiwi","banana","melon","pineapple","melon","apple","cucumber","pineapple","cucumber","orange","cucumber","orange","grape","cherry","apple","cherry","pear","cherry","pear","kiwi","pear","kiwi","banana","apple","banana","melon","pineapple","melon","pineapple","cucumber","pineapple","cucumber","apple","grape","orange","grape","cherry","grape","cherry","pear","cherry","apple","kiwi","banana","kiwi","banana","melon","banana","melon","pineapple","apple","pineapple"];

    while new_n > 100 || !has_run {
        has_run = true;
        let mut sum = 0;
        let mut number = new_n;
        while number > 0 {
            sum += number % 10;
            number /= 10;
        }
        new_n = new_n - sum;
    }

    let result = fruits[(new_n as usize)-1];
    return result;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(subtract_sum(10), "apple");
        assert_eq!(subtract_sum(325), "apple");
        assert_eq!(subtract_sum(1026), "apple");
        assert_eq!(subtract_sum(947), "apple");

    }
}

use std::collections::HashSet;

/* only 39 strict keywords counts, no reserved like abstract or override - as of 2021 Edition */
fn keywords() -> HashSet<&'static str> {
    HashSet::from([
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", 
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", 
        "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", 
        "unsafe", "use", "where", "while", "async", "await", "dyn", "union"])
}

/* If you'll find some definitions in tests too obscure or too obvious, don't hesitate to
open an issue with a better replacement! */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(keywords().len(), 39, "there should be 39 keywords!");
    }
}

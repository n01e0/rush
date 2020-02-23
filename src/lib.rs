pub mod rush;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alias_test() {
        let mut shell = rush::Shell::new() ;
        shell.prompt("$").flush();
        let mut line = vec!["alias", "l=ls"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        shell.exec(line).finish();
        assert!(shell.alias.contains_key("l"));
        assert_eq!(shell.check_alias(vec!["l".to_string()]), vec!["ls".to_string()]);
        assert!(shell.check_alias(vec!["l".to_string()]).len() > 0);
    }
}

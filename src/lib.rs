peg::parser! {
    pub grammar list_parser() for str {
        rule number() -> u32 
            = n:$(['0'..='9']+) {? n.parse().or(Err("expected a valid u32")) }
        
        pub rule list() -> Vec<u32> 
            = "[" l:(number() ** ",") "]" { l }
    }
}

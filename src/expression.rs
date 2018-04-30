use std::vec::Vec;

pub struct Parser {
    stack : Vec<String>,
    previous_was_operator : bool
}

impl Parser {

    pub fn new () -> Parser {
        return Parser { stack : Vec::new (), previous_was_operator : false };
    }

    fn copy_chars(vec : &Vec<char>) -> String {
        //use std::ops::Add;
        use std::iter::FromIterator;
        let s = String::from_iter(vec); // "".to_string();

        //for ch in vec {
        //    s.add(ch.to_string().as_str());
        //}
        return s;
    }
    pub fn parse_tokens (&mut self) {
    }
    pub fn parse_text (&mut self, text : &str) {
        let mut  token : Vec<char>  = Vec::new();
        for ch in text.chars() {
            if ch.is_alphabetic() {
                token.push(ch);
            } else if ch.is_numeric() {
                token.push(ch);
            } else {
                if token.len() > 0 {
                    self.next (Parser::copy_chars(&token)).unwrap_err();
                } else {
                    self.next (ch.to_string()).unwrap_err();
                }
                token.clear();
            }
        }
        if token.len() > 0 {
            self.next (Parser::copy_chars(&token)).unwrap_err();
        }
    }
    fn next (&mut self, in_token : String) -> Result<bool,&str> {
        let mut token = in_token.to_string();
        if token == " " {
            return Ok(true);
        }
        if token == ")" {
            loop {
                if self.stack.len() == 0 {
                    return Err("Mismatched parenthesis, missing '('");
                }
                let previous_operator = self.stack.pop ().unwrap();
                if previous_operator == "(" {
                    return Ok(true);
                }
                self.backend_next (&previous_operator);
                if previous_operator.ends_with("(") {
                    return Ok(true);
                }
            }
        }
        if self.previous_was_operator {
            if      token == "-" {
                token = "_-".to_string();
            } else if token == "+" {
                return Ok(true);
            }  // An optimization just unary plus
        }
        let priority = Parser::priority (&token);
        if priority == -1 {
            self.previous_was_operator = false;
            self.backend_next (&token);  // Just output any literals directly
            return Ok(true);
        }
        self.previous_was_operator = true;

        if priority == -2 {
            self.stack.push (token);
            return Ok(true);
        }
        let right_to_left_associative = Parser::right_to_left_associative(&token);
        while ! self.stack.len () == 0 {
            let previous_operator          = self.stack [self.stack.len()-1].clone();  //.peek ();
            let previous_operator_priority = Parser::priority (&previous_operator);
            if right_to_left_associative && previous_operator_priority <= priority {
                break
            }
            if previous_operator_priority < priority {
                break
            }
            self.stack.pop ().unwrap();
            self.backend_next (&previous_operator);
        }
        self.stack.push (token);
        return Ok(true);
    }

    fn backend_next (&self, token : &String) {
        print!("{} ", token);
    }
    fn right_to_left_associative(token : &String) -> bool
    {
        match token.as_str() {
            "~" => return true,
            "-+" => return true,
            "_-" => return true,
            "^" => return true,
            _ => return false
        }
    }
    fn priority (token : &String) -> i32
    {
        match token.as_str() {
            "~" => return 100,
            "-+" => return 100,
            "_-" => return 100,
            "^"  => return 90,
            "*"  => return 80,
            "/"  => return 80,
            "+"  => return 70,
            "-"  => return 70,
            "<"  => return 60,
            "<=" => return 60,
            ">=" => return 60,
            ">"  => return 60,
            "==" => return 50,
            "!=" => return 50,
            "&"  => return 40,
            "|"  => return 30,
            " "  => return 10,  // Check
            ","  => return 10,
            ")"  => return -2,
            _ => {
                if token.ends_with("(") {
                    return -2;
                }
                return -1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use expression::Parser;

    fn should_parse () {
        let mut parser = Parser::new ();
        parser.parse_text("1+2");
    }
}

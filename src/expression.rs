use std::vec::Vec;

pub struct Parser {
    stack: Vec<String>,
    execution_stack: Vec<f64>,
    previous_was_operator: bool,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser { stack: Vec::new(), execution_stack: Vec::new(), previous_was_operator: true };
    }

    fn copy_chars(vec: &Vec<char>) -> String {
        //use std::ops::Add;
        use std::iter::FromIterator;
        let s = String::from_iter(vec); // "".to_string();

        //for ch in vec {
        //    s.add(ch.to_string().as_str());
        //}
        return s;
    }

    /// Parses and evalues text an returns a value
    ///
    /// ## Examples
    ///
    /// TODO ```
    ///        extern crate tsvstat;
    ///        use tsvstat::expression;
    ///        use tsvstat::expression::Parser;
    ///        let parser = Parser::new ();
    ///        assert_eq! (parser.parse_text("7+2"), 9.);
    ///        assert_eq! (parser.parse_text("7+2+3"), 12.);
    ///        assert_eq! (parser.parse_text("7+2*3"), 13.);
    ///        assert_eq! (parser.parse_text("1+2*5"), 11.);
    ///        assert_eq! (parser.parse_text("(1+2)*5"), 15.);
    /// ```
    ///
    pub fn parse_text(&mut self, text: &str) -> f64 {
        let mut token: Vec<char> = Vec::new();
        for ch in text.chars() {
            println!("PARSE {} LEN={}", ch, token.len());
            if ch.is_alphabetic() {
                token.push(ch);
            } else if ch.is_numeric() {
                token.push(ch);
            } else {
                if token.len() > 0 {
                    self.next(Parser::copy_chars(&token)).unwrap();
                }
                self.next(ch.to_string()).unwrap();
                token.clear();
            }
        }
        if token.len() > 0 {
            self.next(Parser::copy_chars(&token)).unwrap();
            token.clear();
        }
        self.close();
        return self.pop ();
    }

    fn close(&mut self)
    {
        while self.stack.len() > 0 {
            let value = self.stack.pop().unwrap();
            self.backend_next(&value);
        }
        self.previous_was_operator = true;
    }

    fn next(&mut self, in_token: String) -> Result<bool, &str> {
        let mut token = in_token.to_string();

        println!("NEXT: {} StACK LEN={}", token, self.stack.len());

        if token == " " {
            return Ok(true);
        }
        if token == ")" {
            loop {
                if self.stack.len() == 0 {
                    return Err("Mismatched parenthesis, missing '('");
                }
                let previous_operator = self.stack.pop().unwrap();
                if previous_operator == "(" {
                    return Ok(true);
                }
                self.backend_next(&previous_operator);
                if previous_operator.ends_with("(") {
                    return Ok(true);
                }
            }
        }
        if self.previous_was_operator {
            if token == "-" {
                token = "_-".to_string();
            } else if token == "+" {
                return Ok(true);
            }  // An optimization just unary plus
        }
        let priority = Parser::priority(&token);
        println!("PRIORITY {} token={} Parser::right_to_left_associative={}", priority, token, Parser::right_to_left_associative(&token));
        if priority == -1 {
            self.previous_was_operator = false;
            self.backend_next(&token);  // Just output any literals directly
            return Ok(true);
        }
        self.previous_was_operator = true;

        if priority == -2 {
            self.stack.push(token);
            return Ok(true);
        }
        let right_to_left_associative = Parser::right_to_left_associative(&token);
        while self.stack.len() > 0 {
            let previous_operator = self.stack[self.stack.len() - 1].clone();  //.peek ();
            println!("previous_operator={}", previous_operator);
            let previous_operator_priority = Parser::priority(&previous_operator);
            if right_to_left_associative && previous_operator_priority <= priority {
                break;
            }
            if previous_operator_priority < priority {
                break;
            }
            self.stack.pop().unwrap();
            println!("ABC {}", previous_operator);
            self.backend_next(&previous_operator);
        }
        self.stack.push(token);
        return Ok(true);
    }


    fn backend_next(&mut self, token: &String) {
        //println!("STACK: {} ", token);

        if token == "+" {
            self.add();
            println!("ADD: {} LEN:{}", token, self.execution_stack.len());
        } else if token == "-" {
            self.subtract();
            println!("SUB: {} LEN:{}", token, self.execution_stack.len());
        } else if token == "*" {
            self.times();
            println!("TIMES: {} LEN:{}", token, self.execution_stack.len());
        } else if token == "^" {
            self.power();
            println!("TIMES: {} LEN:{}", token, self.execution_stack.len());
        } else if token == "_-" {
            self.minus();
            println!("MINUS: {} LEN:{}", token, self.execution_stack.len());
        } else {
            match token.parse::<f64>() {
                Err(err) => {
                    eprintln!("Cannot parse: {} - {}", token, err);
                    //self.execution_stack.push (NaN)
                },
                Ok(value) => self.execution_stack.push (value)
            }
        }
    }
    fn add(&mut self) {
        let top1 = self.pop();
        let top2 = self.pop();
        self.push(top1 + top2);
    }
    fn subtract(&mut self) {
        let top1 = self.pop();
        let top2 = self.pop();
        self.push(top2 - top1);
    }
    fn minus(&mut self) {
        let top1 = self.pop();
        self.push(-top1);
    }
    fn times(&mut self) {
        let top1 = self.pop();
        let top2 = self.pop();
        self.push(top1 * top2);
    }
    fn power(&mut self) {
        let top1 = self.pop();
        let top2 = self.pop();
        self.push(top2.powf(top1));
    }
    fn pop(&mut self) -> f64 {
        return self.execution_stack.pop().unwrap();
    }
    fn push(&mut self, value: f64) {
        return self.execution_stack.push(value);
    }
    fn right_to_left_associative(token: &String) -> bool
    {
        match token.as_str() {
            "~" => return true,
            "_-" => return true,
            "^" => return true,
            _ => return false
        }
    }
    fn priority(token: &String) -> i32
    {
        match token.as_str() {
            "~" => return 100,
            "-+" => return 100,
            "_-" => return 100,
            "^" => return 90,
            "*" => return 80,
            "/" => return 80,
            "+" => return 70,
            "-" => return 70,
            "<" => return 60,
            "<=" => return 60,
            ">=" => return 60,
            ">" => return 60,
            "==" => return 50,
            "!=" => return 50,
            "&" => return 40,
            "|" => return 30,
            " " => return 10,  // Check
            "," => return 10,
            ")" => return -2,
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

    #[test]
    fn should_parse() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("7+2"), 9.);
        assert_eq!(parser.parse_text("7+2+3"), 12.);
        assert_eq!(parser.parse_text("7+2*3"), 13.);
        assert_eq!(parser.parse_text("1+2*5"), 11.);
        assert_eq!(parser.parse_text("(1+2)*5"), 15.);
        assert_eq!(parser.parse_text("2*5+9"), 19.);
        assert_eq!(parser.parse_text("1+2*5+9"), 20.);
        assert_eq!(parser.parse_text("2*2+2^3+1*2"), 14.);
        assert_eq!(parser.execution_stack.len(), 0);
    }
    #[test]
    fn should_parse_brackets() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("(1+2)*3"), 9.);
        assert_eq!(parser.parse_text("3*(1+2)"), 9.);
        assert_eq!(parser.parse_text("(1*2)+3"), 5.);
        assert_eq!(parser.parse_text("3+(1*2)"), 5.);
        assert_eq!(parser.parse_text("(1+2)*3^2"), 27.);
        assert_eq!(parser.parse_text("4*2^(1+2)"), 32.);
        assert_eq!(parser.parse_text("(1+(((2))))*((((3))^(2)))"), 27.);
        assert_eq!(parser.parse_text("10+-21"), -11.);
        assert_eq!(parser.execution_stack.len(), 0);
    }

    #[test]
    fn should_parse_longer_numbers() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("123456789"), 123456789.);
        assert_eq!(parser.parse_text("(1234567890)"), 1234567890.);
        assert_eq!(parser.parse_text("101+209"), 310.);
        assert_eq!(parser.parse_text("1001+209"), 1210.);
        assert_eq!(parser.execution_stack.len(), 0);
    }

    #[test]
    fn should_parse_prefixes() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("-13"), -13.);
        assert_eq!(parser.parse_text("-1"), -1.);
        assert_eq!(parser.parse_text("0"), 0.);
        assert_eq!(parser.parse_text("1"), 1.);
        assert_eq!(parser.parse_text("+1"), 1.);
        assert_eq!(parser.parse_text("+12"), 12.);
        assert_eq!(parser.execution_stack.len(), 0);

    }
    #[test]
    fn should_parse_prefix_brackets() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("-(-1)"), 1.);
        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("-(+1)"), -1.);
        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq!(parser.parse_text("-(0)"), 0.);
        assert_eq!(parser.parse_text("+(0)"), 0.);
        assert_eq!(parser.parse_text("+(+1)"), 1.);
        assert_eq!(parser.parse_text("+(-1)"), -1.);

        assert_eq!(parser.execution_stack.len(), 0);
    }

    #[test]
    fn should_parse_infix_prefix() {
        let mut parser = Parser::new();

        assert_eq!(parser.execution_stack.len(), 0);
        assert_eq! (parser.parse_text("1++1"), 2.);
        assert_eq! (parser.parse_text("1+-1"), 0.);
        assert_eq! (parser.parse_text("1-+1"), 0.);
        assert_eq! (parser.parse_text("1--1"), 2.);
        assert_eq! (parser.parse_text("1*1"), 1.);
        assert_eq! (parser.parse_text("1*-1"), -1.);
        assert_eq!(parser.execution_stack.len(), 0);

    }
}

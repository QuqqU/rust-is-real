use std::str::Chars;

pub(crate) type ParsingResult<T> = Result<T, ()>;

macro_rules! try_parse {
    ($self:ident, $func:ident) => {
        match $self.$func() {
            Ok(ret) => ret,
            Err(_) => return Err(()),
        }
    };
}

pub(crate) struct FloatingPoint<'a> {
    iter: Chars<'a>,
    curr: char,
}

impl<'a> FloatingPoint<'a> {
    pub(crate) fn accept(input: &'a String) -> bool {
        let mut automaton = Self {
            iter: input.chars(),
            curr: ' ',
        };
        automaton.next();

        match automaton.read().ok() {
            Some(_) => true,
            None => false,
        }
    }

    fn read(&mut self) -> ParsingResult<f64> {
        let (frac_len, fraction) = try_parse!(self, read_fraction);
        let exponent = try_parse!(self, read_exponent);
        try_parse!(self, read_eos);
        Ok((fraction as f64).powi(exponent - frac_len as i32))
    }
}

impl<'a> FloatingPoint<'a> {
    fn read_fraction(&mut self) -> ParsingResult<(usize, i64)> {
        let sign = try_parse!(self, read_sign);

        let mut ret = String::from("");
        while self.is_digit() {
            ret.push(self.curr);
            self.next();
        }

        let mut frac_len = 0;
        if self.curr == '.' {
            self.next();
            while self.is_digit() {
                frac_len += 1;
                ret.push(self.curr);
                self.next();
            }
        }
        let ret = ret;
        let frac_len = frac_len;

        match ret.parse::<i64>().ok() {
            Some(n) => {
                if sign == '+' {
                    Ok((frac_len, n))
                } else {
                    Ok((frac_len, -n))
                }
            }
            None => Err(()),
        }
    }

    fn read_exponent(&mut self) -> ParsingResult<i32> {
        // E/e
        if !try_parse!(self, read_e) {
            return Ok(0);
        }

        // +/-
        let sign = try_parse!(self, read_sign);

        // [0-9]
        let mut ret = String::from("");
        while self.is_digit() {
            ret.push(self.curr);
            self.next();
        }
        let ret = ret;

        match ret.parse::<i32>().ok() {
            Some(n) => {
                if sign == '+' {
                    Ok(n)
                } else {
                    Ok(-n)
                }
            }
            None => Err(()),
        }
    }

    fn read_eos(&self) -> ParsingResult<()> {
        match self.curr {
            '\0' => Ok(()),
            _ => Err(()),
        }
    }

    fn next(&mut self) {
        self.curr = self.iter.next().unwrap_or_default();
    }

    fn read_e(&mut self) -> ParsingResult<bool> {
        match self.curr {
            'E' | 'e' => {
                self.next();
                Ok(true)
            }
            '\0' => Ok(false),
            _ => Err(()),
        }
    }

    fn read_sign(&mut self) -> ParsingResult<char> {
        match self.curr {
            '+' | '-' => {
                let ret = self.curr;
                self.next();
                Ok(ret)
            }
            '\0' => Err(()),
            _ => Ok('+'),
        }
    }

    fn is_digit(&self) -> bool {
        match self.curr {
            n if '0' <= n && n <= '9' => true,
            _ => false,
        }
    }
}

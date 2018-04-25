use std::f64;

/// An accumulator adds up a sequence of numerical observations and providing
/// a set of summary statistics for those numbers.

#[derive(Copy, Clone)]
pub struct Accumulator {
    pub count: i64,
    pub min:   f64,
    pub max:   f64,
    pub sum:   f64,
    pub sum2:  f64,
    pub sum3:  f64,
    pub sum4:  f64
}

impl Accumulator {
    pub fn new () -> Accumulator {
        return Accumulator { count: 0, min: f64::INFINITY, max: f64::NEG_INFINITY, sum: 0., sum2: 0., sum3: 0., sum4: 0.}
    }
    pub fn update (&mut self, input: f64) {
    	self.count = self.count + 1;
        self.min   = if self.min < input { self.min } else { input };
        self.max   = if self.max > input { self.max } else { input };
        self.sum   = self.sum + input;
        let square = input * input;
        self.sum2  = self.sum2  + square;
        self.sum3  = self.sum3  + square*input;
        self.sum4  = self.sum4  + square*square;
    }
    pub fn count (&self) -> i64 {
        return self.count;
    }
    pub fn sum (&self) -> f64 {
        return self.sum;
    }
    pub fn min (&self) -> f64 {
        return self.min;
    }
    pub fn max (&self) -> f64 {
        return self.max;
    }
    pub fn range (&self) -> f64 {
        return self.max - self.min;
    }
    pub fn mean(&self) -> f64 {
        return self.sum / self.count as f64;
    }
    pub fn variance(&self) -> f64 {
        return (self.sum2 - self.sum * self.sum / self.count as f64) / self.count as f64;
    }
    pub fn sd(&self) -> f64 {
        return self.variance().sqrt();
    }
    /// See [Skewness](https://en.wikipedia.org/wiki/Skewness)
    pub fn skew(&self) -> f64 {
    	let divisor = self.count as f64;  // Population nn, Sample would be: nn -1;
 	let e_x3    = self.sum3 / divisor; // Third Non-Central Moment;
 	let mean    = self.mean ();
 	let sd      = self.sd ();
 	let sd2     = sd*sd;
 	let sd3     = sd2*sd;
 	return (e_x3 -3.0*mean*sd2  -mean*mean*mean) / sd3;
    }
    /// See https://en.wikipedia.org/wiki/Kurtosis
    pub fn kurtosis(&self) -> f64 {
	let  divisor = self.count as f64;  // Population: nn, Sample would be: nn -1;
	let  e_x2    = self.sum2 / divisor; // Second Non-Central Moment;
	let  e_x3    = self.sum3 / divisor; // Third Non-Central Moment;
	let  e_x4    = self.sum4 / divisor; // Fourth Non-Central Moment;
	let  mean    = self.mean ();
	let  var     = self.variance ();
	let  mean2   = mean*mean;
	let  mean4   = mean2*mean2;
	return (e_x4 - 4.0*e_x3*mean + 6.0*e_x2*mean2 -4.0*mean4 + mean4  )   / (var*var);
    }
    pub fn excess_kurtosis(&self) -> f64 {
    	return self.kurtosis () - 3.0;
    }
    pub fn coefficient_of_variation(&self) -> f64 {
        return self.sd () / self.mean ();
    }
}

/// Generates a table of summary statistics for a sequence of rows of numerical data.

pub struct Accumulators {
    headers : Vec<String>,
    columns : Vec<Accumulator>,
    rows : usize,
    column : usize
}

impl Accumulators {
    pub fn new() -> Accumulators
    {
        return Accumulators { headers : Vec::new (), columns: Vec::new (), rows: 0, column: 0 };
    }
    pub fn rows(&self) -> usize { return self.rows; }
    pub fn column(&self) -> usize { return self.column; } // COLUMNS

    pub fn new_row(&mut self)
    {
        self.rows = self.rows + 1;
        self.column = 0;
    }

    pub fn add_column_value(&mut self, input: &str) {
        match input.parse::<f64>() {
            Ok(input) => {
                if self.rows == 1 {
                    self.headers.push(self.column.to_string ());
                    self.columns.push(Accumulator::new());
                }
                self.columns[self.column].update(input)
            }
            Err(message) => {
                if self.rows == 1 {
                    self.headers.push(input.to_string());
                    self.columns.push(Accumulator::new());
                } else {
                    eprintln!("Error '{}' while parsing: {} on row={} col={}", message, input, self.rows(), self.column())
                }
            }
        }
        self.column = self.column + 1;
    }

    fn print_array (&self, name : &str, function : &Fn(&Accumulator) -> f64) {
       print! ("{}: ", name);
       for s in self.columns.iter() {
       	   print! ("\t{:.2}", function(s));
       }
       println! ("");
    }

    pub fn print_tsv (&self)
    {
        print! ("Name: ", );
        for s in self.headers.iter() {
       	   print! ("\t{:.2}", s);
        }
        println! ("");
        self.print_array ("Rows",  &| s| s.count () as f64 );
    	self.print_array ("Sum",   &| s| s.sum () );
    	self.print_array ("Min",   &| s| s.min () );
        self.print_array ("Max",   &| s| s.max () );
	self.print_array ("Range", &| s| s.range () );
    	self.print_array ("Mean",  &| s| s.mean () );
    	self.print_array ("SD",    &| s| s.sd () );
    	self.print_array ("Skew",  &| s| s.skew () );
    	self.print_array ("Kurt",  &| s| s.kurtosis () );
    	self.print_array ("xKurt",  &| s| s.excess_kurtosis () );
    	self.print_array ("CV",    &| s| s.coefficient_of_variation() );
    }
}

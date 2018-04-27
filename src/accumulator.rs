use std::f64;

/// An accumulator adds up a sequence of numerical observations and providing
/// a set of summary statistics for those numbers.

#[derive(Copy, Clone)]
pub struct Accumulator {
    pub count: i64,
    pub min:   f64,
    pub max:   f64,
    pub sum:   Sum,
    pub sum2:  Sum,
    pub sum3:  Sum,
    pub sum4:  Sum
}

#[derive(Copy, Clone)]
pub struct Sum {
    pub sum:   f64,
    pub compensation:   f64
}

impl Sum {
    pub fn zero () -> Sum {
        return Sum { compensation: 0., sum: 0. }
    }
    /// See https://en.wikipedia.org/wiki/Kahan_summation_algorithm
    fn add (&mut self, input : f64)  {
        let y = input - self.compensation; // Please see https://en.wikipedia.org/wiki/Kahan_summation_algorithm
        let t = self.sum + y;              // If sum is big but y is small then the low-order bits of y will be lost.
        self.compensation = (t - self.sum) - y; // (t - sum) eliminates the high order part of y and subtracting y recovers the low order part of y.
        self.sum = t;                           //  Next time, the lost low order part of y will be compensated for by adding to the input.
    }
    fn as_f64 (self) -> f64 {
        return self.sum;
    }
}
impl Accumulator {
    pub fn new () -> Accumulator {
        return Accumulator { count: 0, min: f64::INFINITY, max: f64::NEG_INFINITY, sum: Sum::zero(), sum2: Sum::zero(), sum3: Sum::zero(), sum4: Sum::zero()}
    }
    pub fn update (&mut self, input: f64) {
    	self.count = self.count + 1;
        self.min   = if self.min < input { self.min } else { input };
        self.max   = if self.max > input { self.max } else { input };
        self.sum.add (input);
        let square = input * input;
        self.sum2.add (square);
        self.sum3.add (square*input);
        self.sum4.add(square*square);
    }
    pub fn count (&self) -> i64 {
        return self.count;
    }
    pub fn sum (&self) -> f64 {
        return self.sum.as_f64();
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
        return self.sum.as_f64() / self.count as f64;
    }
    fn unscaled_variance(&self) -> f64 {
        let sum = self.sum.as_f64();
        let sum2 = self.sum2.as_f64();
        return sum2 - sum * sum / self.count as f64;
    }
    /// Sample variance
    pub fn variance(&self) -> f64 {
        return self.unscaled_variance() / (self.count-1) as f64;
    }
    pub fn population_variance(&self) -> f64 {
        return self.unscaled_variance() / self.count as f64;
    }
    /// Sample standard deviation
    pub fn sd(&self) -> f64 {
        return self.variance().sqrt();
    }
    /// See [Skewness](https://en.wikipedia.org/wiki/Skewness)
    pub fn skew(&self) -> f64 {
	let nn     = self.count as f64;   // Population nn, Sample would be: nn -1;
 	let mean   = self.mean ();
 	let sd     = self.variance ().sqrt ();
 	let sd3    = sd*sd*sd;
	let scale  = nn /(nn-1.)/(nn-2.);
	return (self.sum3.as_f64() - 3.0*mean*self.sum2.as_f64() + 2.0*nn*mean*mean*mean) / sd3 * scale;
    }
    /// See https://en.wikipedia.org/wiki/Kurtosis
    pub fn kurtosis(&self) -> f64 {
	let nn     = self.count as f64;   // Population nn, Sample would be: nn -1;
	let mean   = self.mean ();
	let var    = self.variance ();
	let mean2  = mean*mean;
	let mean4  = mean2*mean2;
	let scale  = nn*(nn+1.)/(nn-1.)/(nn-2.)/(nn-3.);
	let offset = 3.*(nn-1.)*(nn-1.)/(nn-2.)/(nn-3.);
	return (self.sum4.as_f64() - 4.0*mean*self.sum3.as_f64() + 6.0*self.sum2.as_f64()*mean2 - 3.*nn*mean4) / (var*var) * scale - offset;
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
       print! ("{}", name);
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


#[cfg(test)]
mod tests {

    use accumulator::Accumulator;
    use accumulator::Sum;

    #[test]
    fn sum_should_add_up () {
        let mut value = Sum::zero ();
        assert_eq!(0., value.as_f64());
        value.add(1.);
        assert_eq!(1., value.as_f64());
        value.add(-2.5);
        assert_eq!(-1.5, value.as_f64());
        value.add(8.5);
        assert_eq!(7., value.as_f64());
        value.add(101.1);
        assert_eq!(108.1, value.as_f64());
    }



    ///  This macro is for checking that two floating point numbers have the same value or both are NaN.
    ///  since Nan!=Nan by definition, one cannot just rely on assert_eq!

    macro_rules! assert_eq_or_nan {
        ($left:expr, $right:expr) => ({
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        if !((*left_val).is_nan() && (*right_val).is_nan()) {
                            panic!(r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#, left_val, right_val)
                        }
                    }
                }
            }
        });
    }

    ///  This macro tests for data sets with only one observation.
    
    macro_rules! accumulator_test_for_1_input {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let expected_sum = $value;
	            let mut a = Accumulator::new ();
	            a.update(expected_sum);
	            assert_eq!(a.count(), 1);
	            assert_eq!(a.sum(), expected_sum);
	            assert_eq!(a.mean(), expected_sum);
	            assert!(a.variance().is_nan());
	            assert!(a.skew().is_nan());
	            assert!(a.kurtosis().is_nan());

	            assert!(a.sd().is_nan());
	            assert!(a.coefficient_of_variation().is_nan());
	            assert!(a.excess_kurtosis().is_nan());
                }
            )*
        }
    }

    accumulator_test_for_1_input! {
        test_accumulator_after_1_entry_0: 0.0,
        test_accumulator_after_1_entry_1: 1.0,
        test_accumulator_after_1_entry_2: 2.0,
        test_accumulator_after_1_entry_3: -1.0,
    }

    ///  This macro tests for data sets with two observations.

    macro_rules! accumulator_test_for_2_inputs {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input1,input2,expected_variance) = $value;
	            let sum = input1+input2;
	            let mut a = Accumulator::new ();
	            a.update(input1);
	            a.update(input2);
	            assert_eq!(a.count(), 2);
	            assert_eq!(a.sum(), sum);
	            assert_eq!(a.mean(), sum/2.0);
	            assert_eq!(expected_variance, a.variance(), "Variance");
	            assert!(a.skew().is_nan());
	            assert!(a.kurtosis().is_nan() || a.kurtosis().is_infinite());

	            assert_eq!(a.sd(), a.variance().sqrt(), "SD");
	            assert_eq_or_nan!(a.coefficient_of_variation(), a.sd () / a.mean ());
	            assert!(a.excess_kurtosis().is_nan() || a.excess_kurtosis().is_infinite());
                }
            )*
        }
    }
    accumulator_test_for_2_inputs! {
        test_accumulator_after_2_inputs_0: (0.0, 0.0, 0.0),
        test_accumulator_after_2_inputs_1: (2.0, 0.0, 2.0),
        test_accumulator_after_2_inputs_2: (2.0, 1.0, 0.5),
    }

    ///  This macro tests for data sets with three observations.

    macro_rules! accumulator_test_for_3_inputs {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input1,input2,input3,expected_variance, expected_skew) = $value;
	            let sum = input1+input2+input3;
	            let mut a = Accumulator::new ();
	            a.update(input1);
	            a.update(input2);
	            a.update(input3);
	            assert_eq!(a.count(), 3);
	            assert_eq!(a.sum(), sum);
	            assert_eq!(a.mean(), sum/3.0);
	            assert_eq!(expected_variance, a.variance(), "Variance");
	            assert_eq!(expected_skew,a.skew());
	            assert!(a.kurtosis().is_nan() || a.kurtosis().is_infinite());

	            assert_eq!(a.sd(), a.variance().sqrt(), "SD");
	            assert_eq_or_nan!(a.coefficient_of_variation(), a.sd () / a.mean ());
	            assert!(a.excess_kurtosis().is_nan() || a.excess_kurtosis().is_infinite());
                }
            )*
        }
    }
    accumulator_test_for_3_inputs! {
        test_accumulator_after_3_inputs_0: (0.0, 2.0, 1.0, 1.0, 0.0),
        test_accumulator_after_3_inputs_1: (1.0, 4.0, 1.0, 3.0, 1.7320508075688776),
        test_accumulator_after_3_inputs_2: (3.0, 0.0, 0.0, 3.0, 1.7320508075688776),
    }

    #[test]
    fn test_accumulator_after_4_inputs ()
    {
	let mut a = Accumulator::new ();
	a.update(1.0);
	a.update(1.0);
	a.update(1.0);
	a.update(0.0);
	assert_eq!(a.count(), 4);
	assert_eq!(a.sum(), 3.0);
	assert_eq!(a.mean(), 0.75);
	assert_eq!(a.variance(), 0.25);
	assert_eq!(a.skew(), -2.0);  // -0.5
	assert_eq!(a.kurtosis(), 4.0, "Kurt"); // -32.375

	assert_eq!(a.sd(), a.variance().sqrt());
	assert_eq!(a.coefficient_of_variation(), a.sd () / a.mean ());
	assert_eq!(a.kurtosis(), a.excess_kurtosis() + 3.0);
    }

    #[test]
    fn test_accumulator_after_4_inputs_2 ()
    {
	let mut a = Accumulator::new ();
	a.update(4.0);
	a.update(1.0);
	a.update(1.0);
	a.update(1.0);
	assert_eq!(a.count(), 4);
	assert_eq!(a.sum(), 7.0);
	assert_eq!(a.mean(), 1.75);
	assert_eq!(a.variance(), 2.25);
	assert_eq!(a.skew(), 2.0);  // 0.5
	assert_eq!(a.kurtosis(), 4.0, "Kurt"); // -10.97582304526749

	assert_eq!(a.sd(), a.variance().sqrt());
	assert_eq!(a.coefficient_of_variation(), a.sd () / a.mean ());
	assert_eq!(a.kurtosis(), a.excess_kurtosis() + 3.0);
    }
}

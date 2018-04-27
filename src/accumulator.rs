use std::f64;

/// An accumulator adds up a sequence of numerical observations and providing
/// a set of summary statistics for those numbers.

#[derive(Copy, Clone)]
pub struct Accumulator {
    pub count: u64,
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
    #[inline]
    pub fn count (&self) -> u64 {
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
        return self.unscaled_variance() / (self.count as f64 -1.);
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
        self.print_array ("VAR",    &| s| s.variance () );
        self.print_array ("PVAR",    &| s| s.population_variance () );
        self.print_array ("CV",    &| s| s.coefficient_of_variation() );
    	self.print_array ("Skew",  &| s| s.skew () );
    	self.print_array ("Kurt",  &| s| s.kurtosis () );
    	self.print_array ("xKurt",  &| s| s.excess_kurtosis () );
    }
}


#[cfg(test)]
mod tests {

    use accumulator::Accumulator;
    use accumulator::Sum;
    use std::f64::consts::PI;
    use std::f64::consts::E;
    use std::f64;
    use std::ops::Range;
    use accumulator::Accumulators;

    #[test]
    fn sum_should_add_up () {
        let mut value = Sum::zero();
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
    #[test]
    fn error_should_not_propogate_after_multiple_additions () {

        let mut value = Sum::zero();
        const TIMES : f64 = 9999.;
        const RANGE : Range<u32> = 0u32..(TIMES as u32);
        for _ in RANGE {
            value.add(PI);
        }
        assert_eq!(PI*TIMES, value.as_f64());
        for _ in RANGE {
            value.add(E);
        }
        assert_eq!(PI*TIMES+E*TIMES, value.as_f64());
        // assert_eq!((PI+E)*TIMES, value.as_f64());  // This fails 58592.88494600633 against 58592.884946006336
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

    #[test]
    fn test_accumulator_0_input () {
        let a = Accumulator::new ();
        assert_eq!(a.count(), 0);
        assert_eq!(a.sum(), 0.);
        assert_eq_or_nan!(f64::NAN, a.mean());
        assert_eq_or_nan!(f64::NAN, a.variance());
        assert_eq_or_nan!(f64::NAN,a.skew());
        assert_eq_or_nan!(f64::NAN,a.kurtosis());
        assert_eq_or_nan!(a.sd(), a.variance().sqrt());
        assert_eq_or_nan!(a.coefficient_of_variation(), a.sd () / a.mean ());
        assert_eq_or_nan!(a.kurtosis(), a.excess_kurtosis() + 3.0);
    }

    fn test_accumulator (a : Accumulator, expected_count : u64, expected_sum : f64, expected_variance : f64, expected_skew : f64, expected_kurtosis : f64) {
        assert_eq!(a.count(), expected_count as u64);
        assert_eq!(a.sum(), expected_sum );
        assert_eq!(a.mean(), expected_sum / expected_count as f64 , "Mean");
        assert_eq_or_nan!(expected_variance, a.variance());
        assert_eq_or_nan!(expected_skew,a.skew());
        assert_eq_or_nan!(expected_kurtosis,a.kurtosis());
        assert_eq_or_nan!(a.sd(), a.variance().sqrt());
        assert_eq_or_nan!(a.coefficient_of_variation(), a.sd () / a.mean ());
        assert_eq_or_nan!(a.kurtosis(), a.excess_kurtosis() + 3.0);
    }
    macro_rules! accumulator_test_for_multiple_inputs {
        ($($name:ident: $expected:expr;  $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (expected_variance, expected_skew, expected_kurtosis) = $expected;
    	            let mut a = Accumulator::new ();
                    let mut sum = 0.;
	                for input in $inputs.iter() {
	                   a.update(*input as f64);
	                   sum = sum + input;
                    }
                    test_accumulator (a, $inputs.len()  as u64, sum, expected_variance, expected_skew, expected_kurtosis);
                }
            )*
        }
    }

    accumulator_test_for_multiple_inputs! {
        test_accumulator_1_input_0: (f64::NAN, f64::NAN, f64::NAN); [0.0],
        test_accumulator_1_input_1: (f64::NAN, f64::NAN, f64::NAN); [1.0],
        test_accumulator_1_input_2: (f64::NAN, f64::NAN, f64::NAN); [2.0],
        test_accumulator_1_input_3: (f64::NAN, f64::NAN, f64::NAN); [-1.0],
        test_accumulator_2_inputs_0: (0.0, f64::NAN, f64::NAN);  [0.0, 0.0],
        test_accumulator_2_inputs_1: (2.0, f64::NAN, f64::NAN);  [0.0, 2.0],
        test_accumulator_2_inputs_2: (0.5, f64::NAN, f64::NAN);  [2.0, 1.0],
        test_accumulator_3_inputs_0: (0.0, f64::NAN, f64::NAN); [1.0, 1.0, 1.0],
        test_accumulator_3_inputs_1: (1.0, 0.0, f64::NAN); [0.0, 2.0, 1.0],
        test_accumulator_3_inputs_2: (3.0, 1.7320508075688776, f64::NAN); [1.0, 4.0, 1.0],
        test_accumulator_3_inputs_3: (3.0, 1.7320508075688776, f64::NAN); [3.0, 0.0, 0.0],
        test_accumulator_4_inputs_4: (0.25, -2.0, 4.0); [1.0, 1.0, 1.0, 0.0],
        test_accumulator_4_inputs_5: (2.25,  2.0, 4.0); [4.0, 1.0, 1.0, 1.0],
        test_accumulator_4_inputs_6: (2.0,  0.8838834764831842, -1.75); [1.0, 4.0, 1.0, 3.0, 1.0],  // TODO Check OpenOffice gave: 0.883883476483184
    }

    #[test]
    fn accumulators_should_read_and_sumarise_data () {
        let mut a = Accumulators::new();
        assert_eq!(0, a.column());
        assert_eq!(0, a.rows());
        assert_eq!(0, a.headers.len ());
        a.new_row();
        a.add_column_value("1");
        a.add_column_value("4");
        a.new_row();
        a.add_column_value("2");
        a.add_column_value("3");
        a.new_row();
        a.add_column_value("3");
        a.add_column_value("2");
        a.new_row();
        a.add_column_value("4");
        a.add_column_value("1");
        assert_eq!(2, a.column());
        assert_eq!(4, a.rows());
        assert_eq!(2, a.columns.len());
        assert_eq!(2, a.headers.len ());
        test_accumulator (a.columns[0], 4, 10., 1.6666666666666667, 0., -1.200000000000001);
        test_accumulator (a.columns[1], 4, 10., 1.6666666666666667, 0., -1.200000000000001);
    }
}

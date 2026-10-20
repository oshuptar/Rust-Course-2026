#[derive(Debug, Clone, Default)]
struct NumberWithUnit{
    unit: String, // the struct owns the String
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value : f64) -> Self {
        Self {value,  unit: String::from("")}
    }
    fn with_unit(value : f64, unit: String) -> Self {
        Self {value, unit} // notice that unit is moved here
    }

    fn with_unit_from(other: &Self, value: f64) -> Self {
        Self {value, unit: other.unit.clone()}
        // the shorthand value only works because a variable has exactly the same name as the field. Rust reads it as value: value. An expression like other.unit.clone() isn’t a variable name, so Rust doesn’t know which field it should go into. You need to name the field
    }
    fn add(&self, other: &Self) -> Self {
        if self.unit == other.unit {
            NumberWithUnit::with_unit_from(self, self.value + other.value)
        } else {
            panic!("The units do not match");
        }
    }
    fn mul(&self, other: &Self) -> Self {
        Self::with_unit(self.value * other.value, if !self.unit.is_empty() {format!("{} * {}", self.unit, other.unit)} else {other.unit.clone()})
    }
    fn div(&self, other: &Self) -> Self {
        Self::with_unit(self.value / other.value, if !self.unit.is_empty() {format!("{} / {}", self.unit, other.unit)} else {other.unit.clone()})
    }
    fn add_in_place(&mut self, other: &Self) {
        if self.unit == other.unit {
            self.value += other.value;
        }
    }
    fn mul_in_place(&mut self, other: &Self) {
        self.value  *= other.value;
        self.unit = if !self.unit.is_empty() { format!("{} * {}", self.unit, other.unit) } else {other.unit.clone()};
    }
    fn div_in_place(&mut self, other: &Self) {
        self.value /= other.value;
        self.unit = if !self.unit.is_empty(){ format!("{} / {}", self.unit, other.unit) } else {other.unit.clone()};
    }

    fn mul_vals(elements: &[NumberWithUnit]) -> NumberWithUnit {
        let mut result = NumberWithUnit::unitless(1.0);
        for element in elements {
            result.mul_in_place(element);
        }
        result
    }
    fn mul_vals_vec(elements: Vec<NumberWithUnit>) -> NumberWithUnit {
        let mut result = NumberWithUnit::unitless(1.0);
        for element in elements {
            result.mul_in_place(&element);
        }
        result
    }
}

struct DoubleString(String, String);

impl DoubleString {
    // &str is a type of string slice
    fn from_strs(str_1 : &str, str_2 : &str) -> Self {
        DoubleString(str_1.to_string(), str_2.to_string()) // remember about implicit dereferencing
    }

    // This would not work for string slices and for string literals
    fn from_strings(str_1: &String, str_2: &String) -> Self {
        DoubleString(str_1.clone(), str_2.clone())
    }
    fn show(&self) -> String {
        format!("({}, {})", self.0, self.1) // positional parameter access
    }
}

fn main() {
    let n1 = NumberWithUnit::unitless(10.0);
    let n2 = NumberWithUnit::with_unit(15.0, "kg".to_string());
    let n3 = NumberWithUnit::with_unit_from(&n2, 10.0);
    let n4 = NumberWithUnit::unitless(20.0);
    println!("{:?}", n1);
    println!("{:?}", n2);
    println!("{:?}", n3);
    println!("{:?}", n4);

    let n5 = n1.add(&n4);
    println!("{:?}", n5);
    let n6 = n1.mul(&n4);
    println!("{:?}", n6);
    let mut n7 = n1.div(&n4);
    println!("{:?}", n7);
    n7.add_in_place(&n5);
    println!("{:?}", n7);

    let mut distance = NumberWithUnit { unit: "km".to_string(), value: 100.0 };
    let time_window = NumberWithUnit { unit: "hr".to_string(), value: 2.0 };
    let average_speed = distance.div(&time_window);
    println!("Average speed {:?}", average_speed);
    distance.div_in_place(&time_window);
    let average_speed = distance;
    println!("Average speed {:?}", average_speed);

    let slice: [NumberWithUnit; 3] = [
        NumberWithUnit::with_unit(10.0, "kg".to_string()),
        NumberWithUnit::with_unit(2.0, "kg".to_string()),
        NumberWithUnit::with_unit(1.0, "kg".to_string()),
    ];
    println!("{:?}", NumberWithUnit::mul_vals(&slice));
    println!("{:?}", NumberWithUnit::mul_vals(&slice[0..=2]));

    let vec: Vec<NumberWithUnit> = vec![
        NumberWithUnit::with_unit(10.0, "kg".to_string()),
        NumberWithUnit::with_unit(2.0, "kg".to_string()),
        NumberWithUnit::with_unit(1.0, "kg".to_string()),
    ];
    println!("{:?}", NumberWithUnit::mul_vals_vec(vec.clone()));
    println!("{:?}", NumberWithUnit::mul_vals_vec(vec)); //value was moved, so here it is used - we have to use clone instead (deep copy)

    let string: String = String::from("hello world");
    let str_slice: &str = &string[0..5];
    let double_string_1 = DoubleString::from_strs(&string, str_slice); // &String is automatically converted to &str
    let double_string_2 = DoubleString::from_strings(&string, &str_slice.to_string()); // the argument &String is expected, found &str
    println!("{}", double_string_1.show());
    println!("{}", double_string_2.show());
}


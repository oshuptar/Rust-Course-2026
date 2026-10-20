
#[derive(Debug, Clone, Default)]
struct NumberWithUnit{
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value : f64) -> Self {
        Self {value,  unit: String::from("")}
    }

    fn with_unit(value : f64, unit: String) -> Self {
        Self {value, unit} // notice that the value is moved here
    }

    fn with_unit_from(other: &Self, value: f64) -> Self {
        Self {value, unit: other.unit.clone()}
        // the shorthand value only works because a variable has exactly the same name as the field. Rust reads it as value: value. An expression like other.unit.clone() isn’t a variable name, so Rust doesn’t know which field it should go into. You need to name the field
    }
}

fn main() {
    let n1 = NumberWithUnit::unitless(10.0);
    let n2 = NumberWithUnit::with_unit(15.0, "kg".to_string());
    let n3 = NumberWithUnit::with_unit_from(&n2, 10.0);
    println!("{:?}", n1);
    println!("{:?}", n2);
    println!("{:?}", n3);
}

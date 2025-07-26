use serde_json::{
    Number,
    Value,
};

fn round_float(value: f64, decimal_places: i32) -> f64 {
    let factor = 10.0f64.powi(decimal_places);

    (value * factor).round() / factor
}

pub fn round_floats(value: &mut Value, decimal_places: i32) {
    match value {
        Value::Number(number) => {
            if let Some(float_number) = number.as_f64() {
                *number = Number::from_f64(round_float(float_number, decimal_places))
                    .expect("Should not be inf or NaN");
            }
        }
        Value::Array(values) => {
            for value in values {
                round_floats(value, decimal_places);
            }
        }
        Value::Object(map) => {
            for value in map.values_mut() {
                round_floats(value, decimal_places);
            }
        }
        _ => {}
    };
}

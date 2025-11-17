fn select_validator(validators: Vec<i32>) -> i32 {
    // Create a random value between 0 and 1 and select the validador with the closest value.
    // Given [10, 20, 70] a random value of 0.15 should match 20 validator.
    let rand = rand::random_range(0..100);
    let mut index = 0;
    let mut percent = 0;
    for (i, validator) in validators.iter().enumerate() {
        if rand < percent + validator {
            index = i;
            break;
        }
        percent += validator;
    }

    validators[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_validator_probability_is_close_to_distribution() {
        let validators = vec![10, 20, 70];
        let mut selected_validators: Vec<i32> = vec![];

        for _ in 0..1000 {
            selected_validators.push(select_validator(validators.clone()));
        }

        let selected_10 = selected_validators
            .iter()
            .filter(|&v| *v == validators[0])
            .count();

        let selected_10_percent = selected_10 as f64 / 1000.0;

        let selected_20 = selected_validators
            .iter()
            .filter(|&v| *v == validators[1])
            .count();

        let selected_20_percent = selected_20 as f64 / 1000.0;

        let selected_30 = selected_validators
            .iter()
            .filter(|&v| *v == validators[2])
            .count();

        let selected_30_percent = selected_30 as f64 / 1000.0;

        assert!(selected_10_percent > 0.05 && selected_10_percent < 0.15);
        assert!(selected_20_percent > 0.15 && selected_20_percent < 0.25);
        assert!(selected_30_percent > 0.25 && selected_30_percent < 1.0);
    }
}

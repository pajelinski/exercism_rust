pub fn raindrops(number: i32) -> String {
    let is_number_divisble_by_3 = |x: i32| { x % 3 == 0};
    let is_number_divisble_by_5 = |x: i32| { x % 5 == 0};
    let is_number_divisble_by_7 = |x: i32| { x % 7 == 0};

    let mut raindrops_list = Vec::with_capacity(3);

    if is_number_divisble_by_3(number) {
        raindrops_list.push("Pling");
    }
    if is_number_divisble_by_5(number) {
        raindrops_list.push("Plang");
    }
    if is_number_divisble_by_7(number) {
        raindrops_list.push("Plong");
    }

    match raindrops_list.is_empty(){
        true => number.to_string(),
        false => raindrops_list.iter().fold(String::new(), |mut sum, &val| {sum += val; sum})
    }
}

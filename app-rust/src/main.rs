use fdp_definition::apps::app_1::broadcasted_events::RandomNumber;

fn main() {
    let random_number = RandomNumber { value: 42 };

    println!("{}", serde_json::to_string(&random_number).unwrap());
}

mod creators;

pub fn create_boilerplate(boilerplate_type: &str, boilerplate_location: &str, project_name: &str) {
    println!(
        "new boilderplate: {}, at {}, called {}",
        boilerplate_type, boilerplate_location, project_name
    );

    creators::place_me_at(boilerplate_location);

    match boilerplate_type {
        "express" => creators::create_express_boilerplate(),
        "react" => creators::create_react_boilerplate(),
        "react-ts" => creators::create_react_ts_boilerplate(),
        _ => println!("Ah?.."),
    }
}

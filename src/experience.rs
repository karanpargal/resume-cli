use serde_json::{Result, Value};

pub fn show_experience(json_data: &str) -> Result<()>{
    let v: Value = serde_json::from_str(json_data)?;
    let mut i = 0;
    loop {
        let exp = &v["Experiences"][i];
        if exp.is_null() {
            break;
        }
        println!("");
        println!("Position : {}" , exp["Position"].as_str().unwrap());
        println!("Company : {}" , exp["Company"].as_str().unwrap());
        println!("Location : {}" , exp["Location"].as_str().unwrap());
        println!("Description : {}" , exp["Description"].as_str().unwrap());
        i += 1;
    }

    Ok(())
}

use isahc::prelude::*;

pub async fn set_fan(fan_ressource: &str, state: &str) -> Result<(), isahc::Error>{
    let mut response = isahc::get(&format!("{}/{}", fan_ressource, state))?;
    print!("{}", response.text()?);
    Ok(())
}
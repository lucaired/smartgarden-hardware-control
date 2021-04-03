use reqwest::ClientBuilder;

pub fn set_state_to_fan(state: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: get ressource from function
    let request_url = format!("http://localhost:8100/fan/{}", state);

    let client = ClientBuilder::new().build()?;
    let _response = client.get(&request_url).send()?;
    Ok(())
}

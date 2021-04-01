use reqwest::{ Client };

pub async fn set_fan(client: &Client, fan_ressource: &str, state: &str) -> Result<(), Box<dyn std::error::Error>>{
    client.get(format!("{}/{}", fan_ressource, state)).await?;
    Ok(())
}
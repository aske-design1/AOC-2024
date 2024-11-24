use crate::error_handling::Error;


pub async fn get_input(day_num: u8) -> Result<String, Error>  {
    let path = std::env::current_dir()?
    .join(std::path::PathBuf::from("header_data/session_id.txt"));
    
    let session_id = std::fs::read_to_string(path)?;
    let url = format!("https://adventofcode.com/2024/day/{day_num}/input");
    
    let response = 
    reqwest::Client::new()
    .get(url)
    .header(reqwest::header::COOKIE, session_id)
    .send()
    .await;

    let Ok(response) = response
    else { return Err(Error::NetworkError("Failure parsing future".to_string())) };

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err(Error::NetworkError("Bad request: 400".to_string()))
    }

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Error::NetworkError("Page not found: 404".to_string()))
    }

    match response.text().await {
        Ok(failure) if failure.len() == 210 && &failure[..6] == "Please" 
        => Err(Error::NetworkError("Content not released".to_string())),

        Ok(input) => Ok(input),
        Err(_) => Err(Error::NetworkError("Getting input failed".to_string()))
    }
}
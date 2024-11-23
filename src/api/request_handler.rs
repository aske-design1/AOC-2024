
pub async fn get_input(day_num: u8) -> Result<String, String>  {
    let Ok(dir_path) = std::env::current_dir()
    else { return Err("Couldn't create path to directory".to_string()) };
    let path = dir_path.join(std::path::PathBuf::from("input_files/session_id.txt"));
    
    let Ok(session_id) = std::fs::read_to_string(path) 
    else { return Err(format!("Failed to get session id")) };
    let url = format!("https://adventofcode.com/2024/day/{day_num}/input");
    
    let response = 
    reqwest::Client::new()
    .get(url)
    .header(reqwest::header::COOKIE, session_id)
    .send()
    .await;

    let Ok(response) = response 
    else { return Err("Failure parsing future".to_string()) };

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        return Err("Session id invalid".to_string())
    }

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err("Page not found".to_string())
    }

    match response.text().await {
        Ok(failure) 
        if failure.len() == 210 && &failure[..6] == "Please" => Err("Day not released yet".to_string()),
        Ok(input) => Ok(input),
        Err(_) => Err("Getting input failed".to_string()),
    }
}
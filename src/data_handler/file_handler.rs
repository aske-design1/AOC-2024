use crate::error_handling::Error;
use std::path::PathBuf;

pub fn create_path(dir: &str, file_type: &str, day_num: u8) -> Result<PathBuf, Error> {
    let dir_path = std::env::current_dir()?;

    let formatted_day = match day_num {
        1..=9 => format!("day0{day_num}.{file_type}"),
        10..=25 => format!("day{day_num}.{file_type}"),
        invalid => return Err(Error::InvalidDayNumber(invalid))
    };

    Ok(dir_path.join(
        PathBuf::from(format!("{dir}\\{formatted_day}"))
    ))
}

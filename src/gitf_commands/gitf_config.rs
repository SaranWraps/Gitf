use serde::{Deserialize, Serialize};
use std::{fs::{self, OpenOptions}, io::Error, io::ErrorKind, io::{Read, self}, path::PathBuf};
use dirs;

#[derive(Serialize, Deserialize)]
pub struct FavouritesList{
    pub favourites: Vec<String>
}

pub fn save_config(favourites: FavouritesList) -> io::Result<()>{
    let app_config_dir = get_config_path()?;
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(&app_config_dir)?;
    return match serde_json::to_writer(file, &favourites){
        Ok(_) => Ok(()),
        Err(err) => Err(Error::new(ErrorKind::Other, err))
    };
}


pub fn get_config() -> std::io::Result<FavouritesList> {

    let app_config_path = get_config_path()?; 

    let file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&app_config_path);

    let mut file = file_result?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let mut faves = FavouritesList{
        favourites: Vec::new()
    };

    if file_content.len() > 0{
        faves = match serde_json::from_str(&file_content){
            Ok(favs) => favs,
            Err(err) => return Err(Error::new(ErrorKind::Other, format!("Could not convert from json: {}", err.to_string())))
        };
    }

    Ok(faves)
}


fn get_config_path() -> Result<PathBuf, io::Error>{
    let home_dir = dirs::home_dir().unwrap();
    let final_dir = home_dir.join("Library/Application Support").join("gitf");
    if !final_dir.exists(){
        fs::create_dir_all(&final_dir)?;
    }

    Ok(final_dir.join(".gitf"))

}

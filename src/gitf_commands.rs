mod gitf_config;

pub fn add(branch_name: &str) -> std::io::Result<()>{ 
    let mut config = gitf_config::get_config()?;

    if config.favourites.contains(&branch_name.to_string()){
        return Ok(());
    }

    config.favourites.push(branch_name.to_string());

    gitf_config::save_config(config)?;

    Ok(())
}

pub fn remove(branch_name: &str) -> std::io::Result<()>{
    let mut config = gitf_config::get_config()?;

    for(index, value) in config.favourites.iter().enumerate(){
        if branch_name != value{
            continue;
        }

        config.favourites.remove(index);
        break;
    }

    return gitf_config::save_config(config);
}

pub fn view_branches() -> std::io::Result<()>{
    let config = gitf_config::get_config()?;
    for fav in config.favourites{
        println!("{}", fav);
    }

    Ok(())
}

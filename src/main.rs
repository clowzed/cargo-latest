use colored::*;
use structopt::StructOpt;

macro_rules! error 
{
    ($a:expr) => 
    {{
        println!("{} {}", String::from("[ERROR]:").red().bold(), $a);
        std::process::exit(1);
    }};
}


#[derive(StructOpt, Debug)]
#[structopt(name = "Geting last stable version of a given crate")]
struct Settings 
{
    #[structopt(short, long)]
    crate_name: String,
    
    #[structopt(short, long)]
    latest    : Option<bool>,
}


custom_error::custom_error! 
{
ResolvingError
    CrateNotFound      = "Crate with this name is not found",
    RequestError       = "Check internet connection",
    VersionsParseError = "Failed to parse versions for crate. Check crate name!"
}


#[derive(Debug, serde::Deserialize)]
struct Version 
{
    num    : String,
    yanked : bool,
}


#[derive(Debug, serde::Deserialize)]
struct Crate 
{
    versions: Vec<Version>,
}



async fn check_if_crate_exist(name: &str) -> Result<bool, ResolvingError> 
{
    let crate_url = format!("http://crates.io/api/v1/crates/{}", name);

    let client = reqwest::Client::new();

    return match client.get(crate_url)
                .header(reqwest::header::USER_AGENT, "get last version cli")
                .send()
                .await
    {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Err(ResolvingError::RequestError),
    };
}





async fn find_versions_of_crate(crate_name: &str) -> Result<Vec<Version>, ResolvingError> 
{
    let crate_url = format!("http://crates.io/api/v1/crates/{}/versions", crate_name);
    
    let client = reqwest::Client::new();

    return match client.get(crate_url)
                .header(reqwest::header::USER_AGENT, "get last version cli")
                .send()
                .await
    {
        Ok(response) => 
        {
            match response.json::<Crate>().await
            {
                Ok(parsed_crate) => Ok(parsed_crate.versions),
                Err(_) => Err(ResolvingError::VersionsParseError)
            }
        },
        Err(_) => Err(ResolvingError::RequestError)
    };
}




#[tokio::main]
async fn main() {
    let settings = Settings::from_args();

    match check_if_crate_exist(&settings.crate_name).await 
    {
        Ok(crate_exists) => match crate_exists 
        {
            false => 
            {
                error!(format!("Crate with name '{}' does not exist", settings.crate_name));
            },
            true => 
            {
                match find_versions_of_crate(&settings.crate_name).await
                {
                    Ok(versions) => 
                    {
                        
                        if settings.latest.unwrap_or(true)
                        {
                            let versions = versions.iter().filter(|version| !version.yanked).collect::<Vec<_>>();
                            let version = versions.first();
                            
                            match version
                            {
                                Some(version) =>
                                {
                                    print!("{} = \"{}\"", &settings.crate_name, version.num);
                                    if   version.yanked {  println!("{}{}", " ".repeat(20 - version.num.len()) ,String::from("yanked").red()); }
                                    else                {  println!()                                  }
                                },
                                None => 
                                {
                                    error!("No stable version was found for crate!");
                                }
                            }
                            return;
                            
                        }
                        
                        for (ind, version) in versions.iter().rev().enumerate()
                        {
                            print!("[{:3}] {} = \"{}\"", ind + 1, &settings.crate_name, version.num);
                            if   version.yanked {  println!("{}{}", " ".repeat(20 - version.num.len()) ,String::from("yanked").red()); }
                            else                {  println!()                                  }
                        }
                    },
                    Err(e) => 
                    {
                        error!(e.to_string());
                    }
                }
            }
        },
        Err(e) => 
        {
            error!(e.to_string());
        }
    }
}

mod articles;
use crate::articles::nytimes::{NYTInterface, NYTSectionEnum, Article};

use reqwest::{Error};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let API_KEY: String = fs::read_to_string("src/secret.txt")
        .expect("Should have been able to read the file");

    let APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"));

    let nyt_api: NYTInterface = NYTInterface::new(&APP_USER_AGENT, &API_KEY);

    let res_top: Vec<Article> = nyt_api.get_top_stories(NYTSectionEnum::Technology).await?;
    
    println!("NYT Top Articles ------------------------------");
    res_top
        .into_iter()
        .for_each(
            |article| 
            println!("{:?} {:?}", article.published_at, article.title)
        );

    
    let res_search: Vec<Article> = nyt_api.get_article_search(String::from("tsla"), String::from("")).await?;
    
    println!("NYT Article Search -----------------------------");
    res_search
        .into_iter()
        .for_each(
            |article| 
            println!("{:?} {:?}", article.published_at, article.title)
        );

    Ok(())
}
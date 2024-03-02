use serde::Deserialize;
use reqwest::{Error};

#[derive(Deserialize, Debug)]
struct NYTArticle {
    section: String,
    subsection: String,
    title: String,
    r#abstract: String,
    url: String,
    item_type: String,
    updated_date: String,
    published_date: String,
    created_date: String,
}

#[derive(Deserialize, Debug)]
struct NYTSection {
    status: String,
    copyright: String,
    section: String,
    last_updated: String,
    num_results: usize,
    results: Vec<NYTArticle>
}

enum NYTSectionEnum {
    Arts, 
    Automobiles, 
    BooksReview, 
    Business, 
    Fashion, 
    Food, 
    Health, 
    Home, 
    Insider, 
    Magazine, 
    Movies, 
    NYRegion, 
    Obituaries, 
    Opinion, 
    Politics, 
    RealEstate, 
    Science, 
    Sports, 
    SundayReview, 
    Technology, 
    Theater, 
    TMagazine, 
    Travel, 
    Upshot, 
    UnitedStates, 
    World,
}
impl NYTSectionEnum {
    fn as_str(&self) -> &str {
        match self {
            NYTSectionEnum::Arts => "arts", 
            NYTSectionEnum::Automobiles => "automobiles", 
            NYTSectionEnum::BooksReview => "books_review", 
            NYTSectionEnum::Business => "business", 
            NYTSectionEnum::Fashion => "fashion", 
            NYTSectionEnum::Food => "food", 
            NYTSectionEnum::Health => "health", 
            NYTSectionEnum::Home => "home", 
            NYTSectionEnum::Insider => "insider", 
            NYTSectionEnum::Magazine => "magazine", 
            NYTSectionEnum::Movies => "movies", 
            NYTSectionEnum::NYRegion => "nyregion", 
            NYTSectionEnum::Obituaries => "obituaries", 
            NYTSectionEnum::Opinion => "opinion", 
            NYTSectionEnum::Politics => "politics", 
            NYTSectionEnum::RealEstate => "realestate", 
            NYTSectionEnum::Science => "science", 
            NYTSectionEnum::Sports => "sports", 
            NYTSectionEnum::SundayReview => "sundayreview", 
            NYTSectionEnum::Technology => "technology", 
            NYTSectionEnum::Theater => "theater", 
            NYTSectionEnum::TMagazine => "t_magazine", 
            NYTSectionEnum::Travel => "travel", 
            NYTSectionEnum::Upshot => "upshot", 
            NYTSectionEnum::UnitedStates => "us", 
            NYTSectionEnum::World => "world",
        }
    }
}

struct NYTInterface {
    _client: reqwest::Client,
    _api_key: String,
}

impl NYTInterface {
    pub fn new(APP_USER_AGENT: &str, API_KEY: &str) -> NYTInterface {
        let APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"));
        
        let api_key = API_KEY;
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("failed to build NYTInterface");
        
        NYTInterface { 
            _client: client,
            _api_key: api_key.to_string()
        }
        
    }   
    pub async fn get_top_stories(&self, section: NYTSectionEnum) -> Option<NYTSection> {
        // https://developer.nytimes.com/docs/top-stories-product/1/overview

        // need to see if the api-key can be passed through the AUTHORIZATION header rather than as an insecure url parameter
        let request_url = format!("https://api.nytimes.com/svc/topstories/v2/{param_section}.json?api-key={param_api_key}",
                              param_section = section.as_str(),
                              param_api_key = self._api_key,
                            );

        // println!("NYTInterface::get_top_stories {}", request_url);

        let response = self._client
            .get(&request_url)
            .send()
            .await.ok()?;

        // println!("NYTInterface::get_top_story() -> {:?}", response);

        let data: NYTSection = response.json()
            .await.ok()?;

        Some(data)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    static API_KEY: &str = "2KHS8vkBVrW7Gk70UAjMwZw4JMOrgwUu";
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"));

    let NYT_API: NYTInterface = NYTInterface::new(APP_USER_AGENT, API_KEY);

    match NYT_API.get_top_stories(NYTSectionEnum::Technology).await {
        Some(res) => {
            res.results
                .into_iter()
                .for_each(
                    |article| 
                    println!("{:?} {:?}", article.published_date, article.title)
                );
        },
        None => {},
    }
    
    Ok(())
}
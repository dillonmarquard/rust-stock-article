use serde::Deserialize;
use reqwest::Error;

#[derive(Debug)]
// interface for pulling article data from any website
pub struct Article {
    pub url: String,
    pub title: String, 
    pub text: String,
    pub published_at: String,
}

#[derive(Deserialize, Debug)]
struct NYTTopStoryArticle {
    url: String,
    title: String,
    published_date: String,
    section: String,
    subsection: String,
    r#abstract: String,
}

#[derive(Deserialize, Debug)]
struct NYTTopStoryResponse {
    status: String,
    copyright: String,
    results: Vec<NYTTopStoryArticle>
}

#[derive(Deserialize, Debug)]
struct NYTSearchArticle {
    web_url: String,
    source: String,
    pub_date: String,
    snippet: String,
    r#abstract: String,
    lead_paragraph: String,
}

#[derive(Deserialize, Debug)]
struct NYTArticleSearchObj {
    docs: Vec<NYTSearchArticle>
}

#[derive(Deserialize, Debug)]
struct NYTArticleSearchResponse {
    status: String,
    copyright: String,
    response: NYTArticleSearchObj
}

pub enum NYTSectionEnum {
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

pub struct NYTInterface {
    _client: reqwest::Client,
    _api_key: String,
}

impl NYTInterface {
    pub fn new(app_user_agent: &str, api_key: &str) -> NYTInterface {
        
        let client = reqwest::Client::builder()
            .user_agent(app_user_agent)
            .build()
            .expect("failed to build NYTInterface");
        
        NYTInterface { 
            _client: client,
            _api_key: api_key.to_string()
        }
        
    }   
    pub async fn get_top_stories(&self, section: NYTSectionEnum) -> Result<Vec<Article>, Error> {
        // https://developer.nytimes.com/docs/top-stories-product/1/overview

        // need to see if the api-key can be passed through the AUTHORIZATION header rather than as an insecure url parameter
        let request_url = format!("https://api.nytimes.com/svc/topstories/v2/{param_section}.json?api-key={param_api_key}",
                                param_section = section.as_str(),
                                param_api_key = self._api_key,
                            );

        dbg!("NYTInterface::get_top_stories {}", &request_url);

        let response = self._client.get(&request_url).send().await?;

        dbg!("NYTInterface::get_top_story() -> {:?}", &response);

        let data: NYTTopStoryResponse = response.json().await?;
        let mut res: Vec<Article> = Vec::new();

        data.results
            .into_iter()
            .for_each(
                |article| 
                res.push(Article {
                    url: article.url,
                    title: article.title, 
                    text: article.r#abstract,
                    published_at: article.published_date,
                })
            );

        Ok(res)
    }
    pub async fn get_article_search(&self, query: String, filter: String) -> Result<Vec<Article>, Error> {
        // https://developer.nytimes.com/docs/articlesearch-product/1/overview

        let request_url = format!("https://api.nytimes.com/svc/search/v2/articlesearch.json?query={param_query}&sort=newest&api-key={param_api_key}",
                                    param_query = query,
                                    param_api_key = self._api_key,
                                );

        let response = self._client.get(&request_url).send().await?;

        let data: NYTArticleSearchResponse = response.json().await?;
        let mut res: Vec<Article> = Vec::new();

        data.response.docs
            .into_iter()
            .for_each(
                |article| 
                res.push(Article {
                    url: article.web_url,
                    title: article.snippet, 
                    // text: article.r#abstract,
                    text: article.lead_paragraph,
                    published_at: article.pub_date,
                })
            );

        Ok(res)
    }
}
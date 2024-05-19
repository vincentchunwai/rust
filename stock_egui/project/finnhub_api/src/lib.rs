#[cfg(feature = "async")]
use reqwest::Method;
use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://finnhub.io/api/v1";

#[derive(thiserror::Error, Debug)]

pub enum FinnhubApiError{
    #[error("Failed fetching finnhub data")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Data Parsing failed")]
    DataParseFailed(#[from] serde_json::Error),
    #[error("Url Parsing failed")]
    UrlParsingError(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error)
}

#[derive(Deserialize, Debug)]
pub struct FinnhubApiResponse{
    pub symbol: String,
    pub status: String,
    quote: Quote,
    pub code: Option<String>,
}

impl FinnhubApiResponse{
    pub fn Quote(&self) -> &Quote{
        &self.quote
    }

    pub fn set_symbol(&mut self, symbol: String){
        self.symbol = symbol;
    }
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    c: f64,  // Current price
    d: f64,  // Change
    dp: f64, // Percent change
    h: f64,  // Day high
    l: f64,  // Day low
    o: f64,  // Open price
    pc: f64, // Previous close
}

impl Quote {
    pub fn current_price(&self) -> f64 {
        self.c
    }

    pub fn change(&self) -> f64 {
        self.d
    }

    pub fn percent_change(&self) -> f64 {
        self.dp
    }

    pub fn day_high(&self) -> f64 {
        self.h
    }

    pub fn day_low(&self) -> f64 {
        self.l
    }

    pub fn open_price(&self) -> f64 {
        self.o
    }

    pub fn previous_close(&self) -> f64 {
        self.pc
    }
}




pub struct FinnhubApi{
    api_key: String,
    stock: String,
}

impl FinnhubApi{
    pub fn new(api_key: &str, stock: &str) -> Self {
        Self{
            api_key: api_key.to_string(),
            stock: stock.to_string(),
        }
    }

    fn prepare_url(&self) -> Result<String, FinnhubApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.query_pairs_mut()
            .append_pair("symbol", &self.stock)
            .append_pair("token", &self.api_key);
        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<FinnhubApiResponse, FinnhubApiError> {
        let url = self.prepare_url()?;
        let mut response:FinnhubApiResponse = ureq::get(&url).call()?.into_json()?;
        match response.status.as_str(){
            "ok" => {response.set_symbol(self.stock.clone()); return Ok(response)},
            _ => return Err(map_response_err(response.code)),
        }
    }
}



fn map_response_err(code: Option<String>) -> FinnhubApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => FinnhubApiError::BadRequest("Your API key has been disabled"),
            _ => FinnhubApiError::BadRequest("Unknown error"),
        }
    } else {
        FinnhubApiError::BadRequest("Unknown error")
    }
}
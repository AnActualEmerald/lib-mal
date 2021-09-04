//! ## Quick Start
//! To use `lib-mal` you will need an API key from [MyAnimeList.net](https://myanimelist.net), and a callback URL. An example of how to use `lib-mal` might look like this:
//!
//! ```no_run
//! use lib_mal::ClientBuilder;
//! use std::path::PathBuf;
//! use lib_mal::MALError;
//!
//!  async fn test() -> Result<(), MALError>{
//!     //this has to exactly match a URI that's been registered with the MAL api
//!     let redirect = "[YOUR_REDIRECT_URI_HERE]";
//!     //the MALClient will attempt to refresh the cached access_token, if applicable
//!     let mut client = ClientBuilder::new().secret("[YOUR_SECRET_HERE]".to_string()).caching(true).cache_dir(PathBuf::from("[SOME_CACHE_DIR]")).build_with_refresh().await?;
//!     let (auth_url, challenge, state) = client.get_auth_parts();
//!     //the user will have to have access to a browser in order to log in and give your application permission
//!     println!("Go here to log in :) -> {}", auth_url);
//!     //once the user has the URL, be sure to call client.auth to listen for the callback and complete the OAuth2 handshake
//!     client.auth(&redirect, &challenge, &state).await?;
//!     //once the user is authorized, the API should be usable
//!     //this will get the details, including all fields, for Mobile Suit Gundam
//!     let anime = client.get_anime_details(80, None).await?;
//!     //because so many fields are optional, a lot of the members of lib_mal::model::AnimeDetails are `Option`s
//!     println!("{}: started airing on {}, ended on {}, ranked #{}", anime.show.title, anime.start_date.unwrap(), anime.end_date.unwrap(), anime.rank.unwrap());
//!     Ok(())
//!}
//!```

#[cfg(test)]
mod test;

mod builder;
mod client;
pub mod model;


pub use builder::ClientBuilder;
pub use client::MALClient;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct MALError {
    pub error: String,
    pub message: Option<String>,
    pub info: Option<String>,
}

impl Display for MALError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {} Message: {} Info: {}",
            self.error,
            self.message.as_ref().unwrap_or(&"None".to_string()),
            self.info.as_ref().unwrap_or(&"None".to_string())
        )
    }
}


impl MALError {
    pub fn new(msg: &str, error: &str, info: impl Into<Option<String>>) -> Self {
        MALError {
            error: error.to_owned(),
            message: Some(msg.to_owned()),
            info: info.into(),
        }
    }

}

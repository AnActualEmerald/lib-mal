//! ## Quick Start
//! To use `lib-mal` you will need an API key from [MyAnimeList.net](https://myanimelist.net), and a callback URL. An example of how to use `lib-mal` might look like this:
//!
//! ```no_run
//! use lib_mal::MALClient;
//! use std::path::PathBuf;
//! use lib_mal::MALError;
//!
//!  async fn test() -> Result<(), MALError>{
//!     //this has to exactly match a URI that's been registered with the MAL api
//!     let redirect = "[YOUR_REDIRECT_URI_HERE]";
//!     //the MALClient will attempt to refresh the cached access_token, if applicable
//!     let mut client = MALClient::init("[YOUR_SECRET_HERE]", true, Some(PathBuf::from("[SOME_CACHE_DIR]"))).await;
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

#[cfg(test)]
mod test;

pub mod model;

use model::{
    fields::AnimeFields,
    options::{Params, RankingType, Season},
    AnimeDetails, AnimeList, ForumBoards, ForumTopics, ListStatus, TopicDetails, User,
};

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::random;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str,
    time::SystemTime,
};
use tiny_http::{Response, Server};

///Exposes all of the API functions for the [MyAnimeList API](https://myanimelist.net/apiconfig/references/api/v2)
///
///**With the exception of all the manga-related functions which haven't been implemented yet**
///
///# Example
///```no_run
/// use lib_mal::MALClient;
/// # use lib_mal::MALError;
/// # async fn test() -> Result<(), MALError> {
/// let client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
/// //--do authorization stuff before accessing the functions--//
///
/// //Gets the details with all fields for Mobile Suit Gundam
/// let anime = client.get_anime_details(80, None).await?;
/// //You should actually handle the potential error
/// println!("Title: {} | Started airing: {} | Finished airing: {}",
///     anime.show.title,
///     anime.start_date.unwrap(),
///     anime.end_date.unwrap());
/// # Ok(())
/// # }
///```
pub struct MALClient {
    client_secret: String,
    dirs: PathBuf,
    access_token: String,
    client: reqwest::Client,
    caching: bool,
    pub need_auth: bool,
}

impl MALClient {
    ///Creates the client and fetches the MAL tokens from the cache if available. If `caching` is
    ///false or `cache_dir` is `None` the user will have to log in at the start of every session.
    ///
    ///When created client will attempt to refresh the access_token if it has expired
    pub async fn init(secret: &str, caching: bool, cache_dir: Option<PathBuf>) -> Self {
        let client = reqwest::Client::new();
        let mut will_cache = caching;
        let mut n_a = false;

        let dir = if let Some(d) = cache_dir {
            d
        } else {
            println!("No cache directory was provided, disabling caching");
            will_cache = false;
            PathBuf::new()
        };

        let mut token = String::new();
        if will_cache && dir.join("tokens").exists() {
            if let Ok(tokens) = fs::read(dir.join("tokens")) {
                let mut tok: Tokens = decrypt_tokens(&tokens).unwrap();
                if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    if n.as_secs() - tok.today >= tok.expires_in as u64 {
                        let params = [
                            ("grant_type", "refresh_token"),
                            ("refesh_token", &tok.refresh_token),
                        ];
                        let res = client
                            .post("https://myanimelist.net/v1/oauth2/token")
                            .form(&params)
                            .send()
                            .await
                            .expect("Unable to refresh token");
                        let new_toks: TokenResponse =
                            serde_json::from_str(&res.text().await.unwrap())
                                .expect("Unable to parse response");
                        token = new_toks.access_token.clone();
                        tok = Tokens {
                            access_token: new_toks.access_token,
                            refresh_token: new_toks.refresh_token,
                            expires_in: new_toks.expires_in,
                            today: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };

                        fs::write(dir.join("tokens"), encrypt_token(tok))
                            .expect("Unable to write token file")
                    } else {
                        token = tok.access_token;
                    }
                }
            }
        } else {
            will_cache = caching;
            n_a = true;
        }

        MALClient {
            client_secret: secret.to_owned(),
            dirs: dir,
            need_auth: n_a,
            access_token: token,
            client,
            caching: will_cache,
        }
    }

    ///Creates a client using provided token. Caching is disable by default.
    ///
    ///A client created this way can't authenticate the user if needed because it lacks a
    ///`client_secret`
    pub fn with_access_token(token: &str) -> Self {
        MALClient {
            client_secret: String::new(),
            need_auth: false,
            dirs: PathBuf::new(),
            access_token: token.to_owned(),
            client: reqwest::Client::new(),
            caching: false,
        }
    }

    ///Sets the directory the client will use for the token cache
    pub fn set_cache_dir(&mut self, dir: PathBuf) {
        self.dirs = dir;
    }

    ///Sets wether the client will cache or not
    pub fn set_caching(&mut self, caching: bool) {
        self.caching = caching;
    }

    ///Returns the auth URL and code challenge which will be needed to authorize the user.
    ///
    ///# Example
    ///
    ///```no_run
    ///     use lib_mal::MALClient;
    ///     # use  lib_mal::MALError;
    ///     # async fn test() -> Result<(), MALError> {
    ///     let redirect_uri = "http://localhost:2525";//<-- example uri
    ///     let mut client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
    ///     let (url, challenge, state) = client.get_auth_parts();
    ///     println!("Go here to log in: {}", url);
    ///     client.auth(&redirect_uri, &challenge, &state).await?;
    ///     # Ok(())
    ///     # }
    ///```
    pub fn get_auth_parts(&self) -> (String, String, String) {
        let verifier = pkce::code_verifier(128);
        let challenge = pkce::code_challenge(&verifier);
        let state = format!("bruh{}", random::<u8>());
        let url = format!("https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id={}&code_challenge={}&state={}", self.client_secret, challenge, state, );
        (url, challenge, state)
    }

    ///Listens for the OAuth2 callback from MAL on `callback_url`, which is the redirect_uri
    ///registered when obtaining the API token from MAL. Only HTTP URIs are supported right now.
    ///
    ///For now only applications with a single registered URI are supported, having more than one
    ///seems to cause issues with the MAL api itself
    ///
    ///# Example
    ///
    ///```no_run
    ///     use lib_mal::MALClient;
    ///     # use lib_mal::MALError;
    ///     # async fn test() -> Result<(), MALError> {
    ///     let redirect_uri = "localhost:2525";//<-- example uri,
    ///     //appears as "http://localhost:2525" in the API settings
    ///     let mut client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
    ///     let (url, challenge, state) = client.get_auth_parts();
    ///     println!("Go here to log in: {}", url);
    ///     client.auth(&redirect_uri, &challenge, &state).await?;
    ///     # Ok(())
    ///     # }
    ///
    ///```
    pub async fn auth(
        &mut self,
        callback_url: &str,
        challenge: &str,
        state: &str,
    ) -> Result<(), MALError> {
        let mut code = "".to_owned();
        let url = if callback_url.contains("http") {
            //server won't work if the url has the protocol in it
            callback_url
                .trim_start_matches("http://")
                .trim_start_matches("https://")
        } else {
            callback_url
        };

        let server = Server::http(url).unwrap();
        for i in server.incoming_requests() {
            if !i.url().contains(&format!("state={}", state)) {
                //if the state doesn't match, discard this response
                continue;
            }
            let res_raw = i.url();
            code = res_raw
                .split_once('=')
                .unwrap()
                .1
                .split_once('&')
                .unwrap()
                .0
                .to_owned();
            let response = Response::from_string("You're logged in! You can now close this window");
            i.respond(response).unwrap();
            break;
        }

        self.get_tokens(&code, challenge).await
    }

    async fn get_tokens(&mut self, code: &str, verifier: &str) -> Result<(), MALError> {
        let params = [
            ("client_id", self.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code_verifier", verifier),
            ("code", code),
        ];
        let rec = self
            .client
            .request(Method::POST, "https://myanimelist.net/v1/oauth2/token")
            .form(&params)
            .build()
            .unwrap();
        let res = self.client.execute(rec).await.unwrap();
        let text = res.text().await.unwrap();
        if let Ok(tokens) = serde_json::from_str::<TokenResponse>(&text) {
            self.access_token = tokens.access_token.clone();

            let tjson = Tokens {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                expires_in: tokens.expires_in,
                today: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            if self.caching {
                let mut f =
                    File::create(self.dirs.join("tokens")).expect("Unable to create token file");
                f.write_all(&encrypt_token(tjson))
                    .expect("Unable to write tokens");
            }
            Ok(())
        } else {
            Err(MALError::new("Unable to get tokends", "None", text))
        }
    }

    ///Sends a get request to the specified URL with the appropriate auth header
    async fn do_request(&self, url: String) -> Result<String, MALError> {
        match self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await
        {
            Ok(res) => Ok(res.text().await.unwrap()),
            Err(e) => Err(MALError::new(
                "Unable to send request",
                &format!("{}", e),
                None,
            )),
        }
    }

    ///Sends a put request to the specified URL with the appropriate auth header and
    ///form encoded parameters
    async fn do_request_forms(
        &self,
        url: String,
        params: Vec<(&str, String)>,
    ) -> Result<String, MALError> {
        match self
            .client
            .put(url)
            .bearer_auth(&self.access_token)
            .form(&params)
            .send()
            .await
        {
            Ok(res) => Ok(res.text().await.unwrap()),
            Err(e) => Err(MALError::new(
                "Unable to send request",
                &format!("{}", e),
                None,
            )),
        }
    }

    ///Tries to parse a JSON response string into the type provided in the `::<>` turbofish
    fn parse_response<'a, T: Serialize + Deserialize<'a>>(
        &self,
        res: &'a str,
    ) -> Result<T, MALError> {
        match serde_json::from_str::<T>(res) {
            Ok(v) => Ok(v),
            Err(_) => Err(match serde_json::from_str::<MALError>(res) {
                Ok(o) => o,
                Err(e) => MALError::new(
                    "Unable to parse response",
                    &format!("{}", e),
                    res.to_string(),
                ),
            }),
        }
    }

    ///Returns the current access token. Intended mostly for debugging.
    ///
    ///# Example
    ///
    ///```no_run
    /// # use lib_mal::MALClient;
    /// # use std::path::PathBuf;
    /// # async fn test() {
    ///     let client = MALClient::init("[YOUR_SECRET_HERE]", true, Some(PathBuf::new())).await;
    ///     let token = client.get_access_token();
    /// # }
    ///```
    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

    //Begin API functions

    //--Anime functions--//
    ///Gets a list of anime based on the query string provided
    ///`limit` defaults to 100 if `None`
    ///
    ///# Example
    ///
    ///```no_run
    /// # use lib_mal::MALClient;
    /// # use lib_mal::MALError;
    /// # async fn test() -> Result<(), MALError> {
    ///     # let client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
    ///     let list = client.get_anime_list("Mobile Suit Gundam", None).await?;
    ///     # Ok(())
    /// # }
    ///```
    pub async fn get_anime_list(
        &self,
        query: &str,
        limit: Option<u8>,
    ) -> Result<AnimeList, MALError> {
        let url = format!(
            "https://api.myanimelist.net/v2/anime?q={}&limit={}",
            query,
            limit.unwrap_or(100)
        );
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    ///Gets the details for an anime by the show's ID.
    ///Only returns the fields specified in the `fields` parameter
    ///
    ///Returns all fields when supplied `None`
    ///
    ///# Example
    ///
    ///```no_run
    /// use lib_mal::model::fields::AnimeFields;
    /// # use lib_mal::{MALError, MALClient};
    /// # async fn test() -> Result<(), MALError> {
    /// # let client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
    /// //returns an AnimeDetails struct with just the Rank, Mean, and Studio data for Mobile Suit Gundam
    /// let res = client.get_anime_details(80, AnimeFields::Rank | AnimeFields::Mean | AnimeFields::Studios).await?;
    /// # Ok(())
    /// # }
    ///
    ///```
    ///
    pub async fn get_anime_details<T: Into<Option<AnimeFields>>>(
        &self,
        id: u32,
        fields: T,
    ) -> Result<AnimeDetails, MALError> {
        let url = if let Some(f) = fields.into() {
            format!("https://api.myanimelist.net/v2/anime/{}?fields={}", id, f)
        } else {
            format!(
                "https://api.myanimelist.net/v2/anime/{}?fields={}",
                id,
                AnimeFields::ALL
            )
        };
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    ///Gets a list of anime ranked by `RankingType`
    ///
    ///`limit` defaults to the max of 100 when `None`
    ///
    ///# Example 
    ///
    ///```no_run
    /// # use lib_mal::{MALError, MALClient};
    /// use lib_mal::model::options::RankingType;
    /// # async fn test() -> Result<(), MALError> {
    /// # let client = MALClient::init("[YOUR_SECRET_HERE]", false, None).await;
    /// // Gets a list of the top 5 most popular anime
    /// let ranking_list = client.get_anime_ranking(RankingType::ByPopularity, Some(5)).await?;
    /// # Ok(())
    /// # }
    ///
    ///```
    pub async fn get_anime_ranking(
        &self,
        ranking_type: RankingType,
        limit: Option<u8>,
    ) -> Result<AnimeList, MALError> {
        let url = format!(
            "https://api.myanimelist.net/v2/anime/ranking?ranking_type={}&limit={}",
            ranking_type,
            limit.unwrap_or(100)
        );
        let res = self.do_request(url).await?;
        Ok(serde_json::from_str(&res).unwrap())
    }

    ///Gets the anime for a given season in a given year
    ///
    ///`limit` defaults to the max of 100 when `None`
    pub async fn get_seasonal_anime(
        &self,
        season: Season,
        year: u32,
        limit: Option<u8>,
    ) -> Result<AnimeList, MALError> {
        let url = format!(
            "https://api.myanimelist.net/v2/anime/season/{}/{}?limit={}",
            year,
            season,
            limit.unwrap_or(100)
        );
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    ///Returns the suggested anime for the current user. Can return an empty list if the user has
    ///no suggestions.
    pub async fn get_suggested_anime(&self, limit: Option<u8>) -> Result<AnimeList, MALError> {
        let url = format!(
            "https://api.myanimelist.net/v2/anime/suggestions?limit={}",
            limit.unwrap_or(100)
        );
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    //--User anime list functions--//

    ///Adds an anime to the list, or updates the element if it already exists
    pub async fn update_user_anime_status<T: Params>(
        //this doesn't have to be generic
        &self,
        id: u32,
        update: T,
    ) -> Result<ListStatus, MALError> {
        let params = update.get_params();
        let url = format!("https://api.myanimelist.net/v2/anime/{}/my_list_status", id);
        let res = self.do_request_forms(url, params).await?;
        self.parse_response(&res)
    }

    ///Returns the user's full anime list as an `AnimeList` struct.
    pub async fn get_user_anime_list(&self) -> Result<AnimeList, MALError> {
        let url = "https://api.myanimelist.net/v2/users/@me/animelist?fields=list_status&limit=4";
        let res = self.do_request(url.to_owned()).await?;

        self.parse_response(&res)
    }

    ///Deletes the anime with `id` from the user's anime list
    ///
    ///Returns 404 if the id isn't in the list.
    pub async fn delete_anime_list_item(&self, id: u32) -> Result<(), MALError> {
        let url = format!("https://api.myanimelist.net/v2/anime/{}/my_list_status", id);
        let res = self
            .client
            .delete(url)
            .bearer_auth(&self.access_token)
            .send()
            .await;
        match res {
            Ok(r) => {
                if r.status() == StatusCode::NOT_FOUND {
                    Err(MALError::new(
                        &format!("Anime {} not found", id),
                        r.status().as_str(),
                        None,
                    ))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(MALError::new(
                "Unable to send request",
                &format!("{}", e),
                None,
            )),
        }
    }

    //--Forum functions--//

    ///Returns a vector of `HashMap`s that represent all the forum boards on MAL
    pub async fn get_forum_boards(&self) -> Result<ForumBoards, MALError> {
        let res = self
            .do_request("https://api.myanimelist.net/v2/forum/boards".to_owned())
            .await?;
        self.parse_response(&res)
    }

    ///Returns details of the specified topic
    pub async fn get_forum_topic_detail(
        &self,
        topic_id: u32,
        limit: Option<u8>,
    ) -> Result<TopicDetails, MALError> {
        let url = format!(
            "https://api.myanimelist.net/v2/forum/topic/{}?limit={}",
            topic_id,
            limit.unwrap_or(100)
        );
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    ///Returns all topics for a given query
    pub async fn get_forum_topics(
        &self,
        board_id: Option<u32>,
        subboard_id: Option<u32>,
        query: Option<String>,
        topic_user_name: Option<String>,
        user_name: Option<String>,
        limit: Option<u32>,
    ) -> Result<ForumTopics, MALError> {
        let params = {
            let mut tmp = vec![];
            if let Some(bid) = board_id {
                tmp.push(format!("board_id={}", bid));
            }
            if let Some(bid) = subboard_id {
                tmp.push(format!("subboard_id={}", bid));
            }
            if let Some(bid) = query {
                tmp.push(format!("q={}", bid));
            }
            if let Some(bid) = topic_user_name {
                tmp.push(format!("topic_user_name={}", bid));
            }
            if let Some(bid) = user_name {
                tmp.push(format!("user_name={}", bid));
            }
            tmp.push(format!("limit={}", limit.unwrap_or(100)));
            tmp.join(",")
        };
        let url = format!("https://api.myanimelist.net/v2/forum/topics?{}", params);
        let res = self.do_request(url).await?;
        self.parse_response(&res)
    }

    ///Gets the details for the current user
    pub async fn get_my_user_info(&self) -> Result<User, MALError> {
        let url = "https://api.myanimelist.net/v2/users/@me?fields=anime_statistics";
        let res = self.do_request(url.to_owned()).await?;
        self.parse_response(&res)
    }
}

fn encrypt_token(toks: Tokens) -> Vec<u8> {
    let key = Key::from_slice(b"one two three four five six seve");
    let cypher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"but the eart");
    let plain = serde_json::to_vec(&toks).unwrap();
    let res = cypher.encrypt(nonce, plain.as_ref()).unwrap();
    res
}

fn decrypt_tokens(raw: &[u8]) -> Result<Tokens, MALError> {
    let key = Key::from_slice(b"one two three four five six seve");
    let cypher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"but the eart");
    match cypher.decrypt(nonce, raw.as_ref()) {
        Ok(plain) => {
            let text = String::from_utf8(plain).unwrap();
            Ok(serde_json::from_str(&text).expect("couldn't parse decrypted tokens"))
        }
        Err(e) => Err(MALError::new(
            "Unable to decrypt encrypted tokens",
            &format!("{}", e),
            None,
        )),
    }
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    token_type: String,
    expires_in: u32,
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    access_token: String,
    refresh_token: String,
    expires_in: u32,
    today: u64,
}

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

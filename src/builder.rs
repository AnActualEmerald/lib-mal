use reqwest::Client;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::client::{decrypt_tokens, encrypt_token, TokenResponse, Tokens};
use crate::{MALClient, MALError};

///# Example
///```
///  use lib_mal::ClientBuilder;
///  fn example() {
///     let client = ClientBuilder::new().secret("[YOUR_CLIENT_ID]".to_string()).access_token("exampleExAmPlE".to_string()).build_no_refresh();
///  }
///```
pub struct ClientBuilder {
    client_secret: Option<String>,
    dirs: Option<PathBuf>,
    access_token: Option<String>,
    caching: bool,
}

impl ClientBuilder {
    ///Creates a new ClientBuilder. All fields are set to None by default.
    pub fn new() -> Self {
        ClientBuilder {
            client_secret: None,
            dirs: None,
            access_token: None,
            caching: false,
        }
    }

    /// Sets the client_secret
    /// # Example
    ///
    ///```
    /// # use lib_mal::ClientBuilder;
    /// # fn test() {
    ///     let client =
    ///     ClientBuilder::new().secret("[YOUR_CLIENT_ID]".to_string()).build_no_refresh();
    ///
    ///     let another_client = ClientBuilder::new().secret(None).build_no_refresh();
    /// # }
    ///
    ///```
    pub fn secret(mut self, secret: impl Into<Option<String>>) -> Self {
        self.client_secret = secret.into();
        self
    }

    /// Sets the directory the client will use to cache the tokens
    /// # Example
    ///
    /// ```
    /// # use lib_mal::ClientBuilder;
    /// use std::path::PathBuf;
    /// # fn test() {
    ///     let client = ClientBuilder::new().cache_dir(PathBuf::new()).build_no_refresh();
    /// # }
    /// ```
    pub fn cache_dir(mut self, path: impl Into<Option<PathBuf>>) -> Self {
        self.dirs = path.into();
        self
    }

    /// Sets the access token for the client
    /// # Example
    ///
    /// ```
    /// # use lib_mal::ClientBuilder;
    /// # fn test() {
    ///     let client =
    ///     ClientBuilder::new().access_token("exampleToKeN".to_string()).build_no_refresh();
    /// # }
    /// ```
    pub fn access_token(mut self, token: impl Into<Option<String>>) -> Self {
        self.access_token = token.into();
        self
    }

    /// Sets wether or not the client should cache the tokens
    /// # Example
    ///
    /// ```
    /// # use lib_mal::ClientBuilder;
    /// # fn test() {
    ///     let client = ClientBuilder::new().caching(false).build_no_refresh();
    /// # }
    ///
    /// ```
    pub fn caching(mut self, caching: bool) -> Self {
        self.caching = caching;
        self
    }

    /// Builds a `MALClient` without attempting to refresh the access token
    ///
    /// # Example
    ///
    /// ```
    /// use lib_mal::ClientBuilder;
    /// use std::path::PathBuf;
    /// fn example() {
    ///     let client =
    ///     ClientBuilder::new().secret("[YOUR_CLIENT_ID]".to_string()).caching(true).cache_dir(PathBuf::new()).build_no_refresh();
    /// }
    pub fn build_no_refresh(self) -> MALClient {
        MALClient::new(
            self.client_secret.unwrap_or("".to_string()),
            self.dirs.unwrap_or(PathBuf::new()),
            self.access_token.unwrap_or("".to_string()).clone(),
            Client::new(),
            self.caching,
            false,
        )
    }


    /// Builds a `MALClient` after attempting to refresh the access token from cache 
    ///
    /// # Example
    ///
    /// ```
    /// use lib_mal::ClientBuilder;
    /// use lib_mal::MALError;
    /// use std::path::PathBuf;
    /// async fn example() -> Result<(), MALError> {
    ///     let client =
    ///     ClientBuilder::new().secret("[YOUR_CLIENT_ID]".to_string()).caching(true).cache_dir(PathBuf::new()).build_with_refresh().await?;
    ///
    ///     Ok(())
    /// }
    pub async fn build_with_refresh(self) -> Result<MALClient, MALError> {
        let client = reqwest::Client::new();
        let mut will_cache = self.caching;
        let mut n_a = false;

        let dir = if let Some(d) = self.dirs {
            d
        } else {
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
                            .await;
                        if let Err(e) = res {
                            return Err(MALError::new(
                                "Unable to refresh token",
                                e.to_string().as_str(),
                                None,
                            ));
                        }
                        let new_toks = serde_json::from_str::<TokenResponse>(
                            &res.unwrap().text().await.unwrap(),
                        );
                        if let Err(e) = new_toks {
                            return Err(MALError::new(
                                "Unable to parse token reponse",
                                e.to_string().as_str(),
                                None,
                            ));
                        }
                        let new_toks = new_toks.unwrap();
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

                        if let Err(e) = fs::write(dir.join("tokens"), encrypt_token(tok)) {
                            return Err(MALError::new(
                                "Unable to write tokens to cache",
                                e.to_string().as_str(),
                                None,
                            ));
                        }
                    } else {
                        token = tok.access_token;
                    }
                }
            }
        } else {
            will_cache = self.caching;
            n_a = true;
        }

        Ok(MALClient::new(
            self.client_secret.unwrap_or("".to_string()),
            dir,
            token,
            client,
            will_cache,
            n_a,
        ))
    }
}

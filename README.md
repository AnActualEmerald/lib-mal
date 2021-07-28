# lib-mal
A library for interacting with the MyAnimeList API

Handles authorization, requiring only a client ID from the MyAnimeList API and the user to authorize the application using the URL generated by the `MALClient`.
Tokens are cached by default but this can be disabled when creating the client.

API functions are a work in progress

Manga list functions are currently unsupported, but may be implemented in the future


## Getting Started
To use `lib-mal` you will need an API key from (MyAnimeList.net)[https://myanimelist.net], and a callback URL. An example of how to use `lib-mal` might look like this: 
```rust
use lib_mal::MALClient;
use tokio;

#[tokio::main]
async fn main(){
	//the MALClient will attempt to refresh the cached access_token, if applicable
	let client = MALClient::new([YOUR_SECRET_HERE]).await;
	let (auth_url, challenge) = client.get_auth_parts();
	//the user will have to have access to a browser in order to log in and give your application permission
	println!("Go here to log in :) -> {}", auth_url);
	//once the user has the URL, be sure to call client.auth(&challenge) to listen for the callback and complete the OAuth2 handshake
	client.auth(&challenge).await.expect("Unable to log in");
	//once the user is authorized, the API should be usable
	//this will get the details, including all fields, for Mobile Suit Gundam
	let anime = client.get_anime_details(&80, None).await.expect("Couldn't get anime details");
	//because so many fields are optional, a lot of the members of lib_mal::model::AnimeDetails are `Option`s 
	println!("{}: started airing on {}, ended on {}, ranked #{}", anime.show.title, anime.start_date.ok(), anime.end_date.ok(), anime.rank.ok())

}


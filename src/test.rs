use std::env;

use crate::model::AnimeList;
use crate::MALClient;
use tokio_test::block_on;
#[test]
fn anime_list() {
    let client = setup();
    let expected =
        serde_json::from_str::<AnimeList>(include_str!("test-data/anime_list.json")).unwrap();
    let result = block_on(client.get_anime_list("one", Some(4))).expect("Error performing request");
    let first = expected.data[0].node.id;
    let res_first = result.data[0].node.id;
    assert_eq!(first, res_first); //Really don't want to implement partial_eq for all these structs lol
}

fn setup() -> MALClient {
    let token = env::var("MAL_TOKEN").expect("Accesss token not found in environment");
    MALClient::with_access_token(&token)
}

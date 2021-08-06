use bitflags::bitflags;
use std::fmt::Display;

bitflags! {
    pub struct AnimeFields: u32 {
        const ID                        = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        const Title                     = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        const MainPicture               = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        const AlternativeTitles         = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        const StartDate                 = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        const EndDate                   = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        const Synopsis                  = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        const Mean                      = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        const Rank                      = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        const Popularity                = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        const NumListUsers              = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        const NumScoringUsers           = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        const NSFW                      = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        const CreatedAt                 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        const UpdatedAt                 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        const MediaType                 = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        const Status                    = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        const Genres                    = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const MyListStatus              = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        const NumEpisodes               = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        const StartSeason               = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        const Broadcast                 = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        const Source                    = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        const AverageEpisodeDuration    = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        const Rating                    = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        const Pictures                  = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        const Background                = 0b0000_0100_0000_0000_0000_0000_0000_0000;
        const RelatedAnime              = 0b0000_1000_0000_0000_0000_0000_0000_0000;
        const RelatedManga              = 0b0001_0000_0000_0000_0000_0000_0000_0000;
        const Recommendations           = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        const Studios                   = 0b0100_0000_0000_0000_0000_0000_0000_0000;
        const Statistics                = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        const ALL                       = 0b1111_1111_1111_1111_1111_1111_1111_1111;
    }
}

macro_rules! generate_get_anime_fields_names {
    {$ ($perm:ident => $name:expr),* $(,)?} => {
        impl AnimeFields {
            /// Returns a list of names of all contained fields.
            pub fn get_fields_names(self) -> Vec<&'static str> {
                let mut names = Vec::new();

                $(
                    if self.$perm() {
                        names.push($name);
                    }
                )*

                names
            }
        }
    }
}

generate_get_anime_fields_names! {
    id => "id",
    title => "title",
    main_picture => "main_picture",
    alternative_titles => "alternative_titles",
    start_date => "start_date",
    end_date => "end_date",
    synopsis => "synopsis",
    mean => "mean",
    rank => "rank",
    popularity => "popularity",
    num_list_users => "num_list_users",
    num_scoring_users => "num_scoring_users",
    nsfw => "nsfw",
    created_at => "created_at",
    updated_at => "updated_at",
    media_type => "media_type",
    status => "status",
    genres => "genres",
    my_list_status => "my_list_status",
    num_episodes => "num_episodes",
    start_season => "start_season",
    broadcast => "broadcast",
    source => "source",
    average_episode_duration => "average_episode_duration",
    rating => "rating",
    pictures => "pictures",
    background => "background",
    related_anime => "related_anime",
    related_manga => "related_manga",
    recommendations => "recommendations",
    studios => "studios",
    statistics => "statistics",
}

macro_rules! bits {
    ($($fn_name:ident => $bit_name:ident),* $(,)?) => {
        impl AnimeFields {
            $(
                pub fn $fn_name(self) -> bool {
                    self.contains(Self::$bit_name)
                }
            )*
        }
    };
}

bits!(
    id => ID,
    title => Title,
    main_picture => MainPicture,
    alternative_titles => AlternativeTitles,
    start_date => StartDate,
    end_date => EndDate,
    synopsis => Synopsis,
    mean => Mean,
    rank => Rank,
    popularity => Popularity,
    num_list_users => NumListUsers,
    num_scoring_users => NumScoringUsers,
    nsfw => NSFW,
    created_at => CreatedAt,
    updated_at => UpdatedAt,
    media_type => MediaType,
    status => Status,
    genres => Genres,
    my_list_status => MyListStatus,
    num_episodes => NumEpisodes,
    start_season => StartSeason,
    broadcast => Broadcast,
    source => Source,
    average_episode_duration => AverageEpisodeDuration,
    rating => Rating,
    pictures => Pictures,
    background => Background,
    related_anime => RelatedAnime,
    related_manga => RelatedManga,
    recommendations => Recommendations,
    studios => Studios,
    statistics => Statistics,
);

impl Display for AnimeFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_fields_names().join(","))
    }
}

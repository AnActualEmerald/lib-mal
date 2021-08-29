use std::fmt::Display;

#[derive(Debug)]
pub enum RankingType {
    All,
    Airing,
    Upcoming,
    TV,
    OVA,
    Movie,
    Special,
    ByPopularity,
    Favorite,
}

impl Display for RankingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let me = match self {
            Self::Favorite => "favorite".to_owned(),
            Self::TV => "tv".to_owned(),
            Self::Airing => "airing".to_owned(),
            Self::Upcoming => "upcoming".to_owned(),
            Self::Special => "special".to_owned(),
            Self::ByPopularity => "bypopularity".to_owned(),
            Self::Movie => "movie".to_owned(),
            Self::OVA => "ova".to_owned(),
            Self::All => "all".to_owned(),
        };
        write!(f, "{}", me)
    }
}

#[derive(Debug)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let me = match self {
            Self::Winter => "winter".to_owned(),
            Self::Spring => "spring".to_owned(),
            Self::Summer => "summer".to_owned(),
            Self::Fall => "fall".to_owned(),
        };
        write!(f, "{}", me)
    }
}

#[derive(Debug)]
pub enum Status {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let me = match self {
            Self::Watching => "watching".to_owned(),
            Self::Completed => "completed".to_owned(),
            Self::OnHold => "on_hold".to_owned(),
            Self::Dropped => "dropped".to_owned(),
            Self::PlanToWatch => "plan_to_watch".to_owned(),
        };
        write!(f, "{}", me)
    }
}

pub trait Params {
    fn get_params<'a>(self) -> Vec<(&'a str, String)>;
}

#[derive(Debug)]
pub struct StatusUpdate {
    status: Option<Status>,
    is_rewatching: Option<bool>,
    score: Option<u8>,
    num_watched_episodes: Option<u32>,
    priority: Option<u8>,
    num_times_rewatched: Option<u32>,
    rewatch_value: Option<u8>,
    tags: Option<Vec<String>>,
    comments: Option<String>,
}

impl Default for StatusUpdate {
    fn default() -> Self {
        StatusUpdate {
            status: None,
            is_rewatching: None,
            score: None,
            num_watched_episodes: None,
            priority: None,
            num_times_rewatched: None,
            rewatch_value: None,
            tags: None,
            comments: None,
        }
    }
}

impl StatusUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&mut self, status: Status) {
        self.status = Some(status);
    }

    pub fn is_rewatching(&mut self, is_rewatching: bool) {
        self.is_rewatching = Some(is_rewatching);
    }

    pub fn score(&mut self, score: u8) {
        self.score = Some(score);
    }
    pub fn num_watched_episodes(&mut self, num_watched_episodes: u32) {
        self.num_watched_episodes = Some(num_watched_episodes);
    }
    pub fn priority(&mut self, priority: u8) {
        self.priority = Some(priority);
    }
    pub fn num_times_rewatched(&mut self, num_times_rewatched: u32) {
        self.num_times_rewatched = Some(num_times_rewatched);
    }
    pub fn rewatch_value(&mut self, rewatch_value: u8) {
        self.rewatch_value = Some(rewatch_value);
    }
    pub fn tags(&mut self, tags: Vec<String>) {
        self.tags = Some(tags);
    }
    pub fn comments(&mut self, comments: &str) {
        self.comments = Some(comments.to_owned());
    }
}

impl Params for StatusUpdate {
    fn get_params<'a>(self) -> Vec<(&'a str, String)> {
        let mut params = vec![];
        if let Some(s) = self.status {
            params.push(("status", s.to_string()));
        }
        if let Some(rw) = self.is_rewatching {
            params.push(("is_rewatching", rw.to_string()));
        }
        if let Some(t) = self.score {
            params.push(("score", t.to_string()));
        }
        if let Some(t) = self.num_watched_episodes {
            params.push(("num_watched_episodes", t.to_string()));
        }
        if let Some(t) = self.priority {
            params.push(("priority", t.to_string()));
        }
        if let Some(t) = self.num_times_rewatched {
            params.push(("num_times_rewatched", t.to_string()));
        }
        if let Some(t) = self.rewatch_value {
            params.push(("rewatch_value", t.to_string()));
        }
        if let Some(t) = self.tags {
            params.push(("tags", t.join(",")));
        }
        if let Some(t) = self.comments {
            params.push(("comments", t));
        }

        params
    }
}

pub struct StatusBuilder {
    status: Option<Status>,
    is_rewatching: Option<bool>,
    score: Option<u8>,
    num_watched_episodes: Option<u32>,
    priority: Option<u8>,
    num_times_rewatched: Option<u32>,
    rewatch_value: Option<u8>,
    tags: Option<Vec<String>>,
    comments: Option<String>,
}

impl Default for StatusBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusBuilder {
    pub fn new() -> Self {
        StatusBuilder {
            status: None,
            is_rewatching: None,
            score: None,
            num_watched_episodes: None,
            priority: None,
            num_times_rewatched: None,
            rewatch_value: None,
            tags: None,
            comments: None,
        }
    }

    pub fn status(mut self, status: impl Into<Option<Status>>) -> Self {
        self.status = status.into();
        self
    }

    pub fn is_rewatching(mut self, is_rewatching: impl Into<Option<bool>>) -> Self {
        self.is_rewatching = is_rewatching.into();
        self
    }

    pub fn score(mut self, score: impl Into<Option<u8>>) -> Self {
        self.score = score.into();
        self
    }

    pub fn num_watched_episodes(mut self, num_watched_episodes: impl Into<Option<u32>>) -> Self {
        self.num_watched_episodes = num_watched_episodes.into();
        self
    }

    pub fn priority(mut self, priority: impl Into<Option<u8>>) -> Self {
        self.priority = priority.into();
        self
    }

    pub fn num_times_rewatched(mut self, num_times_rewatched: impl Into<Option<u32>>) -> Self {
        self.num_times_rewatched = num_times_rewatched.into();
        self
    }

    pub fn rewatch_value(mut self, rewatch_value: impl Into<Option<u8>>) -> Self {
        self.rewatch_value = rewatch_value.into();
        self
    }

    pub fn tags(mut self, tags: impl Into<Option<Vec<String>>>) -> Self {
        self.tags = tags.into();
        self
    }

    pub fn comments(mut self, comments: impl Into<Option<String>>) -> Self {
        self.comments = comments.into();
        self
    }

    pub fn build(self) -> StatusUpdate {
        StatusUpdate {
            status: self.status,
            is_rewatching: self.is_rewatching,
            score: self.score,
            num_watched_episodes: self.num_watched_episodes,
            priority: self.priority,
            num_times_rewatched: self.num_times_rewatched,
            rewatch_value: self.rewatch_value,
            tags: self.tags,
            comments: self.comments,
        }
    }
}

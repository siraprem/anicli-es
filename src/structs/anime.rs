pub struct Anime {
    pub title: String,
    pub category: String,
    pub link: String,
}

#[derive(Debug)]
pub struct AnimeEpisodeView {
    pub title: String,
    pub url: String,
    pub code: String,
}

#[derive(Debug)]
pub struct AnimeEpisode {
    pub lang: String,
    pub servers: Vec<AnimeEpisodeView>
}
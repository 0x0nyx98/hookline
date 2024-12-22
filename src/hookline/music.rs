use serde::Deserialize;

#[derive(Deserialize)]
pub struct Year {
    pub period: String,
    shows_count: i64,
    shows_duration: i64,
    venues_count: i64,
    era: String,
    cover_art_urls: CoverArtLinks,
}

#[derive(Deserialize)]
pub struct CoverArtLinks {
    large: String,
    medium: String,
    small: String,
}

#[derive(Deserialize)]
pub struct YearRange {

}

#[derive(Deserialize)]
pub struct Show {

}
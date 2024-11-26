use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCatRequest {
    #[validate(length(min = 1, max = 30))]
    pub name: String,

    #[validate(custom(function = "validate_race"))]
    pub race: String,

    #[validate(custom(function = "validate_sex"))]
    pub sex: String,

    #[serde(rename = "ageInMonth")]
    #[validate(range(min = 1, max = 120082))]
    pub age_in_month: i32,

    #[validate(length(min = 1, max = 200))]
    pub description: String,

    #[serde(rename = "imageUrls")]
    #[validate(custom(function = "validate_image_urls"))]
    pub img_urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCatPayload {
    #[validate(length(min = 1, max = 30))]
    pub name: String,

    #[validate(custom(function = "validate_race"))]
    pub race: String,

    #[validate(custom(function = "validate_sex"))]
    pub sex: String,

    #[serde(rename = "ageInMonth")]
    #[validate(range(min = 1, max = 120082))]
    pub age_in_month: i32,

    #[validate(length(min = 1, max = 200))]
    pub description: String,

    #[serde(rename = "imageUrls")]
    #[validate(custom(function = "validate_image_urls"))]
    pub img_urls: Vec<String>,

    #[serde(rename = "userId")]
    pub user_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub sex: String,
    #[serde(rename = "ageInMonth")]
    pub age_in_month: i32,
    pub description: String,
    #[serde(rename = "imageUrls")]
    pub img_urls: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "userId")]
    pub user_id: i32,
}

#[derive(Serialize, FromRow)]
pub struct CreateCatResponse {
    pub id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, FromRow)]
pub struct CatResponse {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub sex: String,
    #[serde(rename = "ageInMonth")]
    pub age_in_month: i32,
    pub description: String,
    #[serde(rename = "imageUrls")]
    pub img_urls: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(rename = "hasMatched")]
    pub has_matched: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct FilterCat {
    pub id: Option<i32>,
    pub limit: i32,
    pub offset: i32,
    pub race: Option<String>,
    pub sex: Option<String>,
    #[serde(rename = "ageInMonth")]
    pub age_in_month: Option<String>,
    #[serde(rename = "hasMatched")]
    pub has_matched: Option<bool>,
    pub owned: Option<bool>,
    pub search: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

impl Default for FilterCat {
    fn default() -> Self {
        Self {
            id: None,
            limit: 5,
            offset: 0,
            race: None,
            sex: None,
            age_in_month: None,
            has_matched: None,
            owned: None,
            search: None,
            user_id: None,
        }
    }
}

fn validate_race(race: &str) -> Result<(), ValidationError> {
    let valid_races = [
        "Persian",
        "Maine Coon",
        "Siamese",
        "Ragdoll",
        "Bengal",
        "Sphynx",
        "British Shorthair",
        "Abyssinian",
        "Scottish Fold",
        "Birman",
    ];
    if valid_races.contains(&race) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid race"))
    }
}

fn validate_sex(sex: &str) -> Result<(), ValidationError> {
    match sex {
        "male" | "female" => Ok(()),
        _ => Err(ValidationError::new("invalid sex")),
    }
}

fn validate_image_urls(image_urls: &Vec<String>) -> Result<(), ValidationError> {
    for url in image_urls {
        if url::Url::parse(url).is_err() {
            return Err(ValidationError::new("invalid url"));
        }
    }
    Ok(())
}

use crate::id_error;
use crate::v1::types::*;
use crate::v1::utils::*;
use actix_api_macros::*;
use actix_web::{post, web, Responder};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use tracing::{error, trace};

#[derive(ActixApiEnum)]
enum SearchResponse {
    #[success(json)]
    Recipes(Vec<BasicRecipe>),
    #[failure(message = "Error with request: {}")]
    RequestError(String),
    #[failure(message = "Internal server error.", json)]
    InternalError(Uuid),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// The search query.
    ///
    /// If `None`, do not search in any way for the name.
    pub query: Option<String>,
    /// The IDs of the recipes to whitelist in the search.
    ///
    /// If `None`, do not filter the results.
    pub whitelist: Option<Vec<Uuid>>,
    /// The amount of results per page. Defaults to 10.
    #[serde(default = "page_limit_default")]
    pub page_limit: u8,
    /// The page number. Default to 1.
    #[serde(default = "page_number_default")]
    pub page_number: u32,
    /// The nutrients to search for. If specified, the search will only return
    /// recipes that contain *any* of the specified nutrients.
    /// If you prefer, you can think of it as an "a *or* b" rather than an
    /// "a *and* b".
    ///
    /// If `None`, do not search for any nutrients.
    pub nutrients: Option<Vec<Nutrient>>,
}

fn page_limit_default() -> u8 {
    10
}

fn page_number_default() -> u32 {
    1
}

#[post("/search")]
pub async fn search(
    client: web::Data<mongodb::Client>,
    body: web::Json<SearchRequest>,
) -> impl Responder {
    let search_request = body.into_inner();

    trace!("Search request got. {:?}", search_request);

    // Enforce query limits.
    if let Err(e) = validate_query(&search_request) {
        error!("Query limits exceeded: {}", e);
        return SearchResponse::RequestError(e.to_string());
    }

    trace!("Query limits validated. Retrieving recipes.");

    let db = client.get_collection::<database::Recipe>(Collections::Recipes);

    // Sort by newest released, skipping to the right page,
    // taking only the right amount of results.
    let find_options = FindOptions::builder()
        .sort(doc! { "becomesPublic": -1 })
        .skip(Some(
            (search_request.page_number as u64 - 1) * (search_request.page_limit as u64),
        ))
        .limit(Some(search_request.page_limit as i64));

    // Stores the query information.
    let mut query_object = mongodb::bson::Document::new();

    // Add the search query, if provided.
    if let Some(query) = search_request.query {
        // Issue with $text is that it performs full-text and stemming search.
        // If you want to search for "vegetable", "vegetables" will work, but
        // "vege" or "veg" will not.
        // Therefore, we use a combination of partial-text and full-text,
        // by checking with $text and a regex.
        // This also allows for optional search terms. For example, `"vegetable"`
        // (emphasis on those quotes) will match only anything with exactly
        // 'vegetable' in it, and `-vegetable` will remove anything with
        // 'vegetable' in it.
        // The regex replaces spaces with .* so it will match any in-between
        // characters. This means a query like `chees bak` will match
        // 'Cheesy Vegetable Bake'. HOWEVER, `bak chees` will not match. It
        // does not check for reordering, as that can't be done with regex.
        // Overall, this provides a decent search experience for a quick
        // hack-y thing. In the future, something like Algolia may be better
        // to implement, but this is fine for a free solution.
        // https://www.mongodb.com/docs/manual/reference/operator/query/text/#mongodb-query-op.-text
        query_object.insert(
            "$or",
            vec![
                doc! {
                    "title": {
                        "$regex": query.clone().replace(' ', ".*"),
                        "$options": "i"
                    }
                },
                doc! {
                    "$text": {
                        "$search": query
                    }
                },
            ],
        );
    }

    // Add the whitelist, if provided.
    if let Some(whitelist) = search_request.whitelist {
        query_object.insert(
            "_id",
            doc! {
                "$in": whitelist,
            },
        );
    }

    // Add the nutrients, if provided.
    if let Some(nutrients) = search_request.nutrients {
        query_object.insert(
            "$or",
            nutrients
                .iter()
                .map(|nutrient| {
                    doc! {
                        "nutrients": nutrient
                    }
                })
                .collect::<Vec<mongodb::bson::Document>>(),
        );
    }

    let cursor = db.find(query_object, find_options.build()).await;

    let mut cursor = match cursor {
        Err(err) => {
            return SearchResponse::InternalError(id_error!(
                "Error getting search from database: {}",
                err
            ));
        }
        Ok(r) => r,
    };

    let mut recipes = vec![];
    loop {
        match cursor.advance().await {
            Ok(true) => match cursor.deserialize_current() {
                Ok(recipe) => recipes.push(BasicRecipe::from_recipe(&recipe)),
                Err(err) => {
                    return SearchResponse::InternalError(id_error!(
                        "Error deserializing recipe: {}",
                        err
                    ));
                }
            },
            Ok(false) => break,
            Err(err) => {
                return SearchResponse::InternalError(id_error!(
                    "Error getting search from database: {}",
                    err
                ));
            }
        }
    }

    SearchResponse::Recipes(recipes)
}

/// Ensures the query is valid.
fn validate_query(search_request: &SearchRequest) -> Result<(), String> {
    if let Some(query) = &search_request.query {
        if query.len() > 100 {
            return Err("Invalid `query`: Too long. Please limit to 100 characters.".to_string());
        }
    }

    if search_request.page_limit == 0 || search_request.page_limit > 20 {
        return Err(
            "Invalid `pageLimit`: Not within bounds. Please limit to between 1 and 20.".to_string(),
        );
    }

    if search_request.page_number == 0 {
        return Err("Invalid `pageNumber`: Must be greater than 0.".to_string());
    }

    Ok(())
}

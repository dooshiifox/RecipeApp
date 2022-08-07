use crate::v1::types::*;
use actix_web::Scope;

pub mod get;
pub mod get_basic;
pub mod get_short;
pub mod post;
pub mod weekly;

pub fn init(scope: Scope) -> Scope {
    scope
        .service(post::insert)
        .service(get_basic::uuid)
        .service(get_short::short)
        .service(get::uuid)
        .service(weekly::uuid)
}

/// The type of recipe sent in the request body.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRecipe {
    /// The unique identifier of the recipe.
    /// If set, overwrites the recipe with the same id.
    uuid: Option<Uuid>,
    /// The date the recipe was added to the database.
    /// If not set, defaults to the time the request was made.
    date_added: Option<Date>,
    /// The date the recipe will/went public. If not yet that date, can only be referred to by id.
    /// If not set, defaults to the time the request was made.
    becomes_public: Option<Date>,
    /// The staff who helped make this recipe.
    authors: Option<Vec<Uuid>>,
    /// A short string crediting the creators of the recipe.
    credits: Option<Formattable>,
    /// The date the recipe went weekly. None if never was weekly.
    weekly_timestamp: Option<Date>,
    /// The title of the recipe.
    title: Option<String>,
    /// The short title of the recipe.
    short: Option<String>,
    /// A list of common nutrients found in the recipe.
    nutrients: Option<Vec<SerdeStringNutrient>>,
    /// The time to cook the recipe, in minutes
    time_to_cook: Option<u16>,
    /// The servings of the recipe.
    servings: Option<u16>,
    /// The URL to the recipe image. Should be on S3
    image: Option<Url>,
    /// The gradient of the recipe.
    gradient: Option<Gradient>,
    /// The ingredients of the recipe.
    ingredients: Option<Vec<String>>,
    /// The recipe's method
    method: Option<database::Method>,
    /// The quiz information for the end of the recipe
    quiz: Option<database::Quiz>,
}

impl RequestRecipe {
    /// Tries to convert a RequestRecipe into a [`Recipe`].
    pub fn into_recipe(self) -> Result<database::Recipe, String> {
        let mut builder = database::Recipe::builder();

        /// Expands to ```if let Some(value) = self.[field] {
        ///     builder.[setter](value);
        /// }```
        macro_rules! if_some(
            ($field:tt, $setter:ident) => {
                if let Some(value) = self.$field {
                    builder = builder.$setter(value);
                }
            };
        );

        // Set all of the optionable fields on the recipe.
        if_some!(uuid, uuid);
        if_some!(date_added, date_added);
        if_some!(becomes_public, becomes_public);
        if_some!(authors, set_authors);
        if_some!(credits, credits);
        if_some!(weekly_timestamp, weekly_timestamp);
        if_some!(short, short);
        if_some!(title, title);
        if_some!(time_to_cook, time_to_cook);
        if_some!(servings, servings);
        if_some!(image, image);
        if_some!(gradient, gradient);
        if_some!(ingredients, ingredients);
        if_some!(method, method);
        if_some!(quiz, quiz);

        if let Some(nutrients) = self.nutrients {
            builder = builder.nutrients(
                nutrients
                    .iter()
                    .map(|nutrient| nutrient.into_nutrient())
                    .collect(),
            );
        }

        // Set the remaining Vec fields, then build and return.
        builder.build()
    }
}

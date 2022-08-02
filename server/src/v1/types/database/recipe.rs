use crate::v1::types::database::*;
use crate::v1::types::*;
use crate::WeeklyRecipeGetter;
use heck::ToKebabCase;

/// The database Recipe type that is sent to/used by the database.
///
/// Contains more information than found in the [`BasicRecipe`] type.
///
/// [`BasicRecipe`]: crate::v1::types::basic_recipe::BasicRecipe
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[readonly::make]
pub struct Recipe {
    /// The unique identifier of the recipe
    #[serde(rename = "_id")]
    pub uuid: Uuid,
    /// The date the recipe was added to the database
    pub date_added: Date,
    /// The date the recipe will/went public. If not yet that date, can only be referred to by id
    pub becomes_public: Date,
    /// The staff who helped make this recipe.
    pub authors: Vec<Uuid>,
    /// A short string crediting the creators of the recipe. Max 400 chars.
    pub credits: Option<Formattable>,
    /// The date the recipe went weekly. None if never was weekly.
    pub weekly_timestamp: Option<Date>,
    /// The short title of the recipe, kebab-cased.
    pub short: String,
    /// The title of the recipe. Max 80 chars.
    pub title: String,
    /// A list of common nutrients found in the recipe. Should be 1-3 long
    pub nutrients: Vec<Nutrient>,
    /// The time to cook the recipe, in minutes
    pub time_to_cook: u16,
    /// The servings of the recipe.
    pub servings: u16,
    /// The URL to the recipe image. Should be on S3
    pub image: Url,
    /// The gradient of the recipe.
    pub gradient: Gradient,
    /// The ingredients of the recipe. Max 80chars per ingredient.
    pub ingredients: Vec<String>,
    /// The recipe's method
    pub method: Method,
    /// The quiz information for the end of the recipe
    pub quiz: Quiz,
}

impl Recipe {
    /// Constructs a RecipeBuilder which can be used to build a Recipe.
    pub fn builder() -> RecipeBuilder {
        RecipeBuilder::default()
    }

    /// Gets the UUID of the recipe
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Returns if the recipe is currently weekly or not.
    ///
    /// Currently checks if the Recipe has a weekly timestamp and that the
    /// weekly timestamp was not more than 7 days ago. In the future, this
    /// should change to check in the database if this is the newest
    /// valid weekly.
    pub async fn is_weekly(&self, weekly_getter: &WeeklyRecipeGetter) -> bool {
        match weekly_getter.get().await {
            Ok(weekly) => weekly.uuid == self.uuid,
            Err(_) => false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RecipeBuilder {
    /// The unique identifier of the recipe
    uuid: Option<Uuid>,
    /// The date the recipe was added to the database
    date_added: Option<Date>,
    /// The date the recipe will/went public. If not yet that date, can only be referred to by id
    becomes_public: Option<Date>,
    /// The staff who helped make this recipe.
    authors: Vec<Uuid>,
    /// A short string crediting the creators of the recipe. Max 400 chars.
    credits: Option<Formattable>,
    /// The date the recipe went weekly. None if never was weekly.
    weekly_timestamp: Option<Date>,
    /// The short title of the recipe, kebab-cased.
    short: Option<String>,
    /// The title of the recipe. Max 80 chars.
    title: Option<String>,
    /// A list of common nutrients found in the recipe. Should be 1-3 long
    nutrients: Vec<Nutrient>,
    /// The time to cook the recipe, in minutes
    time_to_cook: Option<u16>,
    /// The servings of the recipe.
    servings: Option<u16>,
    /// The URL to the recipe image. Should be on S3
    image: Option<Url>,
    /// The gradient of the recipe.
    gradient: Option<Gradient>,
    /// The ingredients of the recipe. Max 80chars per ingredient.
    ingredients: Vec<String>,
    /// The recipe's method
    method: Option<Method>,
    /// The quiz information for the end of the recipe
    quiz: Option<Quiz>,
}

impl RecipeBuilder {
    /// Builds the RecipeBuilder into a [`Recipe`]
    ///
    /// [`Recipe`]: crate::v1::types::database::Recipe
    pub fn build(self) -> Result<Recipe, String> {
        if self.nutrients.is_empty() || self.nutrients.len() > 3 {
            return Err("Recipe must have 1-3 nutrients".to_string());
        }
        if self.ingredients.is_empty() {
            return Err("Recipe must have at least one ingredient".to_string());
        }

        let short = self
            .get_short()
            .ok_or_else(|| "No short set for recipe.".to_string())?;

        Ok(Recipe {
            uuid: self.uuid.unwrap_or_else(Uuid::random),
            date_added: self.date_added.unwrap_or_else(Date::now),
            becomes_public: self.becomes_public.unwrap_or_else(Date::now),
            authors: self.authors,
            credits: self.credits,
            weekly_timestamp: self.weekly_timestamp,
            title: self
                .title
                .ok_or_else(|| "No title set for recipe.".to_string())?,
            short,
            nutrients: self.nutrients,
            time_to_cook: self
                .time_to_cook
                .ok_or_else(|| "No time to cook set for recipe.".to_string())?,
            servings: self
                .servings
                .ok_or_else(|| "No servings set for recipe.".to_string())?,
            image: self
                .image
                .ok_or_else(|| "No image set for recipe.".to_string())?,
            gradient: self
                .gradient
                .ok_or_else(|| "No gradient set for recipe.".to_string())?,
            ingredients: self.ingredients,
            method: self
                .method
                .ok_or_else(|| "No method set for recipe.".to_string())?,
            quiz: self
                .quiz
                .ok_or_else(|| "No quiz set for recipe.".to_string())?,
        })
    }

    /// Returns the current Short of the recipe.
    pub fn get_short(&self) -> Option<String> {
        if let Some(short) = &self.short {
            Some(short.to_string())
        } else {
            self.title.as_ref().map(|title| title.to_kebab_case())
        }
    }

    /// Set the UUID of the recipe.
    pub fn uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    /// Set the date the recipe was added to the database.
    /// Defaults to the current date.
    pub fn date_added(mut self, date_added: Date) -> Self {
        self.date_added = Some(date_added);
        self
    }
    /// The date the recipe will go public.
    /// Defaults to the current date.
    pub fn becomes_public(mut self, goes_public_on: Date) -> Self {
        self.becomes_public = Some(goes_public_on);
        self
    }
    /// Sets the list of users who made this recipe.
    pub fn set_authors(mut self, made_by: Vec<Uuid>) -> Self {
        self.authors = made_by;
        self
    }

    /// Adds a user to the list of users who made this recipe.
    pub fn add_author(mut self, user_uuid: Uuid) -> Self {
        self.authors.push(user_uuid);
        self
    }
    /// Sets the credits of the recipe.
    pub fn credits(mut self, credits: Formattable) -> Self {
        self.credits = Some(credits);
        self
    }
    /// Sets the timestamp this becomes the weekly recipe.
    /// If not set, will never become the weekly recipe.
    pub fn weekly_timestamp(mut self, weekly_timestamp: Date) -> Self {
        self.weekly_timestamp = Some(weekly_timestamp);
        self
    }

    /// Sets the title of the recipe.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the short title of the recipe.
    pub fn short(mut self, short: String) -> Self {
        self.short = Some(short);
        self
    }

    /// Sets the list of nutrients found in the recipe.
    pub fn nutrients(mut self, nutrients: Vec<Nutrient>) -> Self {
        self.nutrients = nutrients;
        self
    }

    /// Adds a nutrient to the list of nutrients in the recipe.
    pub fn add_nutrient(mut self, nutrient: Nutrient) -> Self {
        self.nutrients.push(nutrient);
        self
    }
    /// Sets the time, in minutes, to cook this recipe.
    pub fn time_to_cook(mut self, time_to_cook: u16) -> Self {
        self.time_to_cook = Some(time_to_cook);
        self
    }
    /// Sets the servings of the recipe.
    pub fn servings(mut self, servings: u16) -> Self {
        self.servings = Some(servings);
        self
    }
    /// Sets the URL to the recipe image.
    pub fn image(mut self, image: Url) -> Self {
        self.image = Some(image);
        self
    }

    /// Sets the gradient of the recipe.
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Sets the ingredients of the recipe.
    pub fn ingredients(mut self, ingredients: Vec<String>) -> Self {
        self.ingredients = ingredients;
        self
    }

    /// Adds an ingredient to the recipe.
    pub fn add_ingredient(mut self, ingredient: String) -> Self {
        self.ingredients.push(ingredient);
        self
    }

    /// Sets the method of the recipe.
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    /// Sets the quiz information for the end of the recipe.
    pub fn quiz(mut self, quiz: Quiz) -> Self {
        self.quiz = Some(quiz);
        self
    }
}

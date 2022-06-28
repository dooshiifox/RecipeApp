use mongodb::Collection;

const DATABASE_NAME: &str = "recipe_db";

/// An enum of all collections present in the database.
/// Exists to make queries easier, so that there is no need to remember
/// the name of a collection.
pub enum Collections {
    Recipes,
}

impl Collections {
    /// Returns the name of the collection.
    pub fn name(&self) -> &'static str {
        match self {
            Collections::Recipes => "recipes",
        }
    }
}

/// A trait that makes getting collections easier.
pub trait GetCollection {
    /// Returns a reference to the collection specified by [`Collections`].
    fn get_collection<T>(&self, collection: Collections) -> Collection<T>;
}

impl GetCollection for mongodb::Client {
    fn get_collection<T>(&self, collection: Collections) -> Collection<T> {
        self.database(DATABASE_NAME).collection(collection.name())
    }
}

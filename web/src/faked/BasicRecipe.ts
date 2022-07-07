import { BasicRecipe } from '$types/BasicRecipe';
import { seperator } from '$types/nutrient';
import {
	randomUuid,
	random,
	pick,
	randomIngredient,
	randomTimeToCook,
	randomServings,
	randomRating,
	randomGradient,
	randomNutrients,
	getRandomImage
} from './random';
import { browser } from '$app/env';

/** Generates `count` number of random Basic Recipes. */
export default function generateBasicRecipes(count: number): BasicRecipe[] {
	const recipes: BasicRecipe[] = [];
	for (let i = 0; i < count; i++) {
		recipes.push(generateBasicRecipe());
	}
	return recipes;
}

/** Generates a single basic recipe. */
export function generateBasicRecipe(): BasicRecipe {
	const title = generateRandomTitle();

	// Convert the title to kebab-case for the url.
	// Match anything not a letter or number and replace all of them in a row.
	// e.g., Sour Bacon, Rice, and Cabbage on Rice => sour-bacon-rice-and-cabbage-on-rice
	const titleKebab = title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

	const basicRecipe = new BasicRecipe(
		randomUuid(),
		'/recipe/' + titleKebab,
		title,
		getRandomImage(),
		randomNutrients(),
		randomTimeToCook(),
		randomServings(),
		randomGradient()
	);

	if (browser) {
		// Random rating 0-5 with 50% chance for no set rating.
		basicRecipe.rating = randomRating(true);
		// 50% chance for the recipe to be bookmarked.
		basicRecipe.bookmarked = random() == 1;
	}

	return basicRecipe;
}

export function generateRandomTitle(): string {
	const foodTypes = [
		'Pizza',
		'Pasta',
		'Sushi',
		'Salad',
		'Sauce',
		'Muffins',
		'Soup',
		'Curry',
		'Burgers',
		'on Rice'
	];
	const adjectives = ['Spicy', 'Sweet', 'Sour', 'Salty', 'Fresh', 'Hot', 'Cold', 'Warm'];

	// Pick between 1 and 3 ingredients.
	const randomIngredients = [];
	const ingredientQuantity = random(1, 4);
	for (let i = 0; i < ingredientQuantity; i++) {
		randomIngredients.push(randomIngredient());
	}
	const randomFoodType = pick(foodTypes);

	// 50% chance to add a random adjective to the front.
	if (random()) {
		return `${pick(adjectives)} ${seperator(randomIngredients)} ${randomFoodType}`;
	} else {
		return `${seperator(randomIngredients)} ${randomFoodType}`;
	}
}

import type { BasicRecipe } from 'src/types/BasicRecipe';
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
import { seperator } from '../types/nutrient';

/** Generates `count` number of random Basic Recipes. */
export default function generateBasicRecipes(count: number): BasicRecipe[] {
	const recipes: BasicRecipe[] = [];
	for (let i = 0; i < count; i++) {
		recipes.push(generateBasicRecipe());
	}
	return recipes;
}

/** Generates  */
export function generateBasicRecipe(): BasicRecipe {
	const title = generateRandomTitle();

	// Convert the title to kebab-case for the url.
	const titleKebab = title.toLowerCase().replace(/\s+/g, '-');

	return {
		id: randomUuid(),
		url: '/recipe/' + titleKebab,
		title,
		image: getRandomImage(),
		nutrients: randomNutrients(),
		timeToCook: randomTimeToCook(),
		servings: randomServings(),
		gradient: randomGradient(),
		rating: randomRating(true),
		// 50% chance for the recipe to be bookmarked.
		bookmarked: random() == 1
	};
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
	for (let i = 0; i < random(1, 3); i++) {
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

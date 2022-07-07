import type { Gradient } from '$types/gradient';

/** Generates a random number [min, max)  */
export function random(min: number, max: number): number;
/** Generates a random number [0, max) */
export function random(max: number): number;
/** Generates a random number 0 or 1 */
export function random(): number;

export function random(min?: number, max?: number): number {
	// Generate a random 0 or 1
	if (min === undefined) {
		return Math.random() < 0.5 ? 0 : 1;
	}

	if (max === undefined) {
		max = min;
		min = 0;
	}
	return Math.floor(Math.random() * (max - min)) + min;
}

/** Picks a random element from a list. */
export function pick<T>(list: T[]): T {
	return list[random(list.length)];
}

/** Returns a randomly generated UUID. */
// https://qawithexperts.com/article/javascript/generating-guiduuid-using-javascript-various-ways/372
export function randomUuid(): string {
	return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
		const r = (Math.random() * 16) | 0;
		const v = c == 'x' ? r : (r & 0x3) | 0x8;
		return v.toString(16);
	});
}

/** Returns a random ingredient in a recipe. */
export function randomIngredient(): string {
	const ingredients = [
		'Tomato',
		'Onion',
		'Garlic',
		'Mushroom',
		'Pineapple',
		'Pepper',
		'Bacon',
		'Cheese',
		'Chicken',
		'Beef',
		'Pork',
		'Fish',
		'Shrimp',
		'Beetroot',
		'Watermelon',
		'Cucumber',
		'Carrot',
		'Celery',
		'Chilli',
		'Bean',
		'Pea',
		'Potato',
		'Cabbage',
		'Cauliflower',
		'Broccoli',
		'Lettuce',
		'Seed',
		'Pumpkin',
		'Rice'
	];

	return pick(ingredients);
}

/** Returns a random amount of time it takes to cook a recipe. */
export function randomTimeToCook(): number {
	const times = [
		3, 5, 8, 10, 12, 15, 20, 25, 30, 40, 45, 50, 60, 75, 80, 90, 105, 120, 150, 180, 210, 240, 270,
		300, 360, 480, 600, 720
	];

	return pick(times);
}

/** Returns a random number of servings for a recipe. */
export function randomServings(): number {
	const servings = [1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 18, 20, 24, 30, 36, 40, 48, 50];

	return pick(servings);
}

/** Returns a random rating for a recipe. */
export function randomRating(allowUndefined?: boolean): number | undefined {
	// If undefined is allowed, return undefined with 50% chance.
	if (allowUndefined && random()) {
		return undefined;
	}
	// Generate a random number [0, 10] and then divide by 2 to get [0, 5]
	return random(11) / 2;
}

/** Returns a random nutrient. */
export function randomNutrient(): string {
	// Edited list from server/src/v1/types/nutrient.rs
	// to remove ones that are unknown and sound made up.
	const nutrients = [
		'Calories',
		'Fat',
		'Saturated Fat',
		'Cholesterol',
		'Carbohydrates',
		'Fiber',
		'Sugar',
		'Protein',
		'Vitamin A',
		'Vitamin B1',
		'Vitamin B2',
		'Vitamin B3',
		'Vitamin B5',
		'Vitamin B6',
		'Vitamin B9',
		'Vitamin B12',
		'Vitamin C',
		'Vitamin D',
		'Vitamin E',
		'Vitamin K',
		'Choline',
		'Calcium',
		'Copper',
		'Iodine',
		'Iron',
		'Magnesium',
		'Manganese',
		'Phosphorus',
		'Potassium',
		'Selenium',
		'Sodium',
		'Zinc',
		'Starch',
		'Sucrose',
		'Glucose',
		'Fructose',
		'Lactose',
		'Omega 3S',
		'Omega 6S'
	];

	return pick(nutrients);
}

/** Returns a random list of 1 - 3 nutrients. */
export function randomNutrients(): string[] {
	const nutrients = [];

	const quantity = random(1, 3);
	for (let i = 0; i < quantity; i++) {
		nutrients.push(randomNutrient());
	}

	return nutrients;
}

/** Picks from a list of premade gradients and returns one. */
export function randomGradient(): Gradient {
	const gradients: [string, string][] = [
		['#d3a972', '#e8a16e'],
		['#f9ddb8', '#ffd886'],
		['#f8f9b8', '#d5ff86'],
		['#d0f9b8', '#a0e9ac'],
		['#b8f9db', '#9fddd3'],
		['#c6d6e6', '#90acc9'],
		['#d6d6e6', '#90acc9'],
		['#d6d6e6', '#b4a8d6'],
		['#d7c2e4', '#c4a6db'],
		['#e4c2de', '#dba6bf'],
		['#e4c2cc', '#dba6a6'],
		['#e4c1b8', '#dbb08c']
	];

	return pick(gradients);
}

/** Returns a random image from a pre-selected group of images. */
export function getRandomImage(): string {
	const images = [
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/avocado-and-lentil-salad_737_373_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/courgette-meatballs_737_373_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/carrot-muffins_737_373_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/tomato-capsicum-pasta-sauce_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/quinoa-and-beetroot-salad-750x380_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/gazpacho-soup_(002)_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/asparagus-tart_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/cauliflower-and-lentil-curry_737_373_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/pumpkin-fritters_737_373_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/corn-and-lentil-fritters_737_372_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/watermelon-salad-750x380_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/beetroot-hummus-750x380_737_372_c1.png',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/kumara-salad_737_373_c1.jpg',
		'https://www.heartfoundation.org.nz/media/images/all-shared-sections/recipes/vegetable-bake-750x380_737_373_c1.png'
	];

	return pick(images);
}

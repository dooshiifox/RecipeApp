import type { Gradient } from './gradient';
import type { Nutrient } from './nutrient';

export interface BasicRecipe {
	/** The UUID of the recipe. */
	id: string;
	/** The URL to the recipe + method. */
	url: string;
	/** The title of the recipe. */
	title: string;
	/** A URL to the image of the recipe. */
	image: string;
	/** The nutrients included in the recipe. */
	nutrients: Nutrient[];
	/** The time it takes to cook the recipe, in minutes. */
	timeToCook: number;
	/** The servings the recipe produces. */
	servings: number;
	/** The background gradient of the recipe. */
	gradient: Gradient;
	/** The recipe's rating. Undefined if not rated. */
	rating?: number;
	/** Whether the recipe is bookmarked for later. */
	bookmarked: boolean;
}

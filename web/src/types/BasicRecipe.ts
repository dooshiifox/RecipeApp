import storage from '../store/localStorage';
import type { Gradient } from './gradient';
import type { Nutrient } from './nutrient';
import type { Uuid } from './uuid';

export class BasicRecipe {
	id: Uuid;
	url: string;
	title: string;
	image: string;
	nutrients: Nutrient[];
	timeToCook: number;
	servings: number;
	gradient: Gradient;

	constructor(
		id: string,
		url: string,
		title: string,
		image: string,
		nutrients: Nutrient[],
		timeToCook: number,
		servings: number,
		gradient: Gradient
	) {
		this.id = id;
		this.url = url;
		this.title = title;
		this.image = image;
		this.nutrients = nutrients;
		this.timeToCook = timeToCook;
		this.servings = servings;
		this.gradient = gradient;
	}

	/** Whether the recipe is bookmarked or not. */
	get bookmarked(): boolean {
		if (storage === undefined) return false;

		// Check localstorage and see if its in the list.
		let bookmarks = storage.bookmarks;
		if (!bookmarks) {
			storage.bookmarks = JSON.stringify([]);
			return false;
		}

		bookmarks = JSON.parse(bookmarks);
		if (bookmarks === null) {
			throw new Error('Could not parse LocalStorage `bookmarks`');
		}
		return bookmarks.includes(this.id);
	}

	/** Updates the bookmark status of the recipe. */
	set bookmarked(bookmarked: boolean) {
		if (storage === undefined) return;

		const bookmarks = JSON.parse(storage.bookmarks || '[]');
		if (!Array.isArray(bookmarks)) {
			throw new Error('Could not parse LocalStorage `bookmarks` as array');
		}

		const bookmarkSet = new Set(bookmarks as string[]);
		if (bookmarked) {
			bookmarkSet.add(this.id);
		} else {
			bookmarkSet.delete(this.id);
		}
		storage.bookmarks = JSON.stringify([...bookmarkSet]);
	}

	/** The rating of the recipe. Undefined if not rated. */
	get rating(): number | undefined {
		if (storage === undefined) return;

		const rating = storage.ratings;
		if (!rating) {
			return undefined;
		}

		const ratings = JSON.parse(rating);
		if (ratings === null) {
			throw new Error('Could not parse LocalStorage `ratings`');
		}

		return ratings[this.id];
	}

	/** Updates the rating of the recipe. */
	set rating(rating: number | undefined) {
		if (storage === undefined) return;

		const ratings = JSON.parse(storage.ratings || '{}');
		if (!(ratings instanceof Object)) {
			throw new Error('Could not parse LocalStorage `ratings` as object');
		}

		if (rating === undefined) {
			delete ratings[this.id];
		} else {
			ratings[this.id] = rating;
		}

		storage.ratings = JSON.stringify(ratings);
	}
}

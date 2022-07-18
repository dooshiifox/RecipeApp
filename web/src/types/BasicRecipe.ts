import { get, type APIErrorResponse } from '$utils/fetch';
import { getBookmarks, setBookmarks } from '$store/bookmarks';
import { getRatings, setRatings } from '$store/ratings';
import type { DateMs } from './date';
import type { Gradient } from './gradient';
import type { Nutrient } from './nutrient';
import type { Uuid } from './uuid';

/** The weekly recipe cacher. */
let weekly: BasicRecipe | null = null;

/** The BasicRecipe cacher. */
const cachedRecipes: Map<Uuid, BasicRecipe> = new Map();

export interface BasicRecipeConstructor {
	dateAdded: DateMs;
	short: string;
	title: string;
	isWeekly: boolean;
	image: string;
	nutrients: Nutrient[];
	timeToCook: number;
	servings: number;
	gradient: Gradient;
}

export interface BasicRecipeConstructorId extends BasicRecipeConstructor {
	id: Uuid;
}
export interface BasicRecipeConstructorUuid extends BasicRecipeConstructor {
	uuid: Uuid;
}

export class BasicRecipe implements BasicRecipeConstructorId {
	id: Uuid;
	dateAdded: DateMs;
	short: string;
	title: string;
	isWeekly: boolean;
	image: string;
	nutrients: Nutrient[];
	timeToCook: number;
	servings: number;
	gradient: Gradient;

	constructor(c: BasicRecipeConstructorId | BasicRecipeConstructorUuid) {
		if ('id' in c) {
			this.id = c.id;
		} else {
			this.id = c.uuid;
		}
		this.dateAdded = c.dateAdded;
		this.short = c.short;
		this.title = c.title;
		this.isWeekly = c.isWeekly;
		this.image = c.image;
		this.nutrients = c.nutrients;
		this.timeToCook = c.timeToCook;
		this.servings = c.servings;
		this.gradient = c.gradient;
	}

	/** Gets a BasicRecipe from the API by its UUID. */
	static getById(id: Uuid, f: typeof fetch = fetch): Promise<BasicRecipe> {
		const cached = cachedRecipes.get(id);
		if (cached) {
			return Promise.resolve(cached);
		}

		return BasicRecipe.getFromUrl(`/recipe-basic/id/${id}`, f);
	}

	/** Gets the current weekly recipe. */
	static getWeekly(f: typeof fetch = fetch): Promise<BasicRecipe> {
		// Check if weekly has already been retrieved.
		if (weekly) return Promise.resolve(weekly);

		return BasicRecipe.getFromUrl('/weekly', f).then((r) => {
			// Cache the weekly recipe.
			weekly = r;
			return weekly;
		});
	}

	/** Returns a new BasicRecipe by retrieving its information from
	 * the given API endpoint.
	 *
	 * Resolves: `BasicRecipe`
	 * Rejects: `{ message: string, data?: { message?: string, data?: any } }`
	 */
	static getFromUrl(url: string, f: typeof fetch = fetch): Promise<BasicRecipe> {
		return get<BasicRecipeConstructorUuid>(url, f)
			.catch((e: APIErrorResponse) => {
				return Promise.reject({
					message: `Server returned an unexpected value when retrieving \`${url}\``,
					data: e.error
				});
			})
			.then((resp) => {
				if (resp.success) {
					const r = new BasicRecipe(resp.data);
					cachedRecipes.set(r.id, r);
					return r;
				} else {
					return Promise.reject({
						message: `Server could not retrieve \`${url}\``,
						data: resp.error
					});
				}
			});
	}

	/** Whether the recipe is bookmarked or not. */
	get bookmarked(): boolean {
		const bookmarks = getBookmarks();

		if (bookmarks === undefined) {
			throw new Error('Could not get bookmarked recipes');
		}
		return bookmarks.has(this.id);
	}

	/** Updates the bookmark status of the recipe. */
	set bookmarked(bookmarked: boolean) {
		const bookmarks = getBookmarks();
		if (bookmarks === undefined) return;

		if (bookmarked) {
			bookmarks.add(this.id);
		} else {
			bookmarks.delete(this.id);
		}

		setBookmarks(bookmarks);
	}

	/** The rating of the recipe. Undefined if not rated. */
	get rating(): number | undefined {
		const ratings = getRatings();

		if (ratings === undefined) {
			throw new Error('Could not get rated recipes');
		}

		return ratings[this.id];
	}

	/** Updates the rating of the recipe. */
	set rating(rating: number | undefined) {
		const ratings = getRatings();
		if (ratings === undefined) return;

		if (rating === undefined) {
			delete ratings[this.id];
		} else {
			ratings[this.id] = rating;
		}

		setRatings(ratings);
	}
}

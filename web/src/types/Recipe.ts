import { get, type APIErrorResponse } from '$utils/fetch';
import { BasicRecipe, type BasicRecipeConstructor } from './BasicRecipe';
import type { Uuid } from './uuid';

export type Formattable = string;

export interface Warning {
	title: string;
	content: Formattable;
}

export interface Info {
	title: string;
	content: Formattable;
}

export interface SubStep {
	content: Formattable;
	image?: string;
	warnings?: Warning[];
	infos?: Info[];
}

export interface Step {
	title: string;
	substeps: SubStep[];
}

export interface Method {
	steps: Step[];
}

/** The Recipe cacher. */
const cachedRecipesByUuid: Map<Uuid, Recipe> = new Map();
const cachedRecipesByShort: Map<Uuid, Recipe> = new Map();

export interface RecipeConstructor extends BasicRecipeConstructor {
	ingredients: string[];
	method: Method;
}

export interface RecipeConstructorId extends RecipeConstructor {
	id: Uuid;
}

export interface RecipeConstructorUuid extends RecipeConstructor {
	uuid: Uuid;
}

export class Recipe extends BasicRecipe implements RecipeConstructorId {
	ingredients: string[];
	method: Method;

	constructor(c: RecipeConstructorId | RecipeConstructorUuid) {
		super(c);

		this.ingredients = c.ingredients;
		this.method = c.method;
	}

	/** Gets a Recipe from the API by its UUID. */
	static getById(id: Uuid, f: typeof fetch = fetch): Promise<Recipe> {
		const cached = cachedRecipesByUuid.get(id);
		if (cached) {
			return Promise.resolve(cached);
		}

		return Recipe.getFromUrl(`/recipe/id/${id}`, f);
	}

	/** Gets a Recipe from the API by its short. */
	static getByShort(short: string, f: typeof fetch = fetch): Promise<Recipe> {
		const cached = cachedRecipesByShort.get(short);
		if (cached) {
			return Promise.resolve(cached);
		}

		return Recipe.getFromUrl(`/recipe/short/${short}`, f);
	}

	/** Returns a new Recipe by retrieving its information from
	 * the given API endpoint.
	 *
	 * Resolves: `Recipe`
	 * Rejects: `APIErrorResponse<APIErrorResponse["error"]>["error"]`
	 */
	static async getFromUrl(url: string, f: typeof fetch = fetch): Promise<Recipe> {
		return get<RecipeConstructorUuid>(url, f)
			.catch((e: APIErrorResponse) => {
				return Promise.reject({
					message: `Server returned an unexpected value when retrieving \`${url}\``,
					data: e.error
				});
			})
			.then((resp) => {
				if (resp.success) {
					const r = new Recipe(resp.data);
					cachedRecipesByUuid.set(r.id, r);
					cachedRecipesByShort.set(r.short, r);
					return r;
				} else {
					return Promise.reject({
						message: `Server could not retrieve \`${url}\``,
						data: resp.error
					});
				}
			});
	}
}

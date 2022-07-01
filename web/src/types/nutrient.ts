/** A type of nutrient. */
export type Nutrient = string;

/** Converts a bunch of Nutrient strings into a "Contains ..." string.
 *
 * # Examples
 *
 * ```ts
 * const contains = toRecipeString(['Carbohydrates']);
 * // contains === 'Contains Carbohydrates'
 *
 * const contains = toRecipeString(['Carbohydrates', 'Fiber']);
 * // contains === 'Contains Carbohydrates and Fiber'
 *
 * const contains = toRecipeString(['Carbohydrates', 'Fiber', 'Protein']);
 * // contains === 'Contains Carbohydrates, Fiber, and Protein'
 * ```
 */
export function toRecipeString(nutrients: Nutrient[]): string {
	if (nutrients.length === 0) {
		return 'Contains... no nutrients?';
	} else if (nutrients.length === 1) {
		return 'Contains ' + nutrients[0];
	} else if (nutrients.length === 2) {
		return `Contains ${nutrients[0]} and ${nutrients[1]}`;
	} else if (nutrients.length === 3) {
		return `Contains ${nutrients[0]}, ${nutrients[1]}, and ${nutrients[2]}`;
	} else {
		return 'Contains a lot of nutrients!';
	}
}

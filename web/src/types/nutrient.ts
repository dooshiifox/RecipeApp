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
	return `Contains ${seperator(nutrients) || 'no nutrients?'}`;
}

export function seperator(text: string[]): string {
	switch (text.length) {
		case 0:
			return '';
		case 1:
			return text[0];
		case 2:
			return `${text[0]} and ${text[1]}`;
		default:
			return `${text.slice(0, -1).join(', ')}, and ${text.slice(-1)}`;
	}
}

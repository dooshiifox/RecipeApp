/** A gradient type is a collection of two colors.
 * The first color is the starting color, generally the brighter one present
 * in the top left of the gradient.
 * The second color is the ending color, generally the darker one present
 * in the bottom right of the gradient.
 */
export type Gradient = [Color, Color];

/** A color of some kind supported by CSS. */
export type Color = string;

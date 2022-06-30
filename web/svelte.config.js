import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import svg from '@poppanator/sveltekit-svg';

// Remove `<rect fill="none" height="24" width="24"/>`
// that are on Material icons.
const removeMaterialBackground = {
	name: 'removeMaterialBackground',
	type: 'visitor',
	fn: () => {
		return {
			element: {
				exit: (node, parentNode) => {
					if (isBlackBackground(node)) {
						// Taken from
						// https://github.com/svg/svgo/blob/a9834efa1603198ae606765292215462ae193d0d/lib/xast.js#L82
						parentNode.children = parentNode.children.filter((child) => child !== node);
					}
				}
			}
		};
	}
};

function isBlackBackground(node) {
	// Check `<rect fill="none" height="24" width="24"/>`
	if (
		node.name === 'rect' &&
		node.attributes.fill === 'none' &&
		node.attributes.height === '24' &&
		node.attributes.width === '24'
	)
		return true;

	// Check `<path d="M0 0h24v24H0V0z" fill="none"/>`
	if (
		node.name === 'path' &&
		node.attributes.d === 'M0 0h24v24H0V0z' &&
		node.attributes.fill === 'none'
	)
		return true;

	return false;
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [
		preprocess({
			postcss: true
		})
	],

	kit: {
		adapter: adapter(),

		// Override http methods in the Todo forms
		methodOverride: {
			allowed: ['PATCH', 'DELETE']
		},
		vite: {
			// Options are optional
			plugins: [
				svg({
					includePaths: ['./src/lib/assets/icons/'],
					svgoOptions: {
						multipass: true,
						plugins: [
							removeMaterialBackground,
							{
								name: 'preset-default',
								// by default svgo removes the viewBox which prevents svg icons from scaling
								// not a good idea! https://github.com/svg/svgo/pull/1461
								params: { overrides: { removeViewBox: false } }
							},
							{ name: 'removeAttrs', params: { attrs: '(fill|stroke)' } }
						]
					}
				})
			]
		}
	}
};

export default config;

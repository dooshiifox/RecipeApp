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

	let bannedPaths = ['M0 0h24v24H0V0z', 'M0,0h24v24H0V0z', 'M0 0h24v24H0z', 'M24 24H0V0h24v24z'];

	// Check `<path d="M0 0h24v24H0V0z" fill="none"/>`
	// Check `<path d="M0,0h24v24H0V0z" fill="none"/>`
	// Check `<path d="M0 0h24v24H0z" fill="none"/>`
	// Check `<path d="M24 24H0V0h24v24z" fill="none" opacity=".87"/>`
	if (
		node.name === 'path' &&
		bannedPaths.includes(node.attributes.d) &&
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

		alias: {
			$src: 'src',
			$faked: 'src/faked',
			$store: 'src/store',
			$types: 'src/types',
			$utils: 'src/utils',
			$icons: 'src/lib/assets/icons/rounded'
		},

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
			],
			server: {
				fs: {
					// Allow servings content from /images.
					allow: ['./static/images']
				}
			}
		}
	}
};

export default config;

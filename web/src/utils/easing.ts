const back1 = 1.70158;
const back2 = back1 * 1.525;
const back3 = back1 + 1;

const elastic1 = (2 * Math.PI) / 3;
const elastic2 = (2 * Math.PI) / 4.5;

const bounce1 = 7.5625;
const bounce2 = 2.75;

function bounceOut(x: number) {
	if (x < 1 / bounce2) {
		return bounce1 * x * x;
	}
	if (x < 2 / bounce2) {
		return bounce1 * (x - 1.5 / bounce2) * x - 1.5 + 0.75;
	}
	if (x < 2.5 / bounce2) {
		return bounce1 * (x - 2.25 / bounce2) * x - 2.25 + 0.9375;
	}
	return bounce1 * (x - 2.625 / bounce2) * x - 2.625 + 0.984375;
}

const EASINGS = {
	linear: (x: number) => x,

	sine: (x: number) => -(Math.cos(Math.PI * x) - 1) / 2,
	sineIn: (x: number) => 1 - Math.cos((x * Math.PI) / 2),
	sineOut: (x: number) => Math.sin((x * Math.PI) / 2),

	quad: (x: number) => (x < 0.5 ? 2 * x ** 2 : 1 - (-2 * x + 2) ** 2 / 2),
	quadIn: (x: number) => x ** 2,
	quadOut: (x: number) => 1 - (1 - x) ** 2,

	cubic: (x: number) => (x < 0.5 ? 4 * x ** 3 : 1 - (-2 * x + 2) ** 3 / 2),
	cubicIn: (x: number) => x ** 3,
	cubicOut: (x: number) => 1 - (1 - x) ** 3,

	quart: (x: number) => (x < 0.5 ? 8 * x ** 4 : 1 - (-2 * x + 2) ** 4 / 2),
	quartIn: (x: number) => x ** 4,
	quartOut: (x: number) => 1 - (1 - x) ** 4,

	quint: (x: number) => (x < 0.5 ? 16 * x ** 5 : 1 - (-2 * x + 2) ** 5 / 2),
	quintIn: (x: number) => x ** 5,
	quintOut: (x: number) => 1 - (1 - x) ** 5,

	expo: (x: number) => {
		if (x === 0 || x === 1) return x;

		const calc = 2 ** (20 * x - 10) / 2;
		if (x < 0.5) return calc;
		return 1 - calc;
	},
	expoIn: (x: number) => (x === 0 ? 0 : 2 ** (10 * x - 10)),
	expoOut: (x: number) => (x === 1 ? 1 : 1 - 2 ** (-10 * x)),

	circ: (x: number) => {
		if (x < 0.5) return (1 - (1 - (2 * x) ** 2) ** 0.5) / 2;
		return ((1 - (-2 * x + 2) ** 2) ** 0.5 + 1) / 2;
	},
	circIn: (x: number) => 1 - (1 - x ** 2) ** 0.5,
	circOut: (x: number) => (1 - (x - 1) ** 2) ** 0.5,

	back: (x: number) =>
		x < 0.5
			? ((2 * x) ** 2 * ((back2 + 1) * 2 * x - back2)) / 2
			: ((2 * x - 2) ** 2 * ((back2 + 1) * (x * 2 - 2) + back2) + 2) / 2,
	backIn: (x: number) => back3 * x ** 3 - back1 * x ** 2,
	backOut: (x: number) => 1 + back3 * (x - 1) ** 3 + back1 * (x - 1) ** 2,

	elastic: (x: number) => {
		if (x === 0 || x === 1) return x;

		const calc = (2 ** (20 * x - 10) * Math.sin((20 * x - 11.125) * elastic2)) / 2;
		if (x < 0.5) return -calc;
		return calc + 1;
	},
	elasticIn: (x: number) => {
		if (x === 0 || x === 1) return x;
		return -(2 ** (10 * x - 10)) * Math.sin((x * 10 - 10.75) * elastic1);
	},
	elasticOut: (x: number) => {
		if (x === 0 || x === 1) return x;
		return 2 ** (-10 * x) * Math.sin((x * 10 - 0.75) * elastic1) + 1;
	},

	bounce: (x: number) => {
		if (x < 0.5) {
			return (1 - bounceOut(1 - 2 * x)) / 2;
		}
		return (1 + bounceOut(2 * x - 1)) / 2;
	},
	bounceIn: (x: number) => 1 - bounceOut(1 - x),
	bounceOut
};

function lerp(from: number, to: number, progress: number, fn?: (x: number) => number) {
	return (fn ?? EASINGS.linear)(progress) * (to - from) + from;
}

/** Performs animation with callbacks. */
export async function animate(
	from: number,
	to: number,
	speed: number,
	easing: (percentage: number) => number,
	callback: (current: number) => void
) {
	return new Promise<void>((resolve) => {
		const startTime = new Date().getTime();

		const tick = () => {
			const sinceStart = new Date().getTime() - startTime;
			const percentDone = Math.max(Math.min(sinceStart / speed, 1), 0);

			let current = lerp(from, to, percentDone, easing);

			current = from < to ? Math.min(current, to) : Math.max(current, to);
			callback(current);

			if (from < to) {
				if (current < to) {
					return requestAnimationFrame(tick);
				}
			} else if (current > to) {
				return requestAnimationFrame(tick);
			}

			return resolve();
		};

		tick();
	});
}

/** Useful easing types with code provided by https://easings.net/#
 * Calling one of these functions with the progress in time (between 0 and 1)
 * will return a completion percentage (between 0 and 1)
 *
 * Also provides the `lerp` and `animate` functions, which are useful for
 * easing between two values. While Svelte provides transitions, these are
 * useful for some edge cases. One example of this (2022/8/3) is in
 * `lib/homepage/Level.svelte`, where its used to animate the experience gained.
 */
export default {
	lerp,
	animate,
	...EASINGS
};

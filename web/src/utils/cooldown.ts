/** A cooldown contains a timer and a callback function.
 *
 * Using the cooldown will immediately run the callback if the cooldown is
 * expired, or will wait until after it is before running it.
 */
export class Cooldown {
	/** The cooldown, in ms. */
	cooldown: number;

	/** The timeout before running the next one, if it should be run again. */
	timeout: NodeJS.Timeout | null;

	/** Whether the timer should restart immediately after expiring. */
	performAgain: boolean;

	/** The callback for the cooldown. */
	callback: () => void;

	constructor(cooldown: number, callback: () => void) {
		this.cooldown = cooldown;
		this.timeout = null;
		this.performAgain = false;
		this.callback = callback;
	}

	/**
	 * Checks if the cooldown is ready.
	 * @returns True if the cooldown is ready, false otherwise.
	 */
	isReady(): boolean {
		return this.timeout === null;
	}

	/**
	 * Attempt to use the cooldown.
	 *
	 * If the cooldown is ready, the callback will be called immediately.
	 * Else, waits for the cooldown to be ready before calling it.
	 *
	 * @returns True if the cooldown was used, false otherwise.
	 */
	use(): boolean {
		console.debug(this);
		if (this.isReady()) {
			// Perform the callback before the timer starts.
			this.callback();
			// Set the timeout for the cooldown.
			this.timeout = setTimeout(() => {
				// Clear the timeout once it's done.
				this.timeout = null;
				// If `use` has been called since this started but before it
				// finished, restart the timer.
				if (this.performAgain) {
					this.performAgain = false;
					this.use();
				}
			}, this.cooldown);
			return true;
		} else {
			// Timer is not ready. Indicate it should restart as soon as
			// it's finished.
			this.performAgain = true;
			return false;
		}
	}
}

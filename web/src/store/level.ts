import { browser } from '$app/env';

/** Returns the amount of XP to get from level `n` to level `n + 1`. */
export function xpRequiredAtLevel(n: number): number {
	// Level 1 => Level 2: 100xp   (100 + 25(0))
	// Level 2 => Level 3: 125xp   (100 + 25(1))
	// Level 3 => Level 4: 150xp   (100 + 25(2))
	// Level 4 => Level 5: 175xp   (100 + 25(3))
	// Level 5 => Level 6: 200xp   (100 + 25(4))
	// Level 6 => Level 7: 225xp   (100 + 25(5))
	// Level 7 => Level 8: 250xp   (100 + 25(6))
	// ...
	return 100 + 25 * (n - 1);
}

/** Converts experience into a level. */
export function xpToLevel(xp: number): number {
	// Found by rearranging `levelToXp` into a quadratic equation.
	// Rearranging to find lvl...
	// xp = (lvl^2 + 7lvl) * 12.5
	// xp/12.5 = lvl^2 + 7lvl
	// 0 = lvl^2 + 7lvl - xp/12.5
	// a = 1,  b = 7,  c = xp/12.5
	// lvl = -b + sqrt(b^2 - 4ac) / 2a
	const discriminant = Math.sqrt(49 + (8 * xp) / 25);
	const root = (-7 + discriminant) / 2;
	return Math.floor(root) + 1;
}

/** Converts a level into the minimum amount of experience required to achieve it. */
export function levelToXp(level: number): number {
	// Equivalent to looping over `toLevelUp` until the total XP is
	// greater than the given amount.
	// Found by converting toLevelUp into the polynomial
	// xp = (((lvl + 3)(lvl + 4) / 2) - 6) * 25
	//    = (((lvl^2 + 7lvl + 12) / 2) - 6) * 25
	//    = (lvl^2 + 7lvl) / 2 * 25
	//    = (lvl^2 + 7lvl) * 12.5
	return (level ** 2 + 7 * level) * 12.5 - 100;
}

export interface LevelingInfo {
	/** The total amount of experience ever recieved by this user. */
	totalXp: number;
	/** The current level of the user. */
	level: number;
	/** The current experience the user has in this level. */
	xp: number;
	/** The amount of experience remaining to reach the next level. */
	xpRemaining: number;
	/** The amount of experience required to reach the next level. */
	xpForNextLevel: number;
}

/** Returns the level and XP in the current level given
 * an amount of XP. */
export function getLevel(totalXp: number): LevelingInfo {
	const level = xpToLevel(totalXp);
	const levelXp = levelToXp(level);
	const xpForNextLevel = xpRequiredAtLevel(level);
	const xpRemaining = xpForNextLevel - levelXp;
	return { totalXp, level, xp: totalXp - levelXp, xpRemaining, xpForNextLevel };
}

/** Returns the total amount of experience the user has. */
export function getTotalXp(): number {
	if (browser) {
		return parseInt(localStorage.getItem('totalXp') ?? '0');
	}
	return -1;
}

/** Sets the total amount of experience the user has. */
export function setTotalXp(totalXp: number): void {
	if (browser) {
		localStorage.setItem('totalXp', totalXp.toString());
	}
}

/** Returns the user's experience and level info. */
export default function getLevelingInfo(): LevelingInfo {
	return getLevel(getTotalXp());
}

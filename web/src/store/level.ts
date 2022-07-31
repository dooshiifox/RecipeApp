import { derived, writable } from 'svelte/store';
import storage, { lsWritable } from './localStorage';

/** Returns the amount of XP to get from level `n` to level `n + 1`. */
export function xpRequiredAtLevel(n: number): number {
	// https://www.desmos.com/calculator/sy7ts2eqyc
	// Level 1 => Level 2: 100xp   (100 + 25(0))
	// Level 2 => Level 3: 125xp   (100 + 25(1))
	// Level 3 => Level 4: 150xp   (100 + 25(2))
	// Level 4 => Level 5: 175xp   (100 + 25(3))
	// Level 5 => Level 6: 200xp   (100 + 25(4))
	// Level 6 => Level 7: 225xp   (100 + 25(5))
	// Level 7 => Level 8: 250xp   (100 + 25(6))
	// ...
	return 75 + 25 * n;
}

/** Converts experience into a level. */
export function xpToLevel(xp: number): number {
	// https://www.desmos.com/calculator/sy7ts2eqyc
	// Found by rearranging `levelToXp` into a quadratic equation.
	// Rearranging to find lvl...
	// xp = 12.5lvl^2 + 62.5lvl - 75
	// xp = 12.5(lvl^2 + 5lvl - 6)
	// xp/12.5 = lvl^2 + 5lvl - 6
	// 0 = lvl^2 + 5lvl - 6 - xp/12.5
	// a = 1,  b = 5,  c = -6 - xp/12.5
	// lvl = -b + sqrt(b^2 - 4ac) / 2a
	//     = (-5 + sqrt(25 - 4(-6 - xp/12.5))) / 2
	return Math.floor((-5 + Math.sqrt(25 - 4 * (-6 - xp / 12.5))) / 2);
}

/** Converts a level into the minimum amount of experience required to achieve it. */
export function levelToXp(level: number): number {
	// https://www.desmos.com/calculator/sy7ts2eqyc
	// Level 1 => 0     (0xp required to get to level 1 as its the base)
	// Level 2 => 100xp
	// Level 3 => 225xp (+125xp)
	// Level 4 => 375xp (+150xp)
	// Level 5 => 550xp (+175xp)
	// Level 6 => 750xp (+200xp)
	return 12.5 * level ** 2 + 62.5 * level - 75;
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
	if (storage === undefined) {
		return -1;
	}
	return storage.totalXp.get() ?? 0;
}

/** Sets the total amount of experience the user has. */
export function setTotalXp(totalXp: number): void {
	if (storage !== undefined) {
		storage.totalXp.set(totalXp);
	}
}

/** Returns the user's experience and level info. */
export default function getLevelingInfo(): LevelingInfo {
	return getLevel(getTotalXp());
}

let totalXp = lsWritable<number>(writable('null'));
if (storage !== undefined) totalXp = storage.totalXp;
export const levelingInfo = derived(totalXp, ($totalXp) => getLevel($totalXp ?? 0));

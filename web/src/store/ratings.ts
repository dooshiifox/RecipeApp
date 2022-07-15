import type { Uuid } from '$types/uuid';
import storage from './localStorage';

/** Returns a `Record<Uuid, number>` for the ratings of recipes. */
export function getRatings(): Record<Uuid, number> | undefined {
	if (storage === undefined) return;

	const ratings = JSON.parse(storage.ratings || '{}');
	if (!(ratings instanceof Object)) {
		throw new Error('Could not parse LocalStorage `ratings` as object');
	}

	return ratings;
}

/** Sets the ratings. */
export function setRatings(ratings: Record<Uuid, number>): void {
	if (storage === undefined) return;

	storage.ratings = JSON.stringify(ratings);
}

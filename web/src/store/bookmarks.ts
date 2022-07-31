import type { Uuid } from '$types/uuid';
import storage from './localStorage';

/** Returns a Set of bookmarks. */
export function getBookmarks(): Set<Uuid> | undefined {
	if (storage === undefined) return;

	const bookmarks = storage.bookmarks.get() || [];
	if (!Array.isArray(bookmarks)) {
		throw new Error('Could not parse LocalStorage `bookmarks` as array');
	}

	return new Set(bookmarks as Uuid[]);
}

/** Sets the bookmarks. */
export function setBookmarks(bookmarks: Set<Uuid>): void {
	if (storage === undefined) return;

	storage.bookmarks.set([...bookmarks]);
}

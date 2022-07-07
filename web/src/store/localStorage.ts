import { browser, dev } from '$app/env';

// const handlerServer = {
// 	get: (target: Record<string, string>, key: string | symbol): string | null => {
// 		console.warn(new Error(`Tried to get from storage on server: \`${String(key)}\``).stack);
// 		return null;
// 	},
// 	set: (target: Record<string, string>, key: string | symbol, value: any): boolean => {
// 		console.warn('Tried to set to storage on server: `%s` = `%s`', key, value);
// 		return false;
// 	}
// };

// Notes about this:
// In dev mode, saves to session because parsing localStorage is slow
// and generating a bunch of fakes can make it very terrible.
// In prod mode, saves to localStorage because it's persistant.
const handlerBrowser = {
	get: (target: Record<string, string>, key: string | symbol): string | null => {
		if (typeof key === 'string') {
			if (dev) {
				return sessionStorage.getItem(key);
			} else {
				return localStorage.getItem(key);
			}
		} else {
			throw new Error('Tried to get key from localStorage with non-string key');
		}
	},
	set: (target: Record<string, string>, key: string | symbol, value: any): boolean => {
		if (typeof key !== 'string') {
			throw new Error('Tried to set key to localStorage with non-string key');
		}
		if (typeof value !== 'string') {
			throw new Error('Tried to set key to localStorage with non-string value');
		}
		if (dev) {
			sessionStorage.setItem(key, value);
		} else {
			localStorage.setItem(key.toString(), value);
		}
		return true;
	}
};

/** If in a server environment, returns `undefined`. Else,
 * returns an object that can get and set values to LocalStorage.
 *
 * Accessed with `storage[key]` and `storage[key] = value`.
 *
 * When getting, `key` is a string and returns a string.
 * When setting, `key` is a string and `value` is a string. */
// const storage = new Proxy({} as Record<string, string>, browser ? handlerBrowser : handlerServer);
const storage = !browser ? undefined : new Proxy({} as Record<string, string>, handlerBrowser);

export default storage;

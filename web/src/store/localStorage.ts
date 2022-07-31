import { browser, dev } from '$app/env';
import type { Uuid } from '$src/types';
import {
	writable,
	type Subscriber,
	type Unsubscriber,
	type Updater,
	type Writable
} from 'svelte/store';

/** Cleanup logic callback. */
declare type Invalidator<T> = (value?: T) => void;

type JsonParseable = string;

export interface LsWritable<T> {
	subscribe(
		this: void,
		run: Subscriber<T | null>,
		invalidate?: Invalidator<T | null>
	): Unsubscriber;
	set(this: void, value: T): void;
	update(this: void, updater: Updater<T | null>): void;
	get(): T | null;
}

export function lsWritable<T>(w: Writable<JsonParseable>): LsWritable<T> {
	const { subscribe, set } = w;

	const get = () => {
		let value = null;
		const unsub = w.subscribe((v) => {
			if (v !== null) value = JSON.parse(v);
		});
		unsub();
		return value as T | null;
	};

	return {
		subscribe(value: Subscriber<T | null>, invalidate?: Invalidator<T | null>) {
			const unsub = subscribe(
				(v) => {
					if (v !== null) value(JSON.parse(v));
					else value(null);
				},
				(v) => {
					if (!invalidate || v === undefined) return;
					if (v !== null) invalidate(JSON.parse(v));
					else invalidate(null);
				}
			);

			return unsub;
		},
		set(value: T) {
			const save = JSON.stringify(value);

			return set(save);
		},
		update(updater: Updater<T | null>) {
			const value = get();
			const save = JSON.stringify(updater(value));

			return set(save);
		},
		get
	};
}

const handlerBrowser = {
	get: (
		target: LocalStorageProxy,
		key: keyof LocalStorageProxy
	): LocalStorageProxy[keyof LocalStorageProxy] => {
		if (typeof key !== 'string') {
			throw new Error('Tried to get key from localStorage with non-string key');
		}

		// Return the LsWritable from the saved state, if it exists.
		if (key in target) {
			return target[key];
		}

		// If it doesn't exist, find it from storage.
		let value: JsonParseable | null = null;
		if (dev) {
			value = sessionStorage.getItem(key);
		} else {
			value = localStorage.getItem(key);
		}
		if (value === null) value = JSON.stringify(null);

		// Create a writable and save it. Yes, even if it wasnt found!
		const w = writable(value);

		// Any time a change is made, save it to storage.
		// Notes about this:
		// In dev mode, saves to session because parsing localStorage is slow
		// and if doing a lot of things in it due to things like Fakers,
		// it makes for a terrible developing time!
		// In prod mode, saves to localStorage because it's persistant.
		w.subscribe((value) => {
			if (dev) {
				sessionStorage.setItem(key, value);
			} else {
				localStorage.setItem(key, value);
			}
		});

		const lsw = lsWritable(w);
		// No idea how to fix this TS error.
		// Essentially, we need a way to say `lsw as LocalStorageProxy[key]`,
		// but TS doesn't have a way of doing that
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore
		target[key] = lsw;
		return target[key];
	},
	set: (_target: LocalStorageProxy, _key: keyof LocalStorageProxy, _value: unknown): boolean => {
		// Make it impossible to use `storage[key] = value`
		// This is because trying to do so with TypeScript only allows for
		// setting with `value` as an LsWritable, and we DONT want to
		// override anything. Instead, we force users to use
		// `storage[key].set(value)`.
		return false;
	}
};

export interface LocalStorageProxy {
	totalXp: LsWritable<number>;
	bookmarks: LsWritable<Uuid[]>;
	ratings: LsWritable<Record<Uuid, number>>;
}

/**
 * `storage[key].get()` - Returns the value of `key` from localstorage.
 * `storage[key].set(value)` - Sets the value of `key` to `value`, setting `value` into localstorage via subcriber.
 * `storage[key].subscribe(callback)` - Subscribes to changes in `key`.
 */
const storage = !browser ? undefined : new Proxy({} as LocalStorageProxy, handlerBrowser);

export default storage;

import { dev } from '$app/env';

/** A response from the server. */
export type APIResponse<T> = APISuccessResponse<T> | APIErrorResponse;

/** A successful response from the server. */
export interface APISuccessResponse<T> {
	success: true;
	data: T;
	message?: string;
}

/** A failed response from the server. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export interface APIErrorResponse<T = any> {
	success: false;
	error?: {
		message?: string;
		data?: T;
	};
}

/** Returns the server address. */
export function getServerAddress(): URL {
	// If in development, return `localhost:8000/api/v1`
	if (dev) {
		// The slash at the end of the URL is important.
		// Also, having 'http://' is required else it doesn't work.

		// ```
		// // Without slash after 'v1', removes 'v1' from the URL.
		// let a = new URL('/api/v1', 'http://localhost:8000');
		// let b = new URL('weekly', a);
		// console.log(b.href); // http://localhost:8000/api/weekly
		//
		// // With slash before new URL, assumes relative to base URL (localhost:8000)
		// let c = new URL('/api/v1/', 'http://localhost:8000');
		// let d = new URL('/weekly', c);
		// console.log(d.href); // http://localhost:8000/weekly
		//
		// // Slash after 'v1' and no slash before 'weekly', works as intended.
		// let e = new URL('/api/v1/', 'http://localhost:8000');
		// let f = new URL('weekly', e);
		// console.log(f.href); // http://localhost:8000/api/v1/weekly
		// ```

		return new URL('/api/v1/', 'http://localhost:8000');
	} else {
		throw new Error("Don't know what the server address is in production");
	}
}

/** The valid HTTP request methods. */
export enum ReqMethod {
	GET = 'GET',
	HEAD = 'HEAD',
	POST = 'POST',
	PUT = 'PUT',
	DELETE = 'DELETE',
	CONNECT = 'CONNECT',
	OPTIONS = 'OPTIONS',
	TRACE = 'TRACE',
	PATCH = 'PATCH'
}

/** Interacts with the given API endpoint.
 *
 * Rejects with an error if the server returns an error or something that isn't
 * a generic API response type.
 *
 * Resolves with the response if the server returns a VALID API RESPONSE.
 * The caller must check themselves if the response is a success or error\
 * using the `success` property.
 *
 * Resolves: `APIResponse<T>`
 * Rejects: `APIErrorResponse`
 */
export async function req<T>(endpoint: string, req?: RequestInit): Promise<APIResponse<T>> {
	// Remove the leading slash from the endpoint
	endpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
	const url = new URL(endpoint, getServerAddress());

	// Check for a get or head request with a body.
	if ((req?.method === ReqMethod.GET || req?.method === ReqMethod.HEAD) && req?.body) {
		throw new Error('GET and HEAD requests cannot have a body');
	}

	const response: Response | false = await fetch(
		url,
		req ?? {
			headers: {
				'Content-Type': 'application/json'
			}
		}
	).catch(() => false);

	if (!response) {
		return Promise.reject({
			success: false,
			error: {
				message: 'Failed to connect to the server'
			}
		});
	}

	// Clone the response.
	// In case of an error, the text must be read, but the body cannot be read
	// twice from a response (in this case, json() and text()).
	// Therefore, we need to clone.
	// https://stackoverflow.com/questions/34786358/what-does-this-error-mean-uncaught-typeerror-already-read#comment86228774_35000918
	const response2 = response.clone();

	// Check for a successful response
	return response.json().then(
		(json: APIResponse<T>) => {
			return Promise.resolve(json);
		},
		() => {
			return Promise.reject({
				success: false,
				error: {
					message: 'Invalid JSON response',
					data: response2.text()
				}
			});
		}
	);
}

/** Make a GET request to a specific API endpoint. */
export async function get<T>(endpoint: string): Promise<APIResponse<T>> {
	return req(endpoint, {
		method: ReqMethod.GET,
		headers: {
			'Content-Type': 'application/json'
		}
	});
}

/** Make a POST request to a specific API endpoint. */
export async function post<T>(endpoint: string, body: BodyInit): Promise<APIResponse<T>> {
	return req(endpoint, {
		method: ReqMethod.POST,
		body
	});
}

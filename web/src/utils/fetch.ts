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
export interface APIErrorResponse {
	success: false;
	error?: {
		message?: string;
		data?: any;
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

/** Interacts with the given API endpoint. */
export async function req<T>(endpoint: string, req?: RequestInit): Promise<APIResponse<T>> {
	// Remove the leading slash from the endpoint
	endpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
	const url = new URL(endpoint, getServerAddress());

	// Check for a get or head request with a body.
	if ((req?.method === ReqMethod.GET || req?.method === ReqMethod.HEAD) && req?.body) {
		throw new Error('GET and HEAD requests cannot have a body');
	}

	const response = await fetch(
		url,
		req ?? {
			headers: {
				'Content-Type': 'application/json'
			}
		}
	);

	// Check for a successful response
	return response.json().then(
		(json) => {
			if (json.success) {
				return json;
			} else {
				throw json;
			}
		},
		() =>
			Promise.reject({
				success: false,
				error: {
					message: response.text()
				}
			})
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
export async function post<T>(endpoint: string, body: any): Promise<APIResponse<T>> {
	return req(endpoint, {
		method: ReqMethod.POST,
		body: JSON.stringify(body)
	});
}

import { json } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, fetch }) => {
	const apiUrl = env.API_URL ?? 'http://localhost:8080';

	try {
		const response = await fetch(new URL('/', apiUrl), {
			method: 'POST',
			headers: { 'Content-Type': request.headers.get('content-type') ?? 'application/json' },
			body: await request.text(),
			signal: AbortSignal.timeout(8_000)
		});

		return new Response(await response.arrayBuffer(), {
			status: response.status,
			headers: { 'Content-Type': response.headers.get('content-type') ?? 'application/json' }
		});
	} catch {
		return json({ errors: [{ message: 'The API is unavailable.' }] }, { status: 502 });
	}
};

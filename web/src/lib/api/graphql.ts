export type GraphqlFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

type GraphqlResponse<T> = {
	data?: T;
	errors?: { message: string }[];
};

export async function graphql<T>(
	fetcher: GraphqlFetch,
	query: string,
	variables: Record<string, unknown> = {}
): Promise<T> {
	const response = await fetcher('/api/graphql', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ query, variables })
	});
	const payload = (await response.json()) as GraphqlResponse<T>;

	if (!response.ok) {
		throw new Error(`API GraphQL indisponible (${response.status})`);
	}
	if (payload.errors?.length) {
		throw new Error(payload.errors.map((error) => error.message).join('\n'));
	}
	if (!payload.data) {
		throw new Error('La réponse GraphQL ne contient aucune donnée.');
	}

	return payload.data;
}

export async function graphqlOrFallback<T>(
	fetcher: GraphqlFetch,
	query: string,
	fallback: T,
	variables: Record<string, unknown> = {}
): Promise<{ value: T; usingFallback: boolean; error: string | null }> {
	try {
		return {
			value: await graphql<T>(fetcher, query, variables),
			usingFallback: false,
			error: null
		};
	} catch (error) {
		return {
			value: fallback,
			usingFallback: true,
			error: error instanceof Error ? error.message : 'Impossible de joindre l’API.'
		};
	}
}

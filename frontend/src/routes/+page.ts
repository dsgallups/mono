import type { apiFile } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const result = await fetch('/api/files');

	const body: apiFile[] = await result.json();

	return {
		files: body
	};
};

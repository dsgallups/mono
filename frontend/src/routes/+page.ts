import type { FileResponse } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, url }): Promise<{ files: FileResponse[] }> => {
	const apiUrl = new URL(url);
	apiUrl.pathname = '/api/files';
	const result = await fetch(apiUrl);

	const files: FileResponse[] = await result.json();

	return {
		files
	};
};

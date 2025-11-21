import type { FileResponse } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	const result = await fetch('/api/files');

	const body: FileResponse[] = await result.json();

	return {
		files: body
	};
};

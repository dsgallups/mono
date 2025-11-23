export type FileResponse = {
	id: number;
	title: string;
};

export type IndexResponse = {
	created_at: string;
	updated_at: string;
	id: number;
	path: string;
	progress: number;
	queue: string;
};

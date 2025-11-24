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
export type FileSimilarity = {
	id: number;
	file_type: FileType;
	title: string;
	path: string;
	chunks: FileChunk[];
};

type FileType = 'text' | 'jpeg' | 'unknown';

export type FileChunk = {
	id: number;
	content: string;
	similarity: number;
};

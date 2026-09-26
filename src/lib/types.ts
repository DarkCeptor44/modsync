export interface Profile {
	id: string;
	name: string;
	source: string;
	destination: string;
	syncExclusions: string[];
	deleteExclusions: string[];
}

export type ProfileInput = Omit<Profile, 'id'>;

export interface SyncOutcome {
	entry: string;
	action: SyncAction;
}

export type SyncAction =
	| { Copied: { bytes: number } }
	| { Removed: { bytes: number } }
	| { Failed: { error: string } }
	| 'Skipped';

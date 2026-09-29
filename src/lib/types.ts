/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

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
	| 'RemovedDir'
	| 'Skipped';

export interface Settings {
	jobs: number;
}

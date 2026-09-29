/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

export const DEV = import.meta.env.DEV;

export function humanBytes(bytes: number): string {
	const UNITS = ['B', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB'];

	let value = bytes;
	let index = 0;

	while (value >= 1024.0 && index < UNITS.length - 1) {
		value /= 1024.0;
		index++;
	}

	let precision = 0;
	if (value >= 100.0) {
		precision = 0;
	} else if (value >= 1.0) {
		precision = 1;
	} else if (value > 0.0) {
		precision = 2;
	}

	return `${value.toFixed(precision)} ${UNITS[index]}`;
}

export function generateId(): string {
	if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
		return crypto.randomUUID();
	}
	if (typeof crypto !== 'undefined' && typeof crypto.getRandomValues === 'function') {
		return (([1e7] as unknown as string) + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, (c) =>
			(
				Number(c) ^
				(crypto.getRandomValues(new Uint8Array(1))[0] & (15 >> (Number(c) / 4)))
			).toString(16)
		);
	}
	return Math.random().toString(36).substring(2, 11);
}

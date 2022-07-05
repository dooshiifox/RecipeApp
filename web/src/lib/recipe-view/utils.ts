/** Converts the timeToMake number into a string. */
export function toTimeString(timeToMake: number): string {
	const hours = Math.floor(timeToMake / 60);
	const minutes = timeToMake % 60;

	let hrString = ``;
	if (hours === 1) hrString = `${hours}hr`;
	else if (hours > 1) hrString = `${hours}hrs`;

	let minString = ``;
	if (minutes === 1) minString = `${minutes}min`;
	else if (minutes > 1) minString = `${minutes}mins`;

	// Trim removes space if hourString or minString is empty.
	return `${hrString} ${minString}`.trim();
}

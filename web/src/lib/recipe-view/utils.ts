/** Converts the timeToMake number into a string. */
export function toTimeString(timeToMake: number): string {
	const hours = Math.floor(timeToMake / 60);
	const minutes = timeToMake % 60;

	if (hours === 0) return `${minutes} mins`;

	if (minutes === 0) return `${hours} hrs`;

	return `${hours} hrs ${minutes} mins`;
}

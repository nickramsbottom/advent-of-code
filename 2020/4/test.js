function validateHeightEntry(heightEntry) {
	const regex = /^(?<value>\d+)(cm|in)$/;

	const matches = regex.exec(heightEntry);
	console.log(matches);

	if (matches === null) {
		return false;
	}

	const value = Number(matches[1]);
	const units = matches[2];

	if (units === 'cm') {
		return value >= 150 && value <= 193;
	}

	if (units === 'in') {
		return value >= 59 && value <= 76;
	}

	return false;
}

const entry = '168cm';

console.log(validateHeightEntry(entry));
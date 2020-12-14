const fs = require('fs');
const records = fs.readFileSync('./input.txt', 'utf8').split(/\n\n/);
const numberOfRecords = records.length;
console.log('numberOfRecords', numberOfRecords);

function parseRecord(record) {
	const parts = record.split(/[\n ]+/);
	return parts.reduce(
		(record, entry) => {
			[name, value] = entry.split(':');
			record[name] = value;
			return record;
		},
		{}
	);
}

function validateYear(yearEntry, from, to) {
	const regex = /^\d{4}$/;
	const validFormat = regex.test(yearEntry);

	if (!validFormat) {
		return false;
	}

	const year = Number(yearEntry);

	return yearEntry >= from && yearEntry <= to;
}

function validateHeightEntry(heightEntry) {
	const regex = /^(\d+)(cm|in)$/;

	const matches = regex.exec(heightEntry);

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

function validateHairColor(colorEntry) {
	const regex = /^#[\da-z]{6}$/;
	return regex.test(colorEntry);
}

function validateEyeColor(colorEntry) {
	return ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].indexOf(colorEntry) !== -1;
}

function validatePassportId(idEntry) {
	const regex = /^\d{9}$/;
	return regex.test(idEntry);
}

let validRecords = 0;
for(const record of records) {
	const {
		iyr,
//		cid,
		pid,
		eyr,
		hcl,
		ecl,
		byr,
		hgt
	} = parseRecord(record);

	const passId = validatePassportId(pid);
	const eyeCol = validateEyeColor(ecl);
	const hairCol = validateHairColor(hcl);
	const height = validateHeightEntry(hgt);
	const birth = validateYear(byr, 1920, 2002);
	const issue = validateYear(iyr, 2010, 2020);
	const expire = validateYear(eyr, 2020, 2030);

	if (passId && eyeCol && hairCol && height && birth && issue && expire) {
		validRecords++
	}
}

console.log(validRecords);

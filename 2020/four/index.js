const fs = require('fs');
const records = fs.readFileSync('./input.txt', 'utf8').split(/\n\n/);

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

	if (iyr && pid && eyr && hcl && ecl && byr && hgt) {
		validRecords++
	}
}

console.log(validRecords);

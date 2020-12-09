const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(string => Number(string));

const preambleLength = 25;
// const invalidEntry = 14360655;

function findInvalidNumber() {
	for(let i = preambleLength; i < input.length; i++) {
		const preamble = input.slice((i - preambleLength), i);
		const number = Number(input[i]);

		if (!complementInArray(number, preamble)) {
			return number;
		}
	}
}

function complementInArray(target, numbers) {
	const differences = new Set();
	
	for (const number of numbers) {
		if (differences.has(number)) {
			return true;
		}

		const diff = target - number;
		differences.add(diff);
	}

	return false;
}

console.log(findInvalidNumber());

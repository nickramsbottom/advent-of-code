const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(string => Number(string));

const preambleLength = 25;
const invalidEntry = 14360655;

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

// console.log(findInvalidNumber());

const numbers = [
	714065,
	715497,
	776112,
	802598,
	749876,
	841964,
	776204,
	835117,
	823680,
	844572,
	851440,
	830638,
	846284,
	906078,
	1248266,
	880860,
	917404,
];

function findSlice() {
	for (let i = 0; i < input.length; i++) {
		const currentSlice = input.slice(i);

		let sum = 0;
		for (let j = 0; j < currentSlice.length; j++) {
			const number = currentSlice[j];
			sum += number;

			if (sum === invalidEntry) {
				return {
					from: i,
					to: i + j,
				}
			}

			if (sum > invalidEntry) {
				break;
			}
		}
	}
}

function findSum() {
	const { from, to } = findSlice();

	const slice = input.slice(from, to);
	slice.sort((a, b) => a - b);

	const sum = slice[0] + slice[slice.length - 1];

	console.log(sum);
}

findSum();

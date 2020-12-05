const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

const exampleCode = ['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'];

function calculateRow(code) {
	let higherIndex = Math.pow(2, code.length) - 1;
	let lowerIndex = 0;

	for (const char of code) {
		const length = higherIndex - lowerIndex;
		const distance = (length - 1) / 2;

		if (char === 'F' || char === 'L') {
			higherIndex -= (distance + 1);
		}

		if (char === 'B' || char === 'R') {
			lowerIndex += (distance + 1);
		}
	}

	return lowerIndex;
}

function calculateSeatId(barcode) {
	const rowPortion = barcode.slice(0, 7);
	const seatPortion = barcode.slice(7);

	return (8 * calculateRow(rowPortion)) + calculateRow(seatPortion);
}

const highestSeatId = input.reduce(
	(highestId, code) => {
		const newSeatId = calculateSeatId(code);

		if (newSeatId > highestId) {
			highestId = newSeatId;
		}

		return highestId;
	},
	0
);

const seatIds = input.map(calculateSeatId).sort((a, b) => a - b);

const lowestId = seatIds[1];
const highestId = seatIds[seatIds.length - 1];

let predictedId = lowestId;
for (let i = 1; i < seatIds.length; i++) {
	if (predictedId !== seatIds[i]) {
		console.log(predictedId);
		break;
	}
	predictedId++;
}

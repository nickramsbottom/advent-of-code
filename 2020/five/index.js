const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

const exampleCode = ['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'].join('');

function calculateSeatId(barcode) {
	const binary = barcode
		.replaceAll('F', 0)
		.replaceAll('B', 1)
		.replaceAll('R', 1)
		.replaceAll('L', 0);

	return parseInt(binary, 2);
}

console.log(calculateSeatId(exampleCode));

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

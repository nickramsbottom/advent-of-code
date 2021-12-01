const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(n => Number(n));

let firstMeasurement = input[0];
let secondMeasurement = input[1];
let lastSum = input[0] + input[1] + input[2]
let count = 0;

for (let i = 3; i < input.length; i++) {
	firstMeasurement = input[i-2];
	secondMeasurement = input[i-1];
	const thisMeasurement = input[i];
	const sum = firstMeasurement + secondMeasurement + thisMeasurement;

	if (sum > lastSum) {
		count++
	}

	lastSum = sum;
}

console.log(count);
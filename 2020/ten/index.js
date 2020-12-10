const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(n => Number(n))
	.sort((a, b) => a - b);

input.unshift(0);

const differences = {
	1: 0,
	2: 0,
	3: 1,
};

for (let i = 0; i < input.length - 1; i++) {
	const thisAdapter = input[i];
	const nextAdapter = input[i + 1];

	const difference = nextAdapter - thisAdapter;

	differences[difference]++
}

console.log(differences['1'] * differences['3']);
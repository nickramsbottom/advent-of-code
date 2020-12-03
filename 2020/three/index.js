const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n').map(row => row.split(''));

let x = 0;
let y = 0;

const xInc = 3;
const yInc = 1;

const yLength = input.length;
const xLength = input[0].length;

const treeChar = '#';

let count = 0;
let call = 0;
while(y < yLength) {
	call += 1;
	const char = input[y][x];

	if (char === treeChar) {
		count += 1;
	}

	y += yInc;
	x = (x + 3) % xLength;
}

console.log(count);
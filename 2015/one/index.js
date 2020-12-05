const fs = require('fs');

const input = fs.readFileSync('./input.txt', 'utf8');

const up = input.match(/\(/g).length;
const down = input.match(/\)/g).length;

console.log(up);
console.log(down);
console.log(up - down);

const array = input.split('');

let floor = 0;

for (let i = 0; i < array.length; i++) {
	const instruction = array[i];

	if (instruction === '(') {
		floor ++;
	}

	if (instruction === ')') {
		floor --;
	}

	if (floor < 0) {
		console.log(i + 1);
		break;
	}
}
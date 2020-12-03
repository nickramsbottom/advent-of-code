const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n').map(row => row.split(''));

function countTrees(right, down) {
	let x = 0;
	let y = 0;

	const xInc = right;
	const yInc = down;

	const yLength = input.length;
	const xLength = input[0].length;

	const treeChar = '#';

	let count = 0;
	while(y < yLength) {
		const char = input[y][x];

		if (char === treeChar) {
			count += 1;
		}

		y += yInc;
		x = (x + right) % xLength;
	}

	return count;
}

console.log(countTrees(3, 1));

const slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

const allSlopes = slopes.reduce(
	(product, [right, down]) => {
		return product * countTrees(right, down);
	},
	1
);

console.log(allSlopes);
// Part one = 2275
// 94
// Part two = 2121
// 86

const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n').map(row => row.split(''));

let currentLayout = JSON.parse(JSON.stringify(input));
let newLayout;

let searching = true;
while (searching) {
	searching = false;
	newLayout = [];

	for (let i = 0; i < currentLayout.length; i++) {
		newLayout.push([]);
		const row = currentLayout[i];
		const rowLength = row.length;

		for (let j = 0; j < row.length; j++) {
			const seat = currentLayout[i][j];
			if (seat === '.') {
				newLayout[i].push('.');
				continue;
			}

			let adjacentOccupied = [];

			adjacentOccupied.push(currentLayout[i-1] && currentLayout[i-1][j-1]);
			adjacentOccupied.push(currentLayout[i-1] && currentLayout[i-1][j]);
			adjacentOccupied.push(currentLayout[i-1] && currentLayout[i-1][j+1]);

			adjacentOccupied.push(currentLayout[i] && currentLayout[i][j-1]);
			adjacentOccupied.push(currentLayout[i] && currentLayout[i][j+1]);

			adjacentOccupied.push(currentLayout[i+1] && currentLayout[i+1][j-1]);
			adjacentOccupied.push(currentLayout[i+1] && currentLayout[i+1][j]);
			adjacentOccupied.push(currentLayout[i+1] && currentLayout[i+1][j+1]);

			const occupiedSeats = adjacentOccupied.filter(char => char === '#').length;

			if (seat === 'L' && occupiedSeats === 0) {
				searching = true;
				newLayout[i].push('#');
				continue;
			}

			if (seat === '#' && occupiedSeats  >= 4) {
				searching = true;
				newLayout[i].push('L');
				continue;
			}

			newLayout[i].push(currentLayout[i][j]);
		}
	}

	currentLayout = newLayout;
}

const singleString = currentLayout.map(row => row.join('')).join('');
const numberOccupied = (singleString.match(/#/g) || []).length;
console.log(numberOccupied);
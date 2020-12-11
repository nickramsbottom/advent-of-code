const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n').map(row => row.split(''));

let currentLayout = JSON.parse(JSON.stringify(input));
let newLayout;

function checkSeatsPart1(grid, i, j) {
	let adjacentOccupied = [];

	adjacentOccupied.push(grid[i-1] && grid[i-1][j-1]);
	adjacentOccupied.push(grid[i-1] && grid[i-1][j]);
	adjacentOccupied.push(grid[i-1] && grid[i-1][j+1]);

	adjacentOccupied.push(grid[i] && grid[i][j-1]);
	adjacentOccupied.push(grid[i] && grid[i][j+1]);

	adjacentOccupied.push(grid[i+1] && grid[i+1][j-1]);
	adjacentOccupied.push(grid[i+1] && grid[i+1][j]);
	adjacentOccupied.push(grid[i+1] && grid[i+1][j+1]);

	return adjacentOccupied.filter(char => char === '#').length;
}

function checkSeatsPart2(grid, i, j){
	let occupiedSeats = 0;

	for (let y = i-1; y > -1; y--) {
		if (grid[y][j]!=='.') {
			grid[y][j]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let y = i+1; y < grid.length; y++) {
		if (grid[y][j]!=='.') {
			grid[y][j]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j+1; x < grid[0].length; x++) {
		if (grid[i][x]!=='.') {
			grid[i][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j-1; x > -1; x--) {
		if (grid[i][x]!=='.') {
			grid[i][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j-1, y = i-1; x > -1, y > -1; x--, y--) {
		if (grid[y][x]!=='.') {
			grid[y][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j+1, y = i-1; x < grid[0].length, y > -1; x++, y--) {
		if (grid[y][x]!=='.') {
			grid[y][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j+1, y = i+1; x < grid[0].length, y < grid.length; x++, y++) {
		if (grid[y][x]!=='.') {
			grid[y][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	for (let x = j-1, y = i+1; x > -1, y < grid.length; x--, y++) {
		if (grid[y][x]!=='.') {
			grid[y][x]=='#' ? occupiedSeats++ : undefined;
			break;
		}
	}

	return occupiedSeats;
}

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

			const occupiedSeats = checkSeatsPart2(currentLayout, i, j);

			if (seat === 'L' && occupiedSeats === 0) {
				searching = true;
				newLayout[i].push('#');
				continue;
			}

			if (seat === '#' && occupiedSeats  >= 5) {
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

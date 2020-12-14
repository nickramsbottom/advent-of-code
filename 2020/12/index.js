const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map((instruction) =>{
		const regex = /^(?<action>[A-Z])(?<value>\d{1,3})$/;
		const groups = regex.exec(instruction).groups;

		return {
			action: groups.action,
			value: Number(groups.value),
		}
	});

// N, E
let movement = [0, 0];

// N, E
let waypointPosition = [1, 10];

for (const instruction of input) {
	const { action, value } = instruction;

	switch(action) {
		case 'F':
			movement[0] += waypointPosition[0] * value;
			movement[1] += waypointPosition[1] * value;
			break;
		case 'N':
			waypointPosition[0] += value;
			break;
		case 'S':
			waypointPosition[0] -= value;
			break;
		case 'E':
			waypointPosition[1] += value;
			break;
		case 'W':
			waypointPosition[1] -= value;
			break;
		case 'L':
			switch(value) {
				case 90:
					waypointPosition = [waypointPosition[1], -waypointPosition[0]];
					break;
				case 180:
					waypointPosition = [-waypointPosition[0], -waypointPosition[1]];
					break;
				case 270:
					waypointPosition = [-waypointPosition[1], waypointPosition[0]];
					break;
			}
			break;
		case 'R':
			switch(value) {
				case 90:
					waypointPosition = [-waypointPosition[1], waypointPosition[0]];
					break;
				case 180:
					waypointPosition = [-waypointPosition[0], -waypointPosition[1]];
					break;
				case 270:
					waypointPosition = [waypointPosition[1], -waypointPosition[0]];
					break;
			}
			break;
	}
}

console.log(Math.abs(movement[0]) + Math.abs(movement[1]));
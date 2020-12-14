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

const movement = {
	N: 0,
	S: 0,
	E: 0,
	W: 0,
};

const translation = {
	0: 'N',
	90: 'E',
	180: 'S',
	270: 'W',
}

let degrees = 90;

for (const instruction of input) {
	const { action, value } = instruction;

	if (movement[action] !== undefined) {
		movement[action] += value;
		continue;
	}

	if (action === 'L') {
		degrees = (degrees - value) % 360;

		while (degrees < 0) {
			degrees += 360;
		}

		continue;
	}

	if (action === 'R') {
		degrees = (degrees + value) % 360;

		while (degrees < 0) {
			degrees += 360;
		}

		continue;
	}

	if (action === 'F') {
		const direction = translation[degrees];
		movement[direction] += value;
		continue;
	}
}

const result = Math.abs(movement.N - movement.S) + Math.abs(movement.E  - movement.W);
console.log(result);

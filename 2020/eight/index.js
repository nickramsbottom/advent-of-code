const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(instruction => {
		const [command, value] = instruction.split(' ');
		return {
			command,
			value: Number(value),
			timesExecuted: 0,
		};
	});

let accumulator = 0;
let index = 0;
while(true) {
	const instruction = input[index];

	if(instruction.timesExecuted > 0) {
		console.log(accumulator);
		break;
	}

	instruction.timesExecuted++;
	const command = instruction.command;
	if (command === 'acc') {
		accumulator += instruction.value;
		index++;
		continue;
	}

	if (command === 'jmp') {
		index += instruction.value;
		continue;
	}

	if (command === 'nop') {
		index++;
		continue;
	}
}
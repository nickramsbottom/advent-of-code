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

function computeAccumulator(instructions) {
	let accumulator = 0;
	let index = 0;

	while(true) {
		const instruction = instructions[index];

		if (index === input.length) {
			return accumulator;
		}

		if (instruction.timesExecuted > 0) {
			throw Error(`in a loop, accumulator was: ${accumulator}`);
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
}

const toTry = [];
for (let i = 0; i < input.length; i++) {
	const instruction = input[i];
	if (instruction.command === 'acc') {
		continue;
	}

	const inputCopy = JSON.parse(JSON.stringify(input));

	if (instruction.command === 'jmp') {
		inputCopy[i].command = 'nop';
	}

	if (instruction.command === 'nop') {
		inputCopy[i].command = 'jmp';
	}

	toTry.push(inputCopy);
}

for (let i = 0; i < toTry.length; i++) {
	try {
		console.log(computeAccumulator(toTry[i]));
		break;
	} catch (e) {
		continue;
	}
}

// console.log(computeAccumulator(input));
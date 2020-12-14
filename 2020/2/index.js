const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

function parseInputLine(inputLine) {
	const [range, char, password] = inputLine.split(' ');
	const [lower, upper] = range.split('-');

	return {
		lower: Number(lower),
		upper: Number(upper),
		password,
		char: char[0],
	}
}

function isPasswordCompliant({lower, upper, password, char}) {
	const charOne = password.charAt(lower - 1);
	const charTwo = password.charAt(upper - 1);

	if (charOne === char && charTwo === char) {
		return false;
	}

	return charOne === char || charTwo === char;
}

// const parsed = parseInputLine(input);
// console.log(parsed);

// const valid = isPasswordCompliant(parsed);
// console.log(valid);

const numberValidLines = input.reduce(
	(numberValid, line) => {
		const parsedInput = parseInputLine(line);
		const isValid = isPasswordCompliant(parsedInput);

		if (isValid) {
			numberValid += 1
		}

		return numberValid
	},
	0
);

console.log(numberValidLines);
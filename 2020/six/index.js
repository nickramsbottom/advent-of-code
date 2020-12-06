const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n\n').map(group => group.split('\n'));

let count = 0;
for (const group of input) {
	const seenAnswers = {};
	for (const answers of group) {
		const chars = answers.split('');
		for (const char of chars) {
			seenAnswers[char] = true;
		}
	}
	count += Object.keys(seenAnswers).length;
}
console.log(count);
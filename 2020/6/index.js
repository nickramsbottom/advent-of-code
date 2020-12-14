const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n\n').map(group => group.split('\n'));

let count = 0;
for (const group of input) {
	const seenAnswers = {};
	for (const answers of group) {
		const chars = answers.split('');
		for (const char of chars) {
			if (seenAnswers[char]) {
				seenAnswers[char]++;
			} else {
				seenAnswers[char] = 1;
			}
		}
	}

	const sizeOfGroup = group.length;
	const answerKeys = Object.keys(seenAnswers);

	for (const key of answerKeys) {
		if(seenAnswers[key] === sizeOfGroup) {
			count++;
		}
	}
}
console.log(count);
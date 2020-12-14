const fs = require('fs');
const input = fs
	.readFileSync('./input.txt', 'utf8')
	.split('\n')
	.map(n => Number(n))
	.sort((a, b) => a - b);

input.unshift(0);
input.push(input[input.length - 1] + 3);

// any adapter can take 1, 2 or 3 below its rating
// and still produce its output
// device can take 3 higher than highest-rated adapter
// outlet is at 0

const calculatedPaths = {};
function calculatePermutations(i) {
	if (i === input.length - 1) {
		return 1;
	}

	if (calculatedPaths[i]) {
		return calculatedPaths[i];
	}

	let combinations = 0;
	for (let j = i + 1; j < input.length; j++) {
		if (input[j] - input[i] <= 3) {
			combinations += calculatePermutations(j);
		}
	}
	calculatedPaths[i] = combinations;
	return combinations;
}
console.log(calculatePermutations(0));

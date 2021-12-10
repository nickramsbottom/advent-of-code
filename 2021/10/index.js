const fs = require('fs');

const input = fs.readFileSync('./input.txt', 'utf8')
	
const lines = input.split('\n').map(line => line.split(''));

const score = lines.reduce(
    (score, line) => score += scoreLine(line),
    0
);

console.log(score);

function scoreLine(line) {
    const stack = [];

    const complements = {
        '}': '{',
        '>': '<',
        ']': '[',
        ')': '(',
    }

    const scores = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137,
    }

    for (char of line) {
        if (!complements[char]) {
            stack.push(char);
        } else {
            const last = stack.pop();
            if (last !== complements[char]) {
                return scores[char];
            }
        }
    }

    return 0;
}


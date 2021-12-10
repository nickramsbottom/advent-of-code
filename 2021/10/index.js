const fs = require('fs');

const input = fs.readFileSync('./input.txt', 'utf8')
	
const lines = input.split('\n').map(line => line.split(''));

const complements = {
    '}': '{',
    '>': '<',
    ']': '[',
    ')': '(',
}

const score = lines.reduce(
    (score, line) => score += scoreLine(line),
    0
);

const incompleteLines = lines.filter(line => scoreLine(line) === 0);
const scores = incompleteLines
    .map(line => completeLineScore(line))
    .sort((a, b) => a - b);

console.log(scores[(scores.length + 1) / 2 - 1]);

function completeLineScore(line) {
    const stack = [];

    for (char of line) {
        if (!complements[char]) {
            stack.push(char);
        } else {
            stack.pop();
        }
    }

    const scoresTable = {
        '(': 1,
        '[': 2,
        '{': 3,
        '<': 4,
    }

    let score = 0;
    let i = 0;
    while (stack.length > 0) {
        const char = stack.pop();
        score = (score * 5) + scoresTable[char];
        i++;
    }

    return score;
}

function scoreLine(line) {
    const stack = [];

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


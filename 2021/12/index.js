const fs = require('fs');
const input = fs
    .readFileSync('./input.txt', 'utf8')
    .split('\n')
    .map(line => line.split('-'));


const graph = {};

for (line of input) {
    const [firstCave, secondCave] = line;
    if (!graph[firstCave]) {
        graph[firstCave] = [];
    }
    graph[firstCave].push(secondCave);
}

console.log(graph);
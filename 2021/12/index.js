const fs = require('fs');
const { setEnvironmentData } = require('worker_threads');
const input = fs
    .readFileSync('./input.txt', 'utf8')
    .split('\n')
    .map(line => line.split('-'));

console.log(part1(createGraph(input)));

function createGraph(relations) {
    const graph = {};

    for (line of relations) {
        const [firstCave, secondCave] = line;
        if (!graph[firstCave]) {
            graph[firstCave] = [];
        }

        if (!graph[secondCave]) {
            graph[secondCave] = [];
        }

        graph[firstCave].push(secondCave);
        graph[secondCave].push(firstCave);
    }

    return graph;
}

function part1(graph) {
    const caves = createGraph(input);
    
    function depthFirstSearch(cave, visited) {
        if (cave === 'end') {
            return 1;
        }

        if (visited.has(cave) && isSmallCave(cave)) {
            return 0;
        }

        let result = 0;

        visited.add(cave);

        for (const nextCave of caves[cave]) {
            result += depthFirstSearch(nextCave, visited);
        }

        visited.delete(cave);

        return result;
    }

    return depthFirstSearch('start', new Set());
}

function isSmallCave(cave) {
    return /[a-z]/.test(cave);
}
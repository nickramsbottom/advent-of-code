const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

const [estimate, busIds] = input;

const buses = busIds
	.split(',')
	.filter(id => id !== 'x')
	.map(id => Number(id));

const waitTimes = buses.map(id => id - estimate%id);

const smallestWait = Math.min(...waitTimes);
const indexOfSmallest = waitTimes.indexOf(smallestWait);

const idWait = smallestWait * buses[indexOfSmallest];
console.log(idWait);
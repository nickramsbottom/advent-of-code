const fs = require('fs');
const input = fs.readFileSync('./input.txt', 'utf8').split('\n');

const [estimate, busIds] = input;

const buses = busIds
	.split(',')
	.reduce(
		(list, id, i) => {

			if (id !== 'x') {
				list.push({
					id: Number(id),
					offset: i,
				})
			}
			return list;
		},
		[]
	);

// const waitTimes = buses.map(id => id - estimate%id);

// const smallestWait = Math.min(...waitTimes);
// const indexOfSmallest = waitTimes.indexOf(smallestWait);

// const idWait = smallestWait * buses[indexOfSmallest];
// console.log(idWait);

const [firstBus, ...otherBuses] = buses;

let multiplier = firstBus.id;
let i = 0;

for (const bus of otherBuses) {
	while(true) {
		const { id, offset } = bus;

		if ((i + offset) % id === 0) {
			multiplier *= id;
			break;
		}

		i += multiplier;
	}
}

console.log(i);

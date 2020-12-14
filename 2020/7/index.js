const fs = require('fs');
const rules = fs.readFileSync('./input.txt', 'utf8').split('\n');

const parsedRules = rules.reduce(
	(dictionary, rule) => {
		const [bagName, inside] = rule
			.replaceAll('.', '')
			.replaceAll(' bags', '')
			.replaceAll(' bag', '')
			.split(' contain ');

		const bagCount = inside.split(', ');

		dictionary[bagName] = inside
			.split(', ')
			.map(bag => {
				if (bag === 'no other') {
					return 'no other';
				}

				const number = Number(bag[0]);
				const name = bag.slice(2);

				return {
					name,
					number,
				}
			})
			.filter(bag => bag !== 'no other');

		return dictionary;
	},
	{}
);

function countBagType(bagName, bagType) {
	if (bagName === bagType) {
		return 0;
	};

	const bagsInside = parsedRules[bagName];

	let bagCount = 0;
	for (const bag of bagsInside) {
		if (bag.name === bagType) {
			bagCount += 1;
		} else {
			bagCount += countBagType(bag.name, bagType);
		}
	}

	return bagCount;
};

const bagNames = Object.keys(parsedRules);

const canContain = bagNames
	.map(name => countBagType(name, 'shiny gold'))
	.filter(count => count !== 0)
	.length;

console.log(canContain);

function countNumberOfBagsInside(bagName) {
	const bagsInside = parsedRules[bagName];

	let bagCount = 0;
	for (const bag of bagsInside) {
		bagCount += bag.number + bag.number * countNumberOfBagsInside(bag.name);
	}
	return bagCount;
};

console.log(countNumberOfBagsInside('shiny gold'));
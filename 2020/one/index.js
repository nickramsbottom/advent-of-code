const numbers = require('./numbers');

function productForPair(target) {
	const differences = new Set();

	for (let i = 0; i < numbers.length; i++) {
		const number = numbers[i];
		const diff = target - number;
		if (differences.has(number)) {
			return number * diff;
		}

		differences.add(diff);
	}

	return false;
}

function productForTriplet(target) {
	for (let i = 0; i < numbers.length; i++) {
		const number = numbers[i];
		const productOfPair = productForPair(2020 - number);

		if (productOfPair !== false) {
			return number * productOfPair;
		} 
	}
}

console.log(productForPair(2020));
console.log(productForTriplet(2020));
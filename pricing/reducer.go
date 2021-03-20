package pricing

func Reduce(priceHistory []Price) Price {
	accumulator := NewPrice(0.00)
	for _, price := range priceHistory {
		accumulator = reduceTwo(accumulator, price)
	}
	return accumulator
}

func reduceTwo(priceA Price, priceB Price) Price {
	multiplierA := percentToMultiplier(priceA.PercentDelta())
	multiplierB := percentToMultiplier(priceB.PercentDelta())
	// fmt.Printf("A : %v - %v, B: %v - %v\n", priceA, multiplierA, priceB, multiplierB)
	updatedPercentDelta := multiplerToPercent(multiplierA * multiplierB)
	return priceA.WithPrice(updatedPercentDelta)
}

func percentToMultiplier(percent float64) float64 {
	return (percent / 100.0) + 1
}

func multiplerToPercent(multipler float64) float64 {
	return (multipler - 1) * 100.0
}

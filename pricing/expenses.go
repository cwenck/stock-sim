package pricing

func ExpenseRatio(priceHistory []Price, expenseRatio float64) (result []Price) {
	for _, price := range priceHistory {
		percentDelta := price.PercentDelta() * (1 - (expenseRatio / 100))
		result = append(result, price.WithPrice(percentDelta))
	}
	return
}

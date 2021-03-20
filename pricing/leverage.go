package pricing

func Leverage(priceHistory []Price, leverage float64) (result []Price) {
	for _, price := range priceHistory {
		percentDelta := price.PercentDelta() * leverage
		result = append(result, price.WithPrice(percentDelta))
	}
	return
}

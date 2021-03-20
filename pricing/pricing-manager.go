package pricing

type PricingManger struct {
	pricingStrategy PricingStrategy
}

func NewPricingManger(pricingStrategy PricingStrategy) PricingManger {
	return PricingManger{pricingStrategy}
}

func (m PricingManger) CalculatePrices(fromPeriod uint, toPeriod uint) []Price {
	var priceHistory []Price
	for period := fromPeriod; period < toPeriod; period++ {
		price := m.pricingStrategy.CalculatePrice(period, priceHistory)
		priceHistory = append(priceHistory, price)
	}
	return priceHistory
}

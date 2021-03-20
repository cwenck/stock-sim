package pricing

type alternatingPricingStrategy struct {
	pricingOptions []Price
}

func NewAlternatingPricingStrategy(pricingOptions []Price) PricingStrategy {
	pricingOptionsCopy := make([]Price, len(pricingOptions))
	copy(pricingOptionsCopy, pricingOptions)
	return alternatingPricingStrategy{pricingOptionsCopy}
}

func (s alternatingPricingStrategy) CalculatePrice(period uint, priceHistory []Price) Price {
	choice := period % uint(len(s.pricingOptions))
	return s.pricingOptions[choice].Clone()
}

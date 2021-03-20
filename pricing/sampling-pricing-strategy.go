package pricing

import (
	"math/rand"
	"time"
)

type samplingPricingStrategy struct {
	pricingOptions []Price
	rng            *rand.Rand
}

func NewSamplingPricingStrategy(pricingOptions []Price) PricingStrategy {
	rngSource := rand.NewSource(time.Now().UnixNano())
	pricingOptionsCopy := make([]Price, len(pricingOptions))
	copy(pricingOptionsCopy, pricingOptions)

	return samplingPricingStrategy{
		pricingOptions: pricingOptionsCopy,
		rng:            rand.New(rngSource),
	}
}

func (s samplingPricingStrategy) CalculatePrice(period uint, priceHistory []Price) Price {
	choice := s.rng.Intn(len(s.pricingOptions))
	return s.pricingOptions[choice].Clone()
}

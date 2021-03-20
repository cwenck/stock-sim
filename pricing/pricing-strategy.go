package pricing

import "fmt"

type PricingStrategy interface {
	CalculatePrice(period uint, priceHistory []Price) Price
}

type Price interface {
	PercentDelta() float64
	WithPrice(price float64) Price
	Add(price Price) Price
	Clone() Price
}

type basicPrice struct {
	percentDelta float64
}

func (p basicPrice) PercentDelta() float64 {
	return p.percentDelta
}

func (p basicPrice) WithPrice(percentDelta float64) Price {
	return basicPrice{percentDelta}
}

func (p basicPrice) Add(price Price) Price {
	return basicPrice{p.percentDelta + price.PercentDelta()}
}

func (p basicPrice) Clone() Price {
	return basicPrice{p.percentDelta}
}

func (p basicPrice) String() string {
	return fmt.Sprintf("%0.3f%%", p.PercentDelta())
}

func NewPrice(percentDelta float64) Price {
	return basicPrice{percentDelta}
}

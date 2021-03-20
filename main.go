package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"

	"github.com/cwenck/stock-sim/pricing"
	"github.com/cwenck/stock-sim/stats"
)

const MARKET_DAYS = 253
const SIMULATIONS = 100000
const WORKER_COUNT = 8
const BUFFER_SIZE = WORKER_COUNT * 10

func createResult(priceHistory []pricing.Price) stats.SimulationResult {
	history1x := pricing.ExpenseRatio(priceHistory, dailyRate(0.03))
	history2x := pricing.ExpenseRatio(pricing.Leverage(priceHistory, 2.0), dailyRate(0.91))

	results := [2]pricing.Price{
		pricing.Reduce(history1x),
		pricing.Reduce(history2x),
	}

	return stats.NewSimulationResult(results)
}

func generatePriceHistory(priceOptions []pricing.Price, input <-chan int, output chan<- stats.SimulationResult) {
	pricingStrategy := pricing.NewSamplingPricingStrategy(priceOptions)
	pricingManger := pricing.NewPricingManger(pricingStrategy)

	for range input {
		priceHistory := pricingManger.CalculatePrices(0, MARKET_DAYS*30)
		output <- createResult(priceHistory)
	}
}

func generateInputs(input chan<- int) {
	for simulationNumber := 0; simulationNumber < SIMULATIONS; simulationNumber++ {
		input <- simulationNumber
	}
	close(input)
}

func runSimulations() []SimulationResult {
	priceOptions := readPrices()
	input := make(chan int, BUFFER_SIZE)
	output := make(chan SimulationResult, BUFFER_SIZE)
	var simulationResults []SimulationResult

	go generateInputs(input)
	for workerNumber := 0; workerNumber < WORKER_COUNT; workerNumber++ {
		go generatePriceHistory(priceOptions, input, output)
	}

	for resultCount := 0; resultCount < SIMULATIONS; resultCount++ {
		simulationResults = append(simulationResults, <-output)
		if resultCount%1000 == 0 {
			fmt.Printf("\rProgress: %d/%d", resultCount, SIMULATIONS)
		}
	}
	close(output)
	fmt.Printf("\rProgress: %d/%d\n", SIMULATIONS, SIMULATIONS)

	return simulationResults
}

func runStats(simulationResults []SimulationResult) {

}

func main() {
	simulationResults := runSimulations()

}

func dailyRate(rate float64) float64 {
	return rate / float64(MARKET_DAYS)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func readPrices() (result []pricing.Price) {
	f, err := os.Open("resources/daily-changes.csv")
	check(err)
	defer f.Close()

	notAsciiRegex := regexp.MustCompile(`[^a-zA-Z0-9\-+.$%]+`)
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		text := notAsciiRegex.ReplaceAllString(scanner.Text(), "")
		parsed, err := strconv.ParseFloat(text, 64)
		check(err)
		parsed *= 100
		// fmt.Printf("%v : %0.08f\n", text, parsed)
		result = append(result, pricing.NewPrice(parsed))
	}

	return
}

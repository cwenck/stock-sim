package stats

import "github.com/cwenck/stock-sim/pricing"

const RESULT_COUNT = 2

type Stats interface {
	
}

type stats struct {
	 
}

type SimulationResult struct {
	results [RESULT_COUNT]pricing.Price
}

type AverageResult struct {
	results [RESULT_COUNT]pricing.Price
	count   uint
}

func NewSimulationResult(results [RESULT_COUNT]pricing.Price) SimulationResult {
	return SimulationResult{results}
}

func AverageReturn(simulationResults []SimulationResult) AverageResult {
	input := make(chan SimulationResult)
	output := 
	go simulationResultProvider()
}

func simulationResultProvider(simulationResults []SimulationResult, input chan<- SimulationResult) {
	for _, result := range simulationResults {
		input <- result
	}
	close(input)
}

func averageingWorker(input <-chan SimulationResult, output chan<- AverageResult) AverageResult {
	accumulator := AverageResult{count: 0}
	for i := 0; i < RESULT_COUNT; i++ {
		accumulator.results[i] = pricing.NewPrice(0.0)
	}

	for result := range input {
		for i := 0; i < RESULT_COUNT; i++ {
			accumulator.results[i] = accumulator.results[i].Add(result.results[i])
		}
		accumulator.count++
	}

	return accumulator
}

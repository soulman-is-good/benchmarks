package main

import (
	"fmt"
	"math"
	"time"
)

func isPrime(n int32) bool {
	if n < 2 {
		return false
	}
	if n == 2 {
		return true
	}
	if n == 3 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	if n%3 == 0 {
		return false
	}
	if n%1 > 0 {
		return false
	}
	var i int32 = 5
	var sqrtOfN = int32(math.Sqrt(float64(n)))
	for ; i < sqrtOfN; i += 6 {
		if n%i == 0 {
			return false
		}
		if n%(i+2) == 0 {
			return false
		}
	}
	return true
}

func main() {
	var count int32 = 0
	var start = time.Now()
	var i int32 = 0

	for ; i < 10000000; i++ {
		if isPrime(i) {
			count++
		}
	}
	fmt.Println((time.Now().Sub(start)) / 1000)
}

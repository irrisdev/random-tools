package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func main() {
	if len(os.Args) < 2 || len(os.Args) > 3 {
		fmt.Println("Usage: date_calc <days> [--date-only]")
		fmt.Println("  <days>        Number of days from today to calculate the target date.")
		fmt.Println("  --date-only   Optional flag to display only the date.")
		return
	}

	days, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Error: <days> must be a valid integer.")
		return
	}

	targetDate := time.Now().AddDate(0, 0, days).Format("02/01/2006")

	if len(os.Args) == 3 && os.Args[2] == "--date-only" {
		fmt.Println(targetDate)
	} else {
		fmt.Printf("Date in %d days: %s\n", days, targetDate)
	}
}
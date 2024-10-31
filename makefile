.PHONY: build clean date-calc

build: date-calc

date-calc:
	@mkdir -p bin
	@go build -o bin/date-calc date-calc/main.go
	@echo "Build complete: bin/date-calc"

clean:
	@rm -rf bin
	@echo "Cleaned build artifacts."
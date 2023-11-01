package main

import (
	"facade/parser"
	"facade/server"
	"log"
)

func main() {
	definitions, parserErr := parser.ParseDirectory("definitions")
	if parserErr != nil {
		log.Fatal(parserErr)
	}
	server.Serve(&definitions)
}

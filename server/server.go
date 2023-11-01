package server

import (
	"facade/handler"
	"facade/parser"
	"log"
	"net/http"
)

const (
	Port = "5000"
)

func Serve(definitions *[]parser.Definition) {
	for _, definition := range *definitions {
		for path, methods := range definition.Paths {
			localDefinition := definition
			localPath := path
			localMethods := methods
			http.HandleFunc(
				localDefinition.BasePath+localPath,
				handler.Handle(localDefinition, localPath, localMethods),
			)
		}
	}
	log.Println("[INFO] Server starting on port " + Port)
	serverErr := http.ListenAndServe(":"+Port, nil)
	if serverErr != nil {
		log.Fatal(serverErr)
	}
}

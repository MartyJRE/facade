package server

import (
	"facade/parser"
	"io"
	"log"
	"net/http"
)

func Serve(address string, definitions *[]parser.Definition) {
	for _, definition := range *definitions {
		for path, methods := range definition.Paths {
			http.HandleFunc(path, func(w http.ResponseWriter, r *http.Request) {

				log.Println("[INFO] Request received")
				found, findErr := methodCopy.FindMethod(r.Method)
				if findErr != nil {
					_, writeStringErr := io.WriteString(w, "Method not found")
					if writeStringErr != nil {
						log.Fatalln("[ERROR] Response not sent")
					}
					log.Println("[ERROR] Method not found")
					return
				}
				log.Println("[INFO] Response sent")
				_, writeStringErr := io.WriteString(w, found.Summary)
				if writeStringErr != nil {
					log.Fatalln("[ERROR] Response not sent")
				}
			})
		}
	}
	serverErr := http.ListenAndServe(address, nil)
	if serverErr != nil {
		log.Fatal(serverErr)
	}
}

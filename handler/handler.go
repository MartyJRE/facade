package handler

import (
	"facade/parser"
	"io"
	"log"
	"net/http"
	"strconv"
	"strings"
)

func Handle(definition parser.Definition, path string, methods parser.Methods) func(http.ResponseWriter, *http.Request) {
	return func(writer http.ResponseWriter, request *http.Request) {
		log.Println("[INFO] Request received " + strings.ToUpper(request.Method) + " " + definition.BasePath + path)
		found, findErr := methods.FindMethod(request.Method)
		if findErr != nil {
			writer.WriteHeader(http.StatusNotFound)
			log.Println("[ERROR] Method not found")
			log.Println(
				"[INFO] Response sent " +
					strings.ToUpper(request.Method) + " " +
					definition.BasePath +
					path + " (" +
					strconv.Itoa(http.StatusNotFound) +
					" " +
					http.StatusText(http.StatusNotFound) + ")",
			)
			return
		}
		log.Println(
			"[INFO] Response sent " +
				strings.ToUpper(request.Method) + " " +
				definition.BasePath + path + " (" +
				strconv.Itoa(http.StatusOK) +
				" " +
				http.StatusText(http.StatusOK) + ")",
		)
		_, writeStringErr := io.WriteString(writer, found.Summary)
		if writeStringErr != nil {
			log.Fatalln("[ERROR] Response not sent")
		}
	}
}

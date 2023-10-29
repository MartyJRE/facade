package parser

import (
	"errors"
	"gopkg.in/yaml.v3"
	"os"
	"strings"
)

type Info struct {
	Version     string `yaml:"version"`
	Title       string `yaml:"title"`
	Description string `yaml:"description"`
}

type Parameter struct {
	Name        string `yaml:"name"`
	In          string `yaml:"in"`
	Description string `yaml:"description"`
	Required    bool   `yaml:"required"`
}

type Response struct {
	Description string `yaml:"description"`
}

type Method struct {
	Description string           `yaml:"description"`
	Parameters  []Parameter      `yaml:"parameters"`
	Summary     string           `yaml:"summary"`
	Responses   map[int]Response `yaml:"responses"`
}

type Methods map[string]Method

type Paths map[string]Methods

type Definition struct {
	Info     Info     `yaml:"info"`
	BasePath string   `yaml:"basePath"`
	Swagger  string   `yaml:"swagger"`
	Consumes []string `yaml:"consumes"`
	Produces []string `yaml:"produces"`
	Schemes  []string `yaml:"schemes"`
	Paths    Paths    `yaml:"paths"`
}

func (methodMap *Methods) FindMethod(name string) (*Method, error) {
	for method, methodDescription := range *methodMap {
		println(strings.ToLower(method))
		println(strings.ToLower(name))
		println(strings.ToLower(method) == strings.ToLower(name))
		if strings.ToLower(method) == strings.ToLower(name) {
			return &methodDescription, nil
		}
	}
	return nil, errors.New("method not found")
}

func ParseDirectory(directoryName string) ([]Definition, error) {
	var definitions []Definition
	dir, dirErr := os.ReadDir(directoryName)
	if dirErr != nil {
		return nil, dirErr
	}
	for _, entry := range dir {
		if entry.IsDir() {
			dirResult, parserErr := ParseDirectory(directoryName + "/" + entry.Name())
			if parserErr != nil {
				return nil, parserErr
			}
			definitions = append(definitions, dirResult...)
		} else {
			definition := Definition{}
			data, fileErr := os.ReadFile(directoryName + "/" + entry.Name())
			if fileErr != nil {
				return nil, fileErr
			}
			yamlError := yaml.Unmarshal(data, &definition)
			if yamlError != nil {
				return nil, yamlError
			}
			definitions = append(definitions, definition)
		}
	}
	return definitions, nil
}

package main

import (
	"fmt"
	"log"
	"net/http"
	"strings"
)

func main() {
	fmt.Println("Starting test server...")

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Printf("\"%s\": %s\n", r.URL.Path, strings.Join(r.URL.Query()["msg"], ", "))

		fmt.Fprintf(w, "ok\n")
	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}

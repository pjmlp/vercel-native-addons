package handler
 
import (
  "fmt"
  "net/http"
)
 
func Handler(w http.ResponseWriter, r *http.Request) {
  w.Header().Set("Content-Type", "application/json")

  fmt.Fprintf(w, `{"message": "Hello Go world"}`)
}
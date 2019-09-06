package main

import (
	"admin/db"
	"github.com/tealeg/xlsx"
	"io"
	"log"
	"net/http"
)

func handle(reader io.Writer,filter string) error {
	file := xlsx.NewFile()
	users := db.GetUser(filter)
	sheet, errInSheet := file.AddSheet("Manthan")
	header := sheet.AddRow()
	headers := []string{"ID", "First Name", "Last Name", "Phone", "Email", "Year", "Branch", "Other", "Is club", "Club", "Github", "Spoj", "Code Chef", "Hacker Earth", "Project", "Designer", "Roll number", "Other", "Web",
		"Android",
		"Apty",
		"Misc",
		"Code"}

	header.WriteSlice(&headers, -1)
	if errInSheet != nil {
		panic(errInSheet)
	}
	for user := range users {
		r := sheet.AddRow()
		r.WriteStruct(&users[user], -1)
	}
	err := file.Write(reader)
	if err != nil {
		return  err
	}
	errInSave:=file.Save("./file.xlsx")
	if err!=nil{
		return  errInSave
	}
	return nil
}

func handlerForFile(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(200)
	w.Header().Add("content-type", "application/vnd.ms-excel")
	w.Header().Add("Content-Disposition", "attachment; filename=\"first_year.xlxs\"")
	handle(w,"first")
}

func saveMarks(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	email := r.Form.Get("email")
	apty := r.Form.Get("apty")
	web := r.Form.Get("web")
	android := r.Form.Get("android")
	misc := r.Form.Get("misc")
	code := r.Form.Get("code")
	c := make(map[string]string)
	c["email"] = email
	c["apty"] = apty
	c["web"] = web
	c["android"] = android
	c["misc"] = misc
	c["code"] = code
	_, error := db.SaveMarks(c)
	if error != nil {
		w.Write([]byte("error"))
	} else {
		w.Write([]byte("sumitted form"))
	}
}
func serv(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "./form.html")
}

func main() {
	http.HandleFunc("/marks", serv)
	http.HandleFunc("/xlsxfile.xlsx", handlerForFile)
	http.HandleFunc("/save", saveMarks)
	log.Fatal(http.ListenAndServe(":8081", nil))

}

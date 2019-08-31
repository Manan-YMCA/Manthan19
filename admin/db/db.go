package db

import (
	"context"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var collection* mongo.Collection

type User struct{
	Id primitive.ObjectID  `bson:"_id"`
	FirstName string `bson:"first_name,omitempty"`
	LastName string `bson:"last_name,omitempty"`
	Phone string `bson:"phone,omitempty"`
	Email string `bson:"email,omitempty"`
	Year string `bson:"year,omitempty"`
	Branch string `bson:"branch,omitempty"`
	IsClub string `bson:"is_club,omitempty"`
	Club string `bson:"club,omitempty"`
	Github string `bson:"github,omitempty"`
	Spoj string `bson:"spoj,omitempty"`
	CodeChef string `bson:"code_chef,omitempty"`
	HackerEarth string `bson:"hacker_earth,omitempty"`
	Message string `bson:"message,omitempty"`
	Designer string `bson:"designer,omitempty"`
	RollNumber string `bson:"roll_number,omitempty"`
	Other string `bson:"other,omitempty"`
	OsLink string `bson:"os_link,omitempty"`
	Web string `bson:"web,omitempty"`
	Android string `bson:"android,omitempty"`
	Apty string `bson:"apty,omitempty"`
	Misc string `bson:"misc,omitempty"`
	Code string `bson:"code,omitempty"`
}


func init() {
	client,errInClient:=mongo.NewClient(options.Client().ApplyURI("mongodb://localhost:27017"));
	if errInClient!=nil{
		panic(errInClient.Error())
	}
	errInConnect:=client.Connect(context.TODO())

	if errInConnect!=nil {
		panic(errInConnect.Error())
	}
	errorInPing:=client.Ping(context.TODO(),nil)
	if errorInPing!=nil {
		panic("error")
	}
	collection=client.Database("manthan").Collection("users")
}

func GetUser()[]User{
	cursor,errInFind:=collection.Find(context.Background(),bson.D{})
	if errInFind!=nil {
		panic(errInFind.Error())
	}
	var results[] User
	for cursor.Next(context.Background()){
		var result User
		e:=cursor.Current
		err:=bson.Unmarshal(e,&result)
		if err!=nil{
			panic(err)
		}
		results=append(results,result)
	}
	return results
}

func SaveMarks(marks map[string]string)(User,error){
	s:=collection.FindOneAndUpdate(context.Background(),bson.M{"email":marks["email"]},bson.M{"$set":marks})
	var result User
	if s.Err()!=nil {
		return User{},s.Err()
	} else {
		s.Decode(&result)
		return result,nil
	}

}
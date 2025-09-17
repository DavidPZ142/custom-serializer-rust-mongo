# Custom Serializer in Rust - Mongo DateTime

How serialize a Mongo bson DateTime with _Serde_
without affect the Serialize to Mongo Document
## Context
Imagine you have a Struct (Document) in Rust defined as follows

```
#[derive(Serialize)]
Struct User{
    #[serde(rename = "_id")]
    id : String,
    name : String,
    age : u16,
    birthday : bson::DateTime
}
```

And with Struct you need serialize to Sent how String to a httpClient or publish in kafka, 
but also you need convert the Struct in a bson::Document to made updates in MongoDB

If you Serialize with _Serde_Json::to_string()_ you obtain something like this:
```
{
  "_id": "1",
  "name": "David",
  "age": 24,
  "birthday": {
    "$date": {
      "$numberLong": "977881002"
    }
  }
}
```
and if we need to communicate with external services, it would not be useful to us as they would not recognise the date in this format.
But when conver into a  **Mongo Document** using _bson::to_document()_ is the expected behavior
```
Document(
    {
        "_id": String("1"), 
        "name": String("David"), 
        "age": Int32(24), 
        "birthday": DateTime(2000-12-27 0:00:00.0 +00:00:00)
    }
)
```

The mongo crate offers us a way to serialize the DateTimes into a string
```     
#[serde(with = "bson_datetime_as_rfc3339_string")]
``` 

But now review what happen if we Serialize with this helper
The serializing to json is correct

``` 
{
    "_id": "1",
    "name": "David",
    "age": 24,
    "birthday": "2000-12-27T00:00:00Z"
}
``` 
But now the behavior when convert it in to a Document is wrong because the Date will save how a String
``` 
Document(
    {
        "_id": String("1"), 
        "name": String("David"), 
        "age": Int32(24), 
        "birthday": String("2000-12-27T00:00:00Z")
    }
)
``` 

## Solution 
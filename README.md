# json-rocket

```rust
cargo run
```

localhost:8000/user

```js
//POST
{
    "name": "Lucas",
    "location": "Brazil",
    "title": "Software Engineer" 
}

//returns
"User created successfully"
```

localhost:8000/user/1

```js
//GET

//returns
{
    "_id": 1,
    "name": "Lucas Martins",
    "location": "Brazil",
    "title": "Software Engineer"
}
```


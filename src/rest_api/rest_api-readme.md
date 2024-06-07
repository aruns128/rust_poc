Testing the Endpoints
Here are some example curl commands to test the endpoints:

- Create an item:

# curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "name": "Item1"}' http://127.0.0.1:8080/create

- Get all items:

# curl http://127.0.0.1:8080/

- Update an item:

# curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "UpdatedItem1"}' http://127.0.0.1:8080/update

- Delete an item:

# curl -X DELETE http://127.0.0.1:8080/delete/1

Readme

1. cargo run
2. use these requests to test

```
curl "http://127.0.0.1:1337/?foo=hello&bar=mike&num=100"
curl "http://127.0.0.1:1337/index?foo=hello&bar=mike&num=100"

curl "http://127.0.0.1:1337/user/10"
curl "http://127.0.0.1:1337/user/10/home"
curl "http://127.0.0.1:1337/user/10/20"

curl -d "param1=value1&param2=value2" "http://127.0.0.1:1337/"
curl -d "foo=hello&bar=mike&num=100" "http://127.0.0.1:1337/test"
curl -d "foo=hello&bar=mike&num=100" "http://127.0.0.1:1337/test_post"

curl -l -H "Content-type: application/json" -X POST -d '{"foo": "hello", "bar": "mike", "num": 100}' "http://127.0.0.1:1337/post_json"

```


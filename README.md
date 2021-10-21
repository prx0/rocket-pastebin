---
marp: false
---

# Rocket pastebin
Based on https://rocket.rs/v0.5-rc/guide/pastebin/

- [] Add a web form to the index where users can manually input new pastes. Accept the form at POST /. Use format and/or rank to specify which of the two POST /routes should be called.
- [] Support deletion of pastes by adding a new DELETE /<id> route. Use PasteId to validate <id>.
- [] Indicate partial uploads with a 206 partial status code. If the user uploads a paste that meets or exceeds the allowed limit, return a 206 partial statuscode. Otherwise, return a 201 created status code.
- [] Set the Content-Type of the return value in upload and retrieve to text/plain.
Return a unique "key" after each upload and require that the key is present and matches when doing deletion. Use one of Rocket's core traits to do the keyvalidation.
- [] Add a PUT /<id> route that allows a user with the key for <id> to replace the existing paste, if any.
- [] Add a new route, GET /<id>/<lang> that syntax highlights the paste with ID <id> for language <lang>. If <lang> is not a known language, do no highlighting. Possibly validate <lang> with FromParam.
- [x] Use the local module to write unit tests for your pastebin.
- [] Dispatch a thread before launching Rocket in main that periodically cleans up idling old pastes in upload/.

## Run test 

``` sh
cargo run test && rm upload/*
```

Run test and remove files generated by tests.

## Endpoint

GET http://localhost:8000/pastebin


GET http://localhost:8000/pastebin/{id}

**query parameter:** id as alphenumeric base62 identifier. See PasteId structure.

**result**: type text/plain; charset=utf-8, represent a pastebin.

POST http://localhost:8000/pastebin/

**body:** type binary, represent your file.

**result**: type text/plain; charset=utf-8, represent a pastebin url
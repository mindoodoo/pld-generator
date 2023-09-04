# pld-generator

Simple utility generator utilizing **github's graphql API** to request cards from specified project and generate a **Project Log Document (PLD)** in **markdown format**

## Functionning

In order to generate the PLD, the pld-generator uses a template (default is template.md) and replaces special tags with specific values.

The following tags are supported :

| **Tag** | **Description**       |
| ------- | --------------------- |
| lucid   | Lucid chart diagrams  |
| cards   | Github projects cards |

In order to be parsed correctly, tags must be surrounded by two pairs of curly braces such as `{{cards}}`.

## Current state

*Still in development*

Implemented :

- [x] Github api key authentication
- [X] Graphql request to get cards
  - [ ] Handle paging
- [ ] Cards parsing
- [ ] Markdown generating

## Improvements

- [ ] Improve storage of gql request &rarr; .graphql file ?
- [ ] Add manual ordering feature
- [ ] Automatic numbering
- [ ] Use [anyhow](https://github.com/dtolnay/anyhow) for error handling
- [ ] Restructure serde datamodel module
    - ie: `model` module ?
- [ ] Github card linter &rarr; automatic formatter
- [ ] Improve deserialization error handling
  - For the time being it will the deserialization will fail in a lot of places if the response is of the error type, this should be better managed by checking the status code first

## Unhandled edge cases that should have user facing errors

- `app.rs` &rarr; if lucid document id is invalid
- Lucid module does not handle unauthenticated requests well enough

## Getting access token

In order to be able to connect to the lucid chart api, you need to get an [OAuth2](https://oauth.net/2/) **access token** and **refresh token**. The documentation for getting these tokens is [documented]() however was a bit of a hassle to understand so you may follow the steps here instead. They were especially unclear in a simple user script case such as this one.

1. Enable developer settings for your lucid chart account

2. Create an application

3. Create an OAuth2 client from within the settings of that application

5. Register the following redirect uri within that app :

`https://lucid.app/oauth2/clients/{client id}/redirect`

Replace `{client id}` with your clients id obviously. Stuff like this won't be mentioned later on.

6. Fill out and open the following authorization url in your browser and grant access

```
https://lucid.app/oauth2/authorize?client_id={client id}&redirect_uri={previously set redirect uri}&scope=lucidchart.document.content:readonly+lucidchart.document.app.folder+lucidchart.document.app.picker:readonly+offline_access
```

This url grants the following scope permissions to the access token you will receive :

- lucidchart.document.content:readonly
- lucidchart.document.app.folder
- lucidchart.document.app.picker:readonly
- offline_access &rarr; **Required in order to get refresh token.**

7. Copy the code it shows. We'll refer to it as `code` from now on.

8. Request **access token** and **refresh token**

```curl
curl 'https://api.lucid.co/oauth2/token' \
  --request 'POST' \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "code": "",
    "client_id": "",
    "client_secret": "",
    "grant_type": "authorization_code",
    "redirect_uri": ""
  }'
```

9. You should now have an access token and a refresh token !
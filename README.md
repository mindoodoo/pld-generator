# pld-generator

Simple utility generator utilizing **github's graphql API** to request cards from specified project and generate a **Project Log Document (PLD)** in **markdown format**

## Current state

*Still in development*

Implemented :
    - [x] Github api key authentication
    - [ ] Graphql request to get cards
    - [ ] Cards parsing
    - [ ] Markdown generating

## Improvements

- [ ] Improve storage of gql request &rarr; .graphql file ?
- [ ] Add manual ordering feature
- [ ] Automatic numbering
- [ ] Use [anyhow](https://github.com/dtolnay/anyhow) for error handling
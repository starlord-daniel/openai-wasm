# WASM application to run OpenAI chat

This application is a simple WASM application that runs a chatbot from OpenAI.

## Features

- [x] Web UI to type questions and get answers from OpenAI chatbot
- [x] WASM backend API to call OpenAI chatbot and handle responses
- [x] Keeping the context of the current conversation
- [ ] Connecting to Azure Search to search through user data
- [ ] Embedding the users question and send to Search
- [ ] Handle user data and send for formatting to OpenAI
- [ ] User authentication
- [ ] User history

## Installation and running

```bash
spin build
spin run
```

## Testing commands

Prepare the environment with the Makefile:

```bash
make prepare
```

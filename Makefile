# Populate the variables in the http/test.http file with the values of the .env file

# Load the .env file
include .env

.PHONY: prepare
prepare: 
	@echo "--- Preparing the environment ---"

	@$(MAKE) prepare-http-test

	@echo "--- Finished preparing the environment ---"

# Replace the variables in the http/test.http.default file with the values of the .env file and create a new file http/test.http
.PHONY: prepare-http-test
prepare-http-test:
	@echo "- Preparing the http test file -"

	@cp http/test.http.default http/test.http

	@for var in $$(grep -oE '\{\{[A-Z_]+\}\}' http/test.http | sort | uniq); do \
        replace=$$(echo $$var | sed 's/{{//g' | sed 's/}}//g'); \
        value=$$(grep $$replace .env | cut -d '=' -f 2); \
        sed -i "" "s#$$var#$$value#g" http/test.http;  \
    done

	@echo "- Finished creating the test.http file -"

.PHONY: run
run:
	@echo "--- Running the application ---"

	spin build
	export SPIN_VARIABLE_OPENAI_API_KEY=$(AZURE_OPENAI_API_KEY) && \
	export SPIN_VARIABLE_OPENAI_ENDPOINT=$(AZURE_OPENAI_ENDPOINT) && \
	export SPIN_VARIABLE_OPENAI_DEPLOYMENT_NAME=$(AZURE_OPENAI_DEPLOYMENT_NAME) && \
	export SPIN_VARIABLE_SEARCH_ENDPOINT=$(AZURE_SEARCH_ENDPOINT) && \
	export SPIN_VARIABLE_SEARCH_API_KEY=$(AZURE_SEARCH_API_KEY) && \
	export SPIN_VARIABLE_SEARCH_INDEX_NAME=$(AZURE_SEARCH_INDEX_NAME) && \
	spin up

	@echo "--- Finished running the application ---"

SHELL := /bin/bash

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_PATH := $(patsubst %/,%,$(dir $(MKFILE_PATH)))
DOCKER ?= $(shell which docker 2> /dev/null || echo "docker")

.PHONY: integration_tests
integration_tests:
	$(DOCKER) compose --project-directory $(PROJECT_PATH) -f e2e/docker-compose.yaml up --build  --exit-code-from tester

.PHONY: sandbox
sandbox:
	$(DOCKER) compose --project-directory $(PROJECT_PATH) -f sandbox/docker-compose.yaml run --build start_services

.PHONY: clean
clean:
	$(DOCKER) compose --project-directory $(PROJECT_PATH) -f e2e/docker-compose.yaml down --volumes --remove-orphans
	$(DOCKER) compose --project-directory $(PROJECT_PATH) -f sandbox/docker-compose.yaml down --volumes --remove-orphans

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

GRPCURL=$(PROJECT_PATH)/bin/grpcurl
$(GRPCURL):
	$(call go-install-tool,$(GRPCURL),github.com/fullstorydev/grpcurl/cmd/grpcurl@v1.8.9)

.PHONY: grpcurl
grpcurl: $(GRPCURL) ## Download grpcurl locally if necessary.

# go-install-tool will 'go install' any package $2 and install it to $1.
define go-install-tool
@[ -f $(1) ] || { \
set -e ;\
TMP_DIR=$$(mktemp -d) ;\
cd $$TMP_DIR ;\
go mod init tmp ;\
echo "Downloading $(2)" ;\
GOBIN=$(PROJECT_PATH)/bin go install $(2) ;\
rm -rf $$TMP_DIR ;\
}
endef

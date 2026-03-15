#!/bin/bash
set -exo pipefail
gen-changelog generate \
    --display-summaries \
    --name "CHANGELOG.md" \
    --package "lambda_sqs" \
    --repository-dir "../.." \
    --next-version "${NEW_VERSION:-${1}}"

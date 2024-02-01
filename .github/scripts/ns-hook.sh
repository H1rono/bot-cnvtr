#!/usr/bin/env bash

set -eu
set -o pipefail

function gh_api() {
	gh api \
		-H "Accept: application/vnd.github+json" \
		-H "X-GitHub-Api-Version: 2022-11-28" \
		"$@"
}

function hook_id() {
	gh_api "/repos/${REPOSITORY}/hooks" \
		| jq -r -e 'map(select(.type == "Repository")) | .[0].id'
}

function push_delivery_id() {
	gh_api "/repos/${REPOSITORY}/hooks/${HOOK_ID}/deliveries" \
		| jq -r -e 'map(select(.event == "push")) | .[0].id'
}

function delivery_request() {
	gh_api "/repos/${REPOSITORY}/hooks/${HOOK_ID}/deliveries/${DELIVERY_ID}" \
		| jq -r -e '.request | @text'
}

function request() {
	json=`cat`
	headers=`echo "$json" | jq '.headers'`
	payload=`echo "$json" | jq -r -e '.payload | @text'`

	content_type=`echo "$headers" | jq -r -e '."Content-Type"'`
	github_delivery=`echo "$headers" | jq -r -e '."X-GitHub-Delivery"'`
	github_event=`echo "$headers" | jq -r -e '."X-GitHub-Event"'`
	github_hook_id=`echo "$headers" | jq -r -e '."X-GitHub-Hook-ID"'`

	curl -f -X POST \
		-H "Content-Type: $content_type" \
		-H "X-GitHub-Delivery: $github_delivery" \
		-H "X-GitHub-Event: $github_event" \
		-H "X-GitHub-Hook-ID: $github_hook_id" \
		-d "$payload" \
		"${1}"
}

case "${1}" in
	"hook_id" | "hook-id" )
		hook_id ;;
	"push_delivery_id" | "push-delivery-id" )
		push_delivery_id ;;
	"delivery_request" | "delivery-request" )
		delivery_request ;;
	"request" )
		request "${2}" ;;
	* ) exit 1 ;;
esac

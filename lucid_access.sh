#!/bin/bash

# This script enables the user to do the following :
#
# - Retrieve auth code for use in creating token
# - Request acces_token and refresh_token
#
# Use the following command in order to export .env to env :
#
# export (cat -p .env | xargs -L 1)
# 
# Expect variables are :
#
# - LUCID_CLIENT_ID
# - LUCID_CLIENT_SECRET

# Utility ANSI escape code vars
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

redirect_uri="https://lucid.app/oauth2/clients/${LUCID_CLIENT_ID}/redirect"
auth_uri="https://lucid.app/oauth2/authorize?client_id=${LUCID_CLIENT_ID}&redirect_uri=${redirect_uri}&scope=lucidchart.document.content:readonly+lucidchart.document.app.folder+lucidchart.document.app.picker:readonly+offline_access"
token_request_uri="https://api.lucid.co/oauth2/token"

c_flag=false

function usage() {
  echo -e "This script will facilitate the OAuth2 process for retrieving a lucid chart ${RED}access_token${NC} and ${RED}refresh_token${NC}"
  echo -e ""
  echo -e "Environement variables requried :"
  echo -e "${YELLOW}LUCID_CLIENT_ID"
  echo -e "${YELLOW}LUDIC_CLIENT_SECRET"
  echo -e ""
  echo -e "Use -c option to only prompt app authorization and code"
  echo -e "Use -a to only prompt for code and request access token. This is actually relatively useless lol"
  exit 0
}

function check_variables() {
  if [[ -z "$LUCID_CLIENT_ID" || -z "$LUCID_CLIENT_SECRET" ]]; then
    echo -e "Please make sure ${YELLOW}LUCID_CLIENT_ID${NC} and ${YELLOW}LUDIC_CLIENT_SECRET${NC} are set..."
    exit 1
  fi
}

function code_prompt() {
  check_variables

  echo -e "Please set the following redirection url for your oauth application in the developper settings :"
  echo -e "${YELLOW}$redirect_uri${NC}\n"

  if [[ "$c_flag" = false ]]; then
    echo -e "Make sure to copy the auth code set as it will be required for the next step..."
  fi;
  read -p "Press enter to open authorization url in firefox..."

  firefox "$auth_uri"

  echo ""
}

function request_tokens() {
  check_variables

  read -p "Please enter the code retrieved in previous step : "

  curl_body="{
    \"code\": \"$REPLY\",
    \"client_id\": \"$LUCID_CLIENT_ID\",
    \"client_secret\": \"$LUCID_CLIENT_SECRET\",
    \"grant_type\": \"authorization_code\",
    \"redirect_uri\": \"$redirect_uri\"
  }"

  curl_args=(
    --request POST
    --header 'Content-Type: application/json'
    --data-raw "$curl_body"
  )

  curl "$token_request_uri" "${curl_args[@]}" | json_pp
}

# Main function is only executed when no flags
function main() {
  code_prompt

  request_tokens
}

while getopts "hca" flag; do
  case $flag in
    h)
      usage
      exit 0
    ;;
    c)
      c_flag=true
      echo -e "${YELLOW}Note:${NC} -c flag set, only prompting for authorization and code\n"
      code_prompt
      exit 0
    ;;
    a)
      request_tokens
      exit 0
    ;;
  esac
done

# No flags

main

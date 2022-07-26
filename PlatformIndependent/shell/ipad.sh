#!/bin/bash

CREDS="textastic:password"

if [ $# -ne 1 ]; then
  echo "Flag required. -f to fetch file, -p to push file";
  exit;
fi

read -p "Enter IP: " IP;
read -e -p "Enter File Name: " FILE;

if [ "$1" == "-f" ]; then
  curl -u "$CREDS" http://$IP/Swap/$FILE --Output $FILE --anyauth;
fi

if [ "$1" == "-p" ]; then
  curl -u "$CREDS" -T $FILE http://$IP/Swap/$FILE --anyauth;
fi




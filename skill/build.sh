#!/usr/bin/env bash

# Builds this skill into a .zip file to be uploaded to AWS Lambda.

rm skill.zip
zip -r skill.zip index.js node_modules

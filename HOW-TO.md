# How to Guide

## Example:
![screenshot](https://github.com/aenoshrajora/VulneraX/blob/master/screenshot.png)

# Start up the tool

`./VulneraX`

You can add this to `/usr/bin` if you want to access it anywhere

## Target URL

Full URL without the parameters

Example: https://portswigger-labs.net/xss/xss.php

## Query Parameters

Enter the query parameters

Put $ where you want the payload to go

Example: x=$&y=$&z=$

## Path to Wordlist

Full path to wordlist

Example: /home/<username>/wordlists/super_awesome_payloads.txt

# Verbose Output

Verbose on will show every request

Verbose off will only show payloads that are potentially vulnerable

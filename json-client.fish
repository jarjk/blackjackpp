#!/usr/bin/env fish
if test "$argv[1]" = ""
    set addr "http://localhost:5225"
else
    set addr "$argv[1]"
end
set uname bob

function GET
    set resp (curl --silent "$addr/$argv[1]")
    if not echo $resp | jq
        echo $resp
    end
    echo
end

function POST
    set resp (curl --silent -XPOST "$addr/$argv[1]")
    if not echo $resp | jq
        echo $resp
    end
    echo
end

GET "join?username=$uname"

POST "bet/$uname?amount=100"

read -P 'action: ' action
POST "move/$uname?action=$action"
GET game_state

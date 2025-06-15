#!/usr/bin/env fish
set addr "localhost:18080"
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

POST "bet/$uname?amount=400"

# GET game_state

read -P 'action: ' action
POST "move/$uname?action=$action"
GET game_state

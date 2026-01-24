build:
  comtrya -d setup -v apply
  pipelight trigger --flag pre-commit --attach
  pipelight logs -vv
  just test
  just clean

test:
  pipelight run tests --attach
  pipelight logs -vv

lint:
  task lint

format:
  task format

audit:
  task audit

coverage:
  task audit:code-coverage

clean:
  task clean

@doctor:
  echo "\n\n=== Just Doctor ===\n\n"
  just -l
  echo "\n\n=== Taskfile Doctor ===\n\n"
  task -l
  echo "\n\n=== Pipelight Doctor ===\n\n"
  pipelight ls
  echo "\n\n=== Lefthook Doctor ===\n\n"
  lefthook validate
  echo "\n\n=== Prek Doctor ===\n\n"
  prek list
  echo "\n\n=== Comtrya Doctor ===\n\n"
  comtrya -d manifests status
  echo "\n\n=== Goji Doctor ===\n\n"
  goji check
  COUNT=$(cat .goji.json | jq '.types | length') ; echo "\n\nFound $COUNT goji types."

prepare-commit-msg file:
  #!/bin/sh
  if [ ! -f ".goji.json" ]; then
    goji init --repo
  fi
  RAWMSG=$(cat {{file}} | grep -v '^[ ]*#')
  echo "prepare raw :: $RAWMSG"
  goji --no-commit --message "$RAWMSG" > {{file}}

lint-commit-msg file:
  #!/bin/sh
  RAWMSG=$(cat {{file}} | grep -v '^[ ]*#')
  echo "lint raw :: $RAWMSG"
  MSG=$(goji check --from-file {{file}})
  CHECK=$(echo $MSG | grep '^Error' | wc -l)
  if [ "$CHECK" -gt 0 ]; then
    return 1
  fi

install:
  lefthook install
  prek install
  prek auto-upgrade
  mise trust --quiet .mise.toml
  @[ -f ".mise.local.toml" ] && mise trust --quiet .mise.local.toml || return 0
  mise install

help:
  task help

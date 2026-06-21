default: test

test-lse:
    devenv shell test-lse

test:
    devenv shell test

test-host:
    devenv shell test-host

coverage:
    devenv shell coverage

coverage-html:
    devenv shell coverage-html

ci:
    devenv shell ci

test-auto:
    devenv shell test-auto

setup:
    @echo "Environment setup is automatically managed by devenv (nix/direnv)."

generate-devcontainer:
    devenv shell generate-devcontainer






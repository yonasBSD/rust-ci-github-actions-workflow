# Import the distribution tasks (required)
import 'scripts/dist.just'

# Import local tasks
# Note: Since `just` doesn't have "optional" imports,
# this file MUST exist or `just` will fail.
import 'scripts/local.just'

# You can still define root-level tasks here
default:
    @just --list

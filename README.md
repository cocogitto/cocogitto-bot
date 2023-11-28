## Cocogitto-bot

[Cocogitto](https://github.com/cocogitto/cocogitto) is a set of cli tools for the [conventional commits specification](https://www.conventionalcommits.org/en/v1.0.0/).

This bot uses cocogitto to perform [status checks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/about-status-checks) on your pull-requests and ensure every commit match the specification.

On failure, your pull-request will be decorated with a comment explaining what went wrong regarding the specification :

![example screenshot](https://docs.cocogitto.io/cog-bot-example.png)

### Installation

To install it just go to [github.com/apps/cocogitto-bot](https://github.com/apps/cocogitto-bot)
and click "Configure". Add the desired repository and grant the required permission.
Once it is done cocogitto-bot will comment on every pull-request events.

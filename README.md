# GitGuardian

Creates and maintains a Top Star Github Profile based on the activity distribution configuration.

<p align="center">
    <img src="https://github.com/spamserv/gitguardian/blob/main/logo.png?raw=true" alt="drawing" width="300"/>
</p>

## What does it do?

Populate config file corresponding with the activity graph on Github:
    - Commits
    - Code Reviews
    - Pull Requests
    - Issues

Your activity distribution configuration can look something like this:
    - commits: 0.65
    - code_reviews: 0.12
    - pull_requests: 0.08
    - issues: 0.15

Which should sum to 1 (100%). Set it as `cron` job or run it daily/occassionally.
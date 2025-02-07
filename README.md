# GitGuardian

Creates and maintains a Top Star Github Profile based on the activity distribution configuration.

<p align="center">
    <img src="https://github.com/spamserv/gitguardian/blob/main/logo.png?raw=true" alt="drawing" width="300"/>
</p>

## How to?

Populate config file corresponding with the activity graph on Github:

    - Commits
    - Code Reviews
    - Pull Requests
    - Issues
    - Minimum and maximum number of activities (randomized)

Your activity distribution configuration can look something like this:

    - commits: f64 (e.g. - 0.65)
    - code_reviews: f64 (e.g. - 0.12)
    - pull_requests: f64 (e.g. - 0.08)
    - issues: f64 (e.g. - 0.5)
    - min_activities: u8 (e.g. - 5)
    - max_activities: u8 (e.g. - 20)


## Explanation

Activities should sum to 1 (100%). 

Set a `minimum` and `maximum` value for the number of activities per day (total number of commits, code reviews, pull requests and issues).

The configuration will generate dummy activities based on the activity diagram.

You can set it as `cron` job or run it daily/occassionally or create a GitHub Action (TBD).


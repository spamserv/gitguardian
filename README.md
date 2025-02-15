# GitGuardian

Creates and maintains a Top Star Github Profile based on the activity distribution configuration.

<p align="center">
    <img src="https://github.com/spamserv/gitguardian/blob/main/logo.png?raw=true" alt="drawing" width="300"/>
</p>

## How to?

Populate config file corresponding with the activity graph on Github (`config.toml`):

    - Commits
    - Code Reviews
    - Pull Requests
    - Issues
    - Minimum and maximum number of activities (randomized)
    - Repository to use for creating fake activity
    - Reposutory owner

Example config:
    
```
[config]
commits = 0.55
pull_requests = 0.08
code_reviews = 0.20
issues = 0.17
low = 4
high = 18
repository_name = "gitspam"
repository_owner = "spamserv"
```

Your activity distribution configuration can look something like this:

    - commits: f64 (e.g. - 0.65)
    - code_reviews: f64 (e.g. - 0.12)
    - pull_requests: f64 (e.g. - 0.08)
    - issues: f64 (e.g. - 0.5)
    - min_activities: u8 (e.g. - 5)
    - max_activities: u8 (e.g. - 20)
    - repository_name: &str
    - repository_owner: &str


## Explanation

1. Activities should sum to 1 (100%). 

2. Set a `minimum` and `maximum` value for the number of activities per day (total number of commits, code reviews, pull requests and issues).

3. Set repository owner and his name.

4. Populate `.env` (copy from `.env.example`) and make sure your `GITHUB_ACCESS_TOKEN` has sufficient permissions to make the necessary changes on the repo level.

The configuration will generate dummy activities based on the activity diagram.


## To Do List

[ ] Set it as `cron` job or run it daily/occassionally or create a GitHub Action (TBD).
[ ] Create tests

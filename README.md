# internal-site-monitor
This is a simple utility that allows me to monitor uptime status of my internal sites without exposing them to the public. [I've pointed my Upptime instance](https://github.com/Sheemap/snazcat-upptime) at it to monitor the sites and report if things go down.

## How it works
On startup the config file is loaded to provide context on how to reach various sites, and how to tell if they are up or not.

Once the application is running, you can visit `{address}:8080/{path}` to check any site, where `path` is the site name. The application will make a GET request to the configured URL, and check its returned status code.
If it is as expected, it will return a `200` status code, indicating the site is up. If its not, it will return a `500` status code. Indicating the site has a problem.

## Configuration
The config file must be located at `config/config.json` relative to the binary. The config should be in json format, and be a list of sites.

Each config in the list represents a target for monitoring. The fields are as follows:

- `name` The site name, and path for checking status
- `status_code` The expected status code when making a GET request
- `url` Where to make the GET request to

```
[
  {
    "name": "SiteName",
    "status_code": 200,
    "url": "http://192.168.1.169:4343"
  }
]
```

## Notes and musings
I probably wont touch or change this in the future, as it accomplishes what I need for now. But if I were, it would be nice to find a more minimal framework than Actix. I think the Actix choice was overkill, and I did it mainly because I wanted something well documented and supported.
Unfortunately, I believe this is one of the main reasons why the docker build takes so long (roughly 10 minutes!!!). Which really just shouldnt happen for an application this simple. If I were to improve this application, the build time is high on my list for improvement.

This was my first Rust web project, and despite its simplicity, I ran into problems with toml and json parsing, as well as figuring out how to pass the config around without its lifetime expiring. Definitely learned more from this project than I was expecting.

Thanks for reading!

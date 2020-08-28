# retstat
[crates.io][crates]

[crates]: https://crates.io/crates/retstat

This program will **ret**urn the **stat**us code to you. That is, in its HTTP
response, the status code you pass in the URL will be used. The body of the
response is the default phrasing for that code. If you pass a code that is
unknown, "Unknown" will be the body.

retstat was born out of the desire to have a reliable source of status codes so
I could test my shell scripts without hitting someone else's server so many
times.

I have it live at <https://genbyte.dev/status/>, so some examples:

- <https://genbyte.dev/status/302> returns Status 302
- <https://genbyte.dev/status/500> returns Status 500

There is a very basic service file for systemd in the root of the source tree.
It assumes the bin is at `/usr/bin/retstat`.

### Configuration
There isn't much configuration available, as this small server doesn't do much.
You can change the listening address with the `-l` cli option. The long option
is `--host`, if you want to use that.

retstat defaults to listening on `localhost:30210`.

Below is the recommended configuration for Nginx.
Thanks to [this][so] answer which taught me how to remove the path the proxy
lives under from the path the proxy receives. What I mean is, if you visit
<https://genbyte.dev/status/200>, retstat won't see `/status/200` but `/200`
instead.

[so]: https://serverfault.com/a/562850/524425

```nginx
location = /status {
	return 302 /status/;
}

location /status/ {
	proxy_pass http://localhost:30210/;
}
```

# retstat
[Homepage: hub.sr.ht][hub] |  [Tickets: todo.sr.ht][todo] | [crates.io][crates]

[hub]: https://sr.ht/~genbyte/retstat/
[todo]: https://todo.sr.ht/~genbyte/retstat
[crates]: https://crates.io/crates/retstat

This program will **ret**urn the **stat**us code you provide it in the URL as
the body of the response while also using that as the status code.

retsta was born out of the desire to have a reliable source of status codes so I
could test my shell scripts without hitting someone else's server so many times.

I have it live at <https://genbyte.dev/status/>, so some examples:

- <https://genbyte.dev/status/302> returns Status 302
- <https://genbyte.dev/status/500> returns Status 500

There is a very basic service file for systemd in the root of the source tree.
It assumes the bin is at `/usr/bin/retstat` and the Rocket.toml file lives in the
`/etc/retstat` directory.

I'd like to move away from Rocket eventually, as it's needlessly (for this
purpose) heavy. Ideally I'll use something like tiny_http, but I wanted the
threading that comes with Rocket by default.

### Configuration
Configuration is controlled through the [`Rocket.toml`](Rocket.toml) file. There
you can change the port and address that the server runs on.

retstat defaults to only running on `localhost`, port `30210`.

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

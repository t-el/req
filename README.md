# req
### a simple http client implemented in rust 
> support get & post methods for now

### examples
#### get 
`cargo run get https://api.github.com/users/defunkt`
```
----------------------------------------------
"server": "GitHub.com"
"date": "Tue, 07 Jun 2022 09:12:01 GMT"
"content-type": "application/json; charset=utf-8"
"cache-control": "public, max-age=60, s-maxage=60"
"vary": "Accept, Accept-Encoding, Accept, X-Requested-With"
"etag": "W/\"9957c1e1e97ebddb007410bebd8e9034536655811efb0c59c5b03bda32a6e32f\""
"last-modified": "Wed, 01 Jun 2022 23:46:26 GMT"
"x-github-media-type": "github.v3; format=json"
"access-control-expose-headers": "ETag, Link, Location, Retry-After, X-GitHub-OTP, X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Used, X-RateLimit-Resource, X-RateLimit-Reset, X-OAuth-Scopes, X-Accepted-OAuth-Scopes, X-Poll-Interval, X-GitHub-Media-Type, X-GitHub-SSO, X-GitHub-Request-Id, Deprecation, Sunset"
"access-control-allow-origin": "*"
"strict-transport-security": "max-age=31536000; includeSubdomains; preload"
"x-frame-options": "deny"
"x-content-type-options": "nosniff"
"x-xss-protection": "0"
"referrer-policy": "origin-when-cross-origin, strict-origin-when-cross-origin"
"content-security-policy": "default-src 'none'"
"x-ratelimit-limit": "60"
"x-ratelimit-remaining": "55"
"x-ratelimit-reset": "1654593807"
"x-ratelimit-resource": "core"
"x-ratelimit-used": "5"
"accept-ranges": "bytes"
"content-length": "1305"
"x-github-request-id": "2A0B:8425:2957B87:2A5FD4F:629F1661"
----------------------------------------------
{
  "login": "defunkt",
  "id": 2,
  "node_id": "MDQ6VXNlcjI=",
  "avatar_url": "https://avatars.githubusercontent.com/u/2?v=4",
  "gravatar_id": "",
  "url": "https://api.github.com/users/defunkt",
  "html_url": "https://github.com/defunkt",
  "followers_url": "https://api.github.com/users/defunkt/followers",
  "following_url": "https://api.github.com/users/defunkt/following{/other_user}",
  "gists_url": "https://api.github.com/users/defunkt/gists{/gist_id}",
  "starred_url": "https://api.github.com/users/defunkt/starred{/owner}{/repo}",
  "subscriptions_url": "https://api.github.com/users/defunkt/subscriptions",
  "organizations_url": "https://api.github.com/users/defunkt/orgs",
  "repos_url": "https://api.github.com/users/defunkt/repos",
  "events_url": "https://api.github.com/users/defunkt/events{/privacy}",
  "received_events_url": "https://api.github.com/users/defunkt/received_events",
  "type": "User",
  "site_admin": false,
  "name": "Chris Wanstrath",
  "company": null,
  "blog": "http://chriswanstrath.com/",
  "location": null,
  "email": null,
  "hireable": null,
  "bio": "🍔",
  "twitter_username": null,
  "public_repos": 107,
  "public_gists": 273,
  "followers": 21432,
  "following": 210,
  "created_at": "2007-10-20T05:24:19Z",
  "updated_at": "2022-06-01T23:46:26Z"
}
```

#### post
`cargo run post https://example.com/post`




# req
### a simple http client implemented in rust 
> support get & post methods for now

### examples
#### get 
`cargo run get https://api.github.com/users/defunkt`
```
-----------------------------------------------
"server": "GitHub.com"
"date": "Tue, 07 Jun 2022 09:31:38 GMT"
"content-type": "application/json; charset=utf-8"
"cache-control": "public, max-age=60, s-maxage=60"
"vary": "Accept, Accept-Encoding, Accept, X-Requested-With"
"etag": "W/\"98639d7c070364b4f15df25be62b123a122f5b85005cf6b75f349b65f1154b06\""
"last-modified": "Sat, 21 May 2022 17:09:37 GMT"
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
"x-ratelimit-remaining": "59"
"x-ratelimit-reset": "1654597898"
"x-ratelimit-resource": "core"
"x-ratelimit-used": "1"
"accept-ranges": "bytes"
"content-length": "1354"
"x-github-request-id": "11DB:6287:569118:5A48F1:629F1AFA"
----------------------------------------------
{
  "login": "t-el",
  "id": 57259373,
  "node_id": "MDQ6VXNlcjU3MjU5Mzcz",
  "avatar_url": "https://avatars.githubusercontent.com/u/57259373?v=4",
  "gravatar_id": "",
  "url": "https://api.github.com/users/t-el",
  "html_url": "https://github.com/t-el",
  "followers_url": "https://api.github.com/users/t-el/followers",
  "following_url": "https://api.github.com/users/t-el/following{/other_user}",
  "gists_url": "https://api.github.com/users/t-el/gists{/gist_id}",
  "starred_url": "https://api.github.com/users/t-el/starred{/owner}{/repo}",
  "subscriptions_url": "https://api.github.com/users/t-el/subscriptions",
  "organizations_url": "https://api.github.com/users/t-el/orgs",
  "repos_url": "https://api.github.com/users/t-el/repos",
  "events_url": "https://api.github.com/users/t-el/events{/privacy}",
  "received_events_url": "https://api.github.com/users/t-el/received_events",
  "type": "User",
  "site_admin": false,
  "name": "Taha Elkarroumy",
  "company": null,
  "blog": "https://venerable-faloodeh-01a096.netlify.app/",
  "location": null,
  "email": null,
  "hireable": null,
  "bio": "Backend developer\r\nLove to code with Nodejs and Python",
  "twitter_username": null,
  "public_repos": 29,
  "public_gists": 2,
  "followers": 1,
  "following": 7,
  "created_at": "2019-11-01T13:05:28Z",
  "updated_at": "2022-05-21T17:09:37Z"
}

```

#### post
`cargo run post https://example.com/post`




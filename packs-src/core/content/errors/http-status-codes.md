---
id = "err/http-status-codes"
kind = "error"
title = "HTTP Status Codes"
category = "reference/errors/http"
tags = ["http", "errors", "api"]
risk = "info"
---
What each class and the common codes actually mean when they show up in logs.

## Classes
<!-- section-tags: http, errors, classes -->

- **1xx** informational · **2xx** success · **3xx** redirect
- **4xx** client did something wrong · **5xx** server fell over

## The ones you meet daily
<!-- section-tags: http, errors, codes -->

| Code | Meaning | Typical cause / action |
|---|---|---|
| 200 | OK | success |
| 204 | No Content | successful DELETE/PUT, empty body |
| 301 / 308 | permanent redirect | update the link/bookmark |
| 302 / 307 | temporary redirect | often a login bounce |
| 304 | Not Modified | cache hit — normal, not an error |
| 400 | Bad Request | malformed input; read the body |
| 401 | Unauthorized | no/bad credentials — auth first |
| 403 | Forbidden | authenticated but not allowed |
| 404 | Not Found | wrong path, or resource deleted |
| 405 | Method Not Allowed | POST to a GET-only route |
| 408 | Request Timeout | slow client / WAN |
| 409 | Conflict | version clash, duplicate |
| 413 | Payload Too Large | upload limits, reverse proxy body cap |
| 429 | Too Many Requests | rate limited — honor Retry-After |
| 500 | Internal Server Error | app bug — server logs, not client |
| 502 | Bad Gateway | upstream dead behind proxy/LB |
| 503 | Service Unavailable | overloaded / maintenance |
| 504 | Gateway Timeout | upstream too slow behind proxy/LB |

Rule: **4xx = look at the request, 5xx = look at the server, 3xx = look at
where it's sending you.**

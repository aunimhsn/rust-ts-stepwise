# GitHub Pages usage

Stepwise is published as a static website on GitHub Pages. Its limits and rules, read on 2026-10-31 in the [GitHub Pages limits](https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits):

| Limit | Value | Stepwise |
|---|---|---|
| Published site | 1 GB at most | A few megabytes: the interface, the WebAssembly module, the SVG elements, the levels and the manual. |
| Source repository | 1 GB recommended | Far below. |
| Bandwidth | Soft limit of 100 GB per month | A visit loads about 2 MB the first time, then the browser cache: about 50,000 first visits a month. A whole school would stay far below. |
| Builds | Soft limit of 10 per hour | Doesn't apply to a website published by a custom GitHub Actions workflow, which Stepwise uses. |
| Deployment | Fails after 10 minutes | The build takes a minute or two. |

Rules:

- GitHub Pages is not a free host for an online business or a commercial service: Stepwise is free and educational, without payment.
- No sensitive data (passwords, payment): Stepwise has no account and sends nothing; the progress stays in the learner's browser.
- Over its limits, a site may stop being served or GitHub may suggest a CDN.

Risk for the owner to accept (written on 2026-10-31):

- A class or a school visiting at once could, in theory, reach the soft bandwidth limit: unlikely at the sizes above. If it happens, the same static files can move to another host (Cloudflare Pages, Netlify) without changing the code.

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

## Address

Stepwise is served at **`https://stepwise.sylent.art/`** (decided by the owner on 2026-10-31), a subdomain of the owner's website, without buying a domain:

- `sylent.art` is itself a project site (the repository `aunimhsn/sylent.art`, with its own `CNAME`), not the user site `aunimhsn.github.io`: GitHub allows one user site per account but one project site per repository, so the two websites don't conflict. Each site has its own bandwidth limit.
- The owner adds a DNS `CNAME` record at the registrar of `sylent.art`: `stepwise` → `aunimhsn.github.io`; the custom domain is set in **Settings › Pages** of the repository, with **Enforce HTTPS**.
- The owner verifies `sylent.art` in the GitHub account (**Settings › Pages › Add a domain**, a DNS `TXT` record), so that no one else can claim a subdomain pointing to GitHub without a repository behind it. No domain was verified on 2026-10-31.
- The website never hard-codes its path: it works at the root of a domain as well as under `/rust-ts-stepwise/` (the default address `https://aunimhsn.github.io/rust-ts-stepwise/`), with the `paths.base` setting of SvelteKit. The domain can change later without changing the code.

## Risks accepted by the owner

On 2026-10-31:

- A class or a school visiting at once could, in theory, reach the soft bandwidth limit: unlikely at the sizes above. If it happens, the same static files can move to another host (Cloudflare Pages, Netlify) without changing the code.

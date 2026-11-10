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

Stepwise is served at **`https://stepwise.sylent.tools/`** (decided by the owner on 2026-11-10, replacing `stepwise.sylent.art` chosen on 2026-10-31), a subdomain of a domain the owner already has, without buying one for Stepwise:

- The DNS of `sylent.tools` is held by Cloudflare. The owner adds a `CNAME` record there: `stepwise` → `aunimhsn.github.io`, **DNS only** (the grey cloud): proxied by Cloudflare, it would hide the domain from GitHub, which could then not issue its HTTPS certificate.
- Once the record answers, the custom domain is set in **Settings › Pages** of the repository, then **Enforce HTTPS** once GitHub has issued the certificate. Set before the record exists, the default address would redirect to a domain that doesn't answer.
- The owner verifies `sylent.tools` in the GitHub account (**Settings › Pages › Add a domain**, a DNS `TXT` record), so that no one else can claim a subdomain pointing to GitHub without a repository behind it.
- Until then, and as a fallback, the website is served at its default address, `https://aunimhsn.github.io/rust-ts-stepwise/`.
- The website never hard-codes its path: the workflow `.github/workflows/deploy.yml` reads it from the settings of Pages (`actions/configure-pages`) and gives it to SvelteKit as `paths.base` (the variable `BASE_PATH` of `web/vite.config.ts`); it is empty at the root of a domain and `/rust-ts-stepwise` at the default address. The domain can change later without changing the code.

## Publishing

- Every push to `main` (each merged pull request) runs `.github/workflows/deploy.yml`: it builds `web/` with the address above and publishes `web/build` to Pages. It can also be run by hand (**Actions › Deploy › Run workflow**).
- Only `main` can deploy: the environment `github-pages`, created when Pages was set to **GitHub Actions** on 2026-11-10, allows that branch alone.
- `main` is protected by the ruleset `main` (created on 2026-11-10): a pull request is required (no approval, the owner works alone; squash merging only), with the check `CI`, the last job of `.github/workflows/ci.yml`; no force push, no deletion; no one bypasses it, not even an administrator. The branch doesn't have to be up to date with `main` before merging, to avoid needless rebases.

## Risks accepted by the owner

On 2026-10-31:

- A class or a school visiting at once could, in theory, reach the soft bandwidth limit: unlikely at the sizes above. If it happens, the same static files can move to another host (Cloudflare Pages, Netlify) without changing the code.
- On 2026-11-10: if Stepwise grows and gets a domain of its own, the old address will redirect to it; the links follow, but not the progress, which the browser keeps per address: the learners would start again, unless the export of the progress of the [backlog](backlog.md) exists by then.

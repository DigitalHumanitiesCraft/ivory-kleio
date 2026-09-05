---
title: Build this website
description: Run and verify the Ivory KLEIO documentation site from its repository.
---

The website uses Astro and Starlight. Its maintained scientific documents are imported from the repository during preparation. Tutorials and introductory pages are authored within the site.

Production build completion and publication are still pending. The repository [work plan](https://github.com/DigitalHumanitiesCraft/ivory-kleio/blob/main/knowledge/plan.md) records the session-close findings.

## Run locally

Use Node.js 24 and npm. From the repository root:

```sh
cd site
npm ci
npm run dev
```

Open the local address printed by Astro, including the `/ivory-kleio/` subpath.

## Verify the build

```sh
npm run check
npm test
npm run build
npm run check:links
npm run preview
```

The production build creates static files in `site/dist/`. The link check validates local destinations and fragment identifiers in that output.

## Edit the correct source

The requirement register, architecture, methodology and research cases remain in their canonical repository files. The preparation script generates the public documentation with source links and remaps cross-references.

Edit those original files, then restart the development command to refresh generated content. Do not edit the generated Markdown directory. Website introductions, components and styles can be edited directly and update during development.

## Publish with GitHub Pages

The repository workflow builds and checks the site before deployment. GitHub Pages must use GitHub Actions as its publishing source. The production path is `/ivory-kleio/`.

Pull requests and updates on `main` run verification. Publication uses a manual workflow run after GitHub Pages is configured and the checks succeed. The documentation site does not run a database service or agent backend.

# DevFlow Pro Blog Posts

This directory contains blog posts that are automatically synced with dev.to through GitHub Actions.

## File Structure
- Each `.md` file in this directory represents a blog post
- Front matter must include required dev.to metadata
- Images should be stored in `/blog/images/`

## Writing Guidelines
1. Use front matter for post metadata:
   ```yaml
   ---
   title: "Your Title"
   published: true
   description: "Your description"
   tags: tag1, tag2, tag3, tag4
   series: "Optional Series Name"
   ---
   ```

2. Follow markdown best practices:
   - Use proper heading hierarchy
   - Include code snippets with language tags
   - Add relevant images with alt text
   - Link to source code when applicable

## Publishing Process
1. Create new `.md` file in this directory
2. Add required front matter
3. Write your content
4. Commit and push to main branch
5. GitHub Action will automatically publish to dev.to

## Configuration
The GitHub Action uses the following environment variables:
- `DEVTO_API_KEY`: Your dev.to API key (stored in GitHub Secrets)

## Getting Started
1. Get your dev.to API key from https://dev.to/settings/account
2. Add it to your repository secrets as `DEVTO_API_KEY`
3. Start writing posts in this directory!

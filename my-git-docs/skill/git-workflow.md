# Git Workflow

## Safety
- Do not overwrite user changes.
- No destructive git commands unless explicitly requested.

## Commit Message Rules (Conventional Commits)
- Commit messages **MUST** follow Conventional Commits.
- When helpful, include a concise summary and meaningful bullet points.
- Scopes should focus on the package boundary; do not drill into deep internal dependency details.

### Language
- Commit summary and body, branch names, tags and pushed PR or release text are written in English
  only, whatever language the request came in; words quoted from a person are translated.
- The standing rules' 「Language」 names the only exception, and the close refuses any other Hangul.

### Multiline Body Formatting
- Multiline bodies are written as `-` bullets per line.
- Do not embed literal `\n` characters.
- No unnecessary blank lines between bullets (except when grouping is truly needed).

### Example

```text
refactor: improve component consistency and UI styling

- Update icon styling in tree components (BoardGroupTree, OrgTree)
- Remove redundant PostComments/PostCommentThread components
- Unify reply/comment rendering through PostReplies component
- Enhance PostForm with content type icons and better UI
- Switch from dompurify to isomorphic-dompurify for SSR compatibility
- Add navigation links to dashboard recent items
- Fix content sanitization in reply threads
- Clean up code formatting and imports
```

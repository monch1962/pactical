# Templates

This directory contains a set of templates that can be used to transform Pact contract specifications into executable test cases, stubs, documentation etc.

There's also value in having many eyes reviewing these templates with a view of continually improving them over time

## Template format

Each of these templates are based on the Handlebars (https://handlebarsjs.com) framework. According to their website, Handlebars is a _general-purpose semantic templating framework_; as such there's nothing special about it in the context of Pact, test cases, stubs, etc. It was chosen because it's both simple to use and extensible, and would be easily swappable for a different templating engine if a better one emerged.

As the URL suggests, Handlebars' original implementation was in Node.JS, but it's now been implemented in a large number of common programming languages and is essentially now a language-agnostic framework.

Handlebars is a derivative of Mustache (http://mustache.github.io/), and has strong backwards compatibility with Mustache templates. We did initially consider using Mustache but found that it has limitations for this specific purpose. The lack of a "helper" capability in particular meant we'd need to build & maintain a lot of bespoke Go code for things such as converting text to lowercase, formatting text as JSON etc. In contrast, Handlebars' helper capability lets us write simple code fragments to do these types of tasks, and do so in a way that are much easier to maintain.

### Custom helpers

We've created a couple of Handlebars helpers for transforming Pact contracts, and they're included in the main.go file. These helpers are:
- _toJSON_ which will format supplied text in JSON format for presenting in a template. You'll generally want to use it within triple-brackets e.g. `{{{toJSON myContent}}}` to ensure that your data doesn't get HTML-escaped and `'` characters wind up being displayed as `&quot;`
- _lowerCase_ which will convert supplied text to lowercase. In particular, we found that `GET`, `PUT`, `POST`, ... in Pact specifications often need to be convered to `get`, `put`, `post`, ... to be used in different testing frameworks, so this helper should make it easy
- _isoTimeRFC3339_ will return an ISO RFC-3339 formatted timestamp for the current time. There's no parameters for this variable, so use it as `{{isoTimeRFC3339}}`
- _envVar_ will return the value of a supplied environment variable. Use something like `{{{envVar "MY_VAR"}}}` to insert the value of the MY_VAR environment variable

It's _highly_ likely that creating templates for different languages/frameworks will expose a need for more custom helpers over time. Please feel free to create new helpers and submit them via pull requests for inclusion in the core code repository

## Future intentions

The intention is that this set of templates will be expanded over time to cover a wide variety of languages, toolsets and frameworks. 

While _I_ happen to think Karate is a pretty nice, general-purpose API test framework, there's no reason for your team to use Karate for Pact testing if you're already using another set of tools and you're comfortable with them - just create a new template! For example, if you're working in a team that uses Haskell, you could create templates to tests in Haskell's Tasty framework that your team would be comfortable using.

If you create some new templates that you think others would like to use, don't sit on them - create a pull request to have them included in this repo so everyone can start using them

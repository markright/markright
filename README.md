![Markright](https://rawgit.com/markright/markright/master/artwork/Markright-mark.svg)

# Markright

A new plain text formatting syntax

## Why a new syntax?

We all know **Markdown**: it's great and used everywhere (even this document is written with it).

But it has some flaws:

* Its specs weren't unambiguous by the beginning.
* Many typographical HTML5 tags are not supported by default.
* Some choices make unintuitive to add support for missing tags.

These are exactly the issues that **Markright** aims to solve!

## Current status

**Markright** is in an early stage of specs definition.

Any suggestion or contribution is **welcome**!!

## Basics

This is a brief overview of what it’s like to use **Markright** and how it differs from **Markdown**.

### Paragraphs, headers, blockquotes

```
# A First Level Header

## A Second Level Header

This is just a regular paragraph.
This is a second line.

The quick brown fox jumped over the lazy dog's back.

### Header 3

> This is a blockquote.
> 
> This is the second paragraph in the blockquote.
>
> -- The Author
```

Output:

```
<h1>A First Level Header</h1>

<h2>A Second Level Header</h2>

<p>This is just a regular paragraph.<br/>
This is a second line.</p>

<p>The quick brown fox jumped over the lazy dog's back.</p>

<h3>Header 3</h3>

<blockquote>
  <p>This is a blockquote.</p>
  
  <p>This is the second paragraph in the blockquote.</p>
  
  <cite>The Author</cite>
</blockquote>
```

### Typography

```
Some of these words *are emphasized*.

Use two asterisks for **strong emphasis**.

Use three asterisks for ***very strong emphasis***.

Use two underscores for __underline words__.

Use two tildes for ~~strikethrough~~.

If x^y to elevate a word, or x#y to push it down.
```

Output:

```
<p>Some of these words <em>are emphasized</em>.</p>

<p>Use two asterisks for <strong>strong emphasis</strong>.</p>

<p>Use three asterisks for <strong><em>very strong emphasis</em></strong>.</p>

<p>Use two underscores for <u>underline words</u>.</p>

<p>Use two tildes for <del>strikethrough</del>.</p>

<p>Use x<sup>y</sup> to elevate a word, or x<sub>y</sub> to push it down.
```

## License

Copyright (c) 2017 Emanuele Bertoldi

[MIT License](http://en.wikipedia.org/wiki/MIT_License)


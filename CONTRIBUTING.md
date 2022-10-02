# Contribution Guideline

This document contains the guideline for contributing into this project. Contributors are encouraged to follow the guidelines. Failure to do so may
result in closed PR.

# Making commits

Make sure your commit message has one of the following formats:
- Frontend|Backend|Bot: Message
- Frontend|Backedn|Bot(Here, write about the thing you worked on): Message

Add optional extended description about what you did in new commit if necessary.

# Frontend

## Naming convention

- Use pascal case when naming interfaces, classes, functions, variables, and components.
- Use screaming snake case when naming constants
- Use snake case when naming properties(interfaces, and classes)

Make sure to use names that are meaningful.

## Code style

- Tab: 2 spaces
- Spaces around operators(`+`, `-`, `&`, etc.)
- Don't write super long lines(eg. a line that contains 100+ characters)

## Before commit

- Run `yarn format` to reformat newly written code.

# Backend and Discord Bot

## Naming convention

Follow standard Rust naming convention and make sure names are meaningful.

## Code style

- Tab: 2 spaces
- Spaces around operators(`+`, `-`, `&`, etc.)
- Don't write super long lines(eg. a line that contains 100+ characters)

## Before commit

Run `cargo fmt` to reformat newly written code.
